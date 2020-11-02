from itertools import islice
from functools import reduce

import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.autograd import Variable
import numpy as np

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

class TNet(nn.module):
    def __init__(self, k: int):
        super(TNet, self).__init__()
        dims = [k, 64, 128, 1024, 512, 256, k**2]
        cnn_dims = window(dims[:4])
        fc_dims = window(dims[3:])
        
        self.k = k
        self.cnn = [nn.Conv1d(i, o, 1) for (i, o) in cnn_dims]
        self.fc = [nn.Linear(i, o) for (i, o) in fc_dims]
        self.bn = [nn.BatchNorm1d(d) for d in dims[1:-1]]

    def forward(self, x):
        batchsize = x.size()[0]
        bn_iter = iter(self.bn)

        # Extremely obtuse one-liner for sequential application of ReLU(BatchNorm(CNN)) for each CNN layer
        # But I finally found a use for the walrus operator so I'm keeping it.
        x = reduce(lambda x, f: (cnn := f[0], bn := f[1], F.relu(bn(cnn(x))))[-1], zip(self.cnn, bn_iter), x)
        x = torch.max(x, 2, keepdim = True)[0]
        x = x.view(-1, 1024)

        x = reduce(lambda x, f: (fc := f[0], bn := f[1], F.relu(bn(fc(x))))[-1], zip(self.fc, bn_iter), x)
        x = self.fc[-1](x)

        ident = Variable(torch.from_numpy(np.identity(self.k))).view(1, self.k**2).repeat(batchsize, 1)
        x += ident
        x.view(-1, self.k, self.k)

        return x