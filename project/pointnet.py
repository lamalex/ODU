#!/usr/bin/env python3
'''
Pytorch implementation of Pointnet

Usage:
    pointnet.py [--one] [--epochs=<n>] [--path=<path>]

--one           Perform a single training iteration
--epochs=<n>    Number of epochs to train model [default: 10]
--path=<dir>    Data input directory
'''

import os
import sys
import random
import logging
import numpy as np
from enum import Enum
from pathlib import Path
from docopt import docopt
from itertools import islice
from functools import partial  # , reduce
from nptyping import NDArray
from typing import Tuple, List, Dict, Optional, Callable

import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.utils.data import Dataset, DataLoader
from torchvision import transforms

logger = logging.getLogger('pointnet')
logging.basicConfig(level=logging.DEBUG,
                    format='%(asctime)s %(levelname)-8s %(message)s',
                    datefmt='%m-%d %H:%M')


def safe_cast(f: Callable, val: any, more: Optional[str] = None) -> any:
    '''
    '''
    try:
        return f(val)
    except ValueError:
        msg = f'ValueError casting \'{val}\' to {f} {str() if more is None else more}'
        logger.warning(msg)
        return f()


# Would have liked to use a DataClass here to define
# a simple Mesh type with vertices/faces accessors, but
# LO: Python 3.6.9 vs Python 3.7
# A Tuple of lists will work, but I don't love it.
def read_off(file: str) -> Tuple[List[int], List[float]]:
    '''
    https://en.wikipedia.org/wiki/OFF_(file_format)
    https://segeval.cs.princeton.edu/public/off_format.html
    Simple geometry definition file containing lists
    of vertices, faces, and edges

    ModelNet40 has a number of malformatted files
    so some additional checks have been added to read
    the header in format

    OFFxxxxx xxx xxx
      instead of
    OFF xxxxx xxx xxx
      or
    OFF
    xxxxx xxx xxx

    All of these should readable with this function

    Args:
        file: File system path to be read

    Returns:
        Tuple of (vertices, faces)
    '''
    cast_w_msg = partial(lambda msg, f, v: safe_cast(f, v, msg), f'in {file}')

    firstline = file.readline().strip()

    if not firstline.startswith('OFF'):
        logger.error(f'{file} lacks a valid OFF header')
        raise ValueError

    num_line = firstline[3:].strip() if firstline != 'OFF' else file.readline()

    n_verts, n_faces, _ = tuple(
        [cast_w_msg(int, s) for s in num_line.split(' ')]
    )
    vertices = [
        [cast_w_msg(float, s) for s in file.readline().strip().split(' ')]
        for _ in range(n_verts)
    ]
    faces = [
        [cast_w_msg(int, s) for s in file.readline().strip().split(' ')][1:]
        for _ in range(n_faces)
    ]

    return vertices, faces


def _triangle_area(a: NDArray[np.float64], b: NDArray[np.float64],
                   c: NDArray[np.float64]) -> np.float64:
    '''
    Calculate area of a triangle by its vertices
    https://en.wikipedia.org/wiki/Heron%27s_formula

    Args:
        a: First vertiex defining triangle
        b: Second vertex defining triangle
        c: Third vertex defining triangle

    Returns:
        Area of triangle defined by a, b, c
    '''
    side_a = np.linalg.norm(a - b)
    side_b = np.linalg.norm(b - c)
    side_c = np.linalg.norm(c - a)
    s = 0.5 * (side_a + side_b + side_c)
    return max(s * (s - side_a) * (s - side_b) * (s - side_c), 0)**0.5


def _sample_point(a: NDArray[np.float64],
                  b: NDArray[np.float64],
                  c: NDArray[np.float64]) -> Tuple[np.float64,
                                                   np.float64,
                                                   np.float64]:
    '''
    Sample points on surface of a triangle defined by 3 vertices

    Args:
        a: First vertex defining triangle
        b: Second vertex defining triangle
        c: Third vertex defining triangle

    Returns:
        Point defined by (x,y,z) located on surface of triangle defined
        by input (a, b, c)
    '''
    # barycentric coordinates on a triangle
    # https://mathworld.wolfram.com/BarycentricCoordinates.html
    s, t = sorted([random.random(), random.random()])
    def f(i): return s * a[i] + (t-s) * b[i] + (1-t) * c[i]
    return (f(0), f(1), f(2))


class Pointcloudify:
    '''
    Process a mesh into a point cloud
    '''

    def __init__(self, samples_per_face: int = 1024):
        '''
        Args:
            samples_per_face: number of points per face to sample
            Default 1024 is from Pointnet paper
        '''
        self.samples_per_face = samples_per_face

    def __call__(self, mesh: Tuple[List[int], List[float]]) -> NDArray:
        '''
        Sample
        '''
        verts, faces = mesh
        verts = np.array(verts)

        # calculate areas of all faces in our mesh
        areas = (_triangle_area(verts[face[0]],
                                verts[face[1]],
                                verts[face[2]]) for face in faces)

        sampled_faces = (random.choices(faces,
                                        weights=areas,
                                        k=self.samples_per_face))

        return np.array([_sample_point(verts[sampled_face[0]],
                                       verts[sampled_face[1]],
                                       verts[sampled_face[2]]) for sampled_face
                         in sampled_faces])


class DataSplitType(Enum):
    '''
    ModelNet has training and test data split into different folders
    Rather than using magic strings use an enum
    '''
    TRAIN = 'train'
    TEST = 'test'


def _list_classes_from_path(root_dir: Path) -> Dict[str, int]:
    '''
    Args:
        root_dir: Path to 3D model data with substructure of form
            root_dir/(class)/(data split)/class_xxxx.off

    Returns:
        Dictionary sorted alphabetically with key: classname
        and value class # based on alphanumeric ordering
    '''
    return {klass: i for i, klass in
            enumerate([klass for klass in sorted(os.listdir(root_dir))
                       if os.path.isdir(root_dir/klass)])}


# Python 3.8 has TypedDict which WOULD BE NICE
def _get_files_for_class(root_dir: Path,
                         class_map: Tuple[str, int],
                         split_type: 'DataSplitType') -> List[Dict[str, any]]:
    '''
    Args:
        root_dir: Path to 3D model data with substructure of form
            root_dir/(class)/(data split)/class_xxxx.off
        class_map: (Class name, id) for which t
        split_type: test/train

    Returns:
        Mapping of .off file path and object class for all files of class
        in split_type
    '''
    class_name, class_id = class_map
    return [{
        'path': path/class_name/split_type.value/fname,
        'class': class_id
    } for fname in os.listdir(root_dir/class_name/split_type.value)
        if fname.endswith('.off')]


def window(seq, n=2):
    '''
    Returns a sliding window (of width n) over data from the iterable
    s -> (s0,s1,...s[n-1]), (s1,s2,...,sn), ...
    '''
    it = iter(seq)
    result = tuple(islice(it, n))
    if len(result) == n:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result


def _identity(cuda: bool, bs: int, n: int) -> torch.Tensor:
    '''
    '''
    eye = torch.eye(n, requires_grad=True).repeat(bs, 1, 1)
    return eye.cuda() if cuda else eye


class UnitNormalize:
    '''
    '''

    def __call__(self, pointcloud):
        return pointcloud / np.max(np.linalg.norm(pointcloud - np.mean(pointcloud, axis=0), axis=1))


class Tensorfy:
    '''
    Torch has its own ToTensor class in transforms module
    but torch.transforms.ToTensor creates a 3D tensor instead of 2
    '''

    def __call__(self, pointcloud):
        return torch.from_numpy(pointcloud)


def base_transform_pipeline() -> transforms.Compose:
    '''
    Returns:
        Transformation pipeline
    '''
    return transforms.Compose([
        Pointcloudify(),
        UnitNormalize(),
        Tensorfy()])


class PointCloudDataSet(Dataset):
    '''
    Point cloud data loading helper
    https://pytorch.org/tutorials/beginner/data_loading_tutorial.html
    '''

    def __init__(self,
                 root_dir: str,
                 split_type: 'DataSplitType',
                 transform: Optional[Callable] = base_transform_pipeline()):
        '''
        Args:
            root_dir: Path to dataset
            split_type: Which split to load
            transform: Optional transformation to be applied to sample
        '''
        self.classes_map = _list_classes_from_path(root_dir)
        self.datafiles = [datafile for class_map in self.classes_map.items()
                          for datafile in _get_files_for_class(
            path,
            class_map,
            split_type)]
        self.transform_pipeline = transform

    def _process(self, file: str) -> torch.Tensor:
        '''
        Load .off file and perform any preprocessing

        Args:
            file: path to .off file to load and process

        Returns:
            post-transformation tensor from .off file
        '''
        mesh = read_off(file)

        try:
            if self.transform_pipeline is None:
                return base_transform_pipeline()(mesh)
            return self.transform_pipeline(mesh)
        except IndexError as err:
            logger.error(
                f'Encountered an error during transform pipeline in {file}: {err}')
            # Will be interesting to see what this get's classified as ...
            # Probably we actually want to bail on this model entirely but
            # I'll wait and see if this actually happens in practice
            return torch.rand((1024, 3))

    def __len__(self):
        return len(self.datafiles)

    def __getitem__(self, idx):
        with open(self.datafiles[idx]['path'], 'r') as f:
            pointcloud = self._process(f)

        return {'pointcloud': pointcloud, 'class': self.datafiles[idx]['class']}


class TNet(nn.Module):
    '''
    Regression network for predicting a k x k transformation matrix.
    A sequence of batch normalized CNNs, MLPs, and a max pooling layer.
    '''

    def __init__(self, k: int):
        '''
        Args:
            k: Input layer dimensionality
        '''
        super().__init__()
        dims = [k, 64, 128, 1024, 512, 256, k**2]
        cnn_dims = window(dims[:4])
        fc_dims = window(dims[3:])

        self.k = k
        self.cnn = nn.ModuleList([nn.Conv1d(i, o, 1) for (i, o) in cnn_dims])
        self.fc = nn.ModuleList([nn.Linear(i, o) for (i, o) in fc_dims])
        self.bn = nn.ModuleList([nn.BatchNorm1d(d) for d in dims[1:-1]])

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        '''
        Predict (bs, k, k) affine transformation matrix
        for projecting model into normalized space.

        Args:
            x: A batch of n examples with shape (batch size, n, k)

        Returns:
            Tensor of (bs, k, k) affine transformation matrix
        '''
        batchsize = x.size()[0]
        bn_iter = iter(self.bn)

        # Extremely obtuse one-liner for sequential application of
        # ReLU(BatchNorm(CNN)) for each CNN layer, but I finally found a use
        # for the walrus operator so I'm keeping it.
        # EXCEPT OMFG GOOGLE COLAB IS PYTHON 3.6 SO I CAN'T USE := 🤬
        # x = reduce(lambda x,
        #             f: (cnn := f[0],
        #                 bn := f[1],
        #                 F.relu(bn(cnn(x)))
        #                )[-1],
        #             zip(self.cnn, bn_iter), x)

        # Same as above in a much more readable and Python 3.6 friendly version
        for i in range(len(self.cnn)):
            bn = bn_iter.__next__()
            x = F.relu(bn(self.cnn[i](x)))

        x = torch.max(x, 2, keepdim=True)[0]
        x = x.view(-1, 1024)

        # IT STINGS EVERY TIME
        # x = reduce(lambda x,
        #            f: (fc := f[0],
        #                bn := f[1],
        #                F.relu(bn(fc(x)))
        #               )[-1],
        #            zip(self.fc, bn_iter), x)
        for i in range(len(self.fc[:-1])):
            bn = bn_iter.__next__()
            x = F.relu(bn(self.fc[i](x)))

        x = self.fc[-1](x)
        x = x.view(-1, self.k, self.k) + \
            _identity(x.is_cuda, batchsize, self.k)

        return x


def _tensor_mul(x1: torch.Tensor, x2: torch.Tensor) -> torch.Tensor:
    return torch.bmm(torch.transpose(
        x1, 1, 2),
        x2).transpose(1, 2)


def _ident_diff(x: torch.Tensor) -> torch.Tensor:
    '''
    '''
    eye = _identity(x.is_cuda, x.size(0), x.size(1))
    return eye - torch.bmm(x, x.transpose(1, 2))


class Pointnet:
    @staticmethod
    def loss(outputs, labels, points_xformed, features_xformed, alpha=0.001):
        criterion = torch.nn.NLLLoss()
        batchsize = outputs.size(0)

        points_diff = _ident_diff(points_xformed)
        features_diff = _ident_diff(features_xformed)

        return criterion(outputs, labels) + alpha * (torch.norm(points_diff) + torch.norm(features_diff)) / float(batchsize)


class PointnetTransformPipeline(nn.Module):
    def __init__(self):
        super().__init__()
        self.pointwise_transform = TNet(k=3)
        self.featurewise_transform = TNet(k=64)

        dims = [3, 64, 128, 1024]
        self.cnn = nn.ModuleList([nn.Conv1d(i, o, 1)
                                  for (i, o) in window(dims)])
        self.bn = nn.ModuleList([nn.BatchNorm1d(dim) for dim in dims[1:]])

    def forward(self, x: torch.Tensor) -> Tuple[torch.Tensor, torch.Tensor, torch.Tensor]:
        x_point_transformed = self.pointwise_transform(x)
        x = _tensor_mul(x, x_point_transformed)
        x = F.relu(self.bn[0](self.cnn[0](x)))

        x_feature_transformed = self.featurewise_transform(x)
        x = _tensor_mul(x, x_feature_transformed)
        x = F.relu(self.bn[1](self.cnn[1](x)))
        x = self.bn[2](self.cnn[2](x))
        x = nn.MaxPool1d(x.size(-1))(x)
        x = nn.Flatten(1)(x)

        return x, x_point_transformed, x_feature_transformed


class PointnetClassificationNet(nn.Module):
    def __init__(self, classes: int = 10, dropout: float = 0.3):
        '''
        Args:
            classes: [default: 10] from ModelNet10
            dropout: [default: 0.3] from PointNet paper 'keep ratio 0.7'
        '''
        super().__init__()
        self.transformer = PointnetTransformPipeline()
        dims = [1024, 512, 256, classes]
        self.fc = nn.ModuleList([nn.Linear(i, o) for (i, o) in window(dims)])
        self.bn = nn.ModuleList([nn.BatchNorm1d(dim) for dim in dims[1:-1]])
        self.dropout = nn.Dropout(p=dropout)

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        x, x_point_transformed, x_feature_transformed = self.transformer(x)
        x = F.relu(self.bn[0](self.fc[0](x)))
        x = F.relu(self.bn[1](self.dropout(self.fc[1](x))))
        output = self.fc[2](x)

        return (
            F.log_softmax(output, dim=1),
            x_point_transformed,
            x_feature_transformed
        )


if __name__ == '__main__':
    args = docopt(__doc__)

    path = args['--path']

    path = Path(path)
    training_data = PointCloudDataSet(
        root_dir=path,
        split_type=DataSplitType.TRAIN)

    test_data = PointCloudDataSet(
        root_dir=path,
        split_type=DataSplitType.TEST)

    # Defined in Pointnet paper
    BATCH_SIZE = 32
    training_dataloader = DataLoader(
        dataset=training_data,
        batch_size=BATCH_SIZE,
        shuffle=True)

    test_dataloader = DataLoader(
        dataset=test_data,
        batch_size=BATCH_SIZE)
    device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')

    num_classes = len(training_data.classes_map)
    pn = PointnetClassificationNet(classes=num_classes)
    if torch.cuda.device_count() > 1:
        logger.info(f'Running on {torch.cuda.device_count()} GPUs')
        pn = nn.DataParallel(pn)
    pn.to(device)

    optimizer = torch.optim.Adam(pn.parameters())

    if args['--one']:
        logger.warning("Executing single forward pass then exiting")
        data = iter(training_dataloader).__next__()
        inputs = data['pointcloud'].to(device).float().transpose(1, 2)
        labels = data['class'].to(device)

        pn(inputs)
        sys.exit(0)

    # TODO:
    #  * Turn this into a function so we can use it to train segmentation
    #  * Reduce learning rate after 20 epochs (from paper)
    #    - https://pytorch.org/docs/stable/optim.html#how-to-adjust-learning-rate
    # Training loop from https://pytorch.org/tutorials/beginner/blitz/cifar10_tutorial.html
    epochs = int(args['--epochs'])
    logger.info(f'[ {"Beginning Pointnet training loop":^47} ]')
    for epoch in range(1, epochs + 1):
        pn.train()
        running_loss = 0.0
        for i, data in enumerate(training_dataloader, 1):
            inputs = data['pointcloud'].to(device).float().transpose(1, 2)
            labels = data['class'].to(device)

            optimizer.zero_grad()

            outputs, points_xformed, features_xformed = pn(inputs)
            loss = Pointnet.loss(
                outputs, labels, points_xformed, features_xformed)
            loss.backward()
            optimizer.step()

            # Print epoch stats
            running_loss += loss.item()
            if i % 10 == 9:
                logger.info(
                    f'[ Epoch: {epoch:>3} | Batch: {i:>5}/{len(training_dataloader):<5} | Loss: {running_loss / 10:.5f} ]')
                running_loss = 0.0

        # Evaluate accuracy after each epoch
        pn.eval()
        total = 0
        correct = 0

        with torch.no_grad():
            for data in test_dataloader:
                inputs = data['pointcloud'].to(device).float().transpose(1, 2)
                labels = data['class'].to(device)
                outputs, *_ = pn(inputs)
                _, predicted = torch.max(outputs.data, 1)
                total += labels.size(0)
                correct += (predicted == labels).sum().item()
            acc_str = f'Accuracy after epoch {epoch}: {100.0 * correct / total:.3f}'
            logger.info(f'[ {acc_str:^47} ]')

        torch.save(pn.state_dict(), f'pointnet-{epoch}.pt')
    logger.info(f'[ {"Finished training Pointnet":^47} ]')
