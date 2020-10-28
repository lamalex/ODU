+++
title = "PointNet: Segmentation and Classification of point-clouds with Deep Learning"
outputs = ["Reveal"]

[reveal_hugo]
slide_number = true
theme = "moon"
hide_cursor_time = 500
+++

<style>
.container{
    display: flex;
}
.space-evenly {
  justify-content: space-between;
}
.col{
    flex: 1;
}
.text-left {
  text-align: left;
}
.text-sm {
  font-size: 20px;
}
.text-xs {
  font-size: 10px
}
</style>

# PointNet
Segmentation and Classification of point-clouds with Deep Learning

```php
Alex Launi & Tareq Alghamdi
ODU CS722
Fall 🎃 2020
```

---

## What is a point cloud?
<div class="container" data-markdown>
  <div class="col">
![Torus gif from WikiPedia](images/Point_cloud_torus.gif)
  </div>
  <div class="col">
> A set of 3D points $$ \{ P_{i} | i = 1,\ldots,n \} $$,
> where each point $P_i$ is a vector of its  $(x,y,z)$ coordinates 
> plus extra feature channels such as color, normal, etc.
  </div>
</div>

---

## Point Clouds

<div class="container text-left" data-markdown>
  <div class="col">
#### are generated from
 - MS Kinect 
 - LiDAR Scans
 - 3D modelling/CAD software
  </div>

  <div class="col">
- are unordered
- exhibit local structure
- are invariant to (certain) transformations
  </div>
</div>


{{% note %}}
1) Points in 3d space have no defined ordering. This makes some approaches difficult, for instance they cannot be sorted
2) Adjacency of points matters. Points cannot be arbitrarily permuted. While they are *not ordered*; order matters.
3) Sets of points can be skewed, rotated, scaled, and should still be identifiable as their object class
{{% /note %}}

---

## Challenges of Deep Learning with Point Clouds:

<div class="container space-evenly" data-markdown>
  <div class="col">
- Irregular
  </div>
  <div class="col">
- Unstructured
  </div>
  <div class="col">
- Unordered
  </div>
</div>

![Challenges of point cloud data](images/MLP.png)

---

{{% section %}}

## Structured Grid-Based Learning

<div class="container" data-markdown>
  <div class="col">
![A typical 2D convolution operation](images/CNN.png)
  </div>
  <div class="col">
- The convolution operation requires a structured grid. 
- Point cloud data are unstructured, and this is a challenge for deep learning. 
  </div>
</div>

#### To overcome this challenge, many approaches convert the point cloud data into a structured form. 

---

#### Voxel-Based Approach

![The point cloud of an airplane is voxelized to a 30×30×30 volumetric occupancy grid](images/VBA.png)

#### Although voxel-based methods have shown good performance, they suffer from high memory consumption due to the sparsity of the voxels

--- 

## Multi-View-Based Approach

![Ok cool](images/MVB.jpg)
<div class="container text-sm space-evenly" data-markdown>
<div class="col">
- Images are actual representations of the 3D world projected onto a 2D grid
- Each 2D image represents the same object viewed from a different angle
</div>
<div class="col">
- Can then apply existing 2D CNN techniques to the projections
- Multi-view based networks have better performance than voxel-based methods. 
</div>
</div>

--- 

## Higher-Dimensional Lattices
- There are other methods for point cloud processing using deep learning that convert the point clouds into a higher-dimensional regular lattice.
- SplatNet
- SFCNN
- Compared to voxel-based and multi-view approaches, higher-dimensional approaches have better performance in terms of segmentation with SplatNet. 
- They are also better than the voxel-based approach in terms of classification.

{{% /section %}}

--- 

# Deep Learning Directly with Point Clouds

---

## PointNet
PointNet is a **deep learning** approach for **scene segmentation** and **object classification** of 3D structures.

{{< figure src="images/tasks.png" title="PointNet can find all of the unique objects in the scan, and identify them!" >}}

---

PointNet is a novel approach to 3D-structure classification. Many prior approaches transformed data into structured formats like
 - 3D Voxels
 - Image grids

but PointNet operates directly on **sets of points** using a *convolution network* and *max pooling function*.

> PointNet learns to summarize a point cloud by a sparse set of key points, which is approximately an object's skeleton 💀

---

## PointNet Architecture
![PointNet architecture diagram](images/architecture.png)
- 2 Joint Alignment Networks
- Local & Global information combination
- Max pooling aggregation layer

---

{{% section %}}

# Joint Alignment Network

---

### Labeling an object should be invariant to rigid transformations of that object.

{{% note %}}
Imagine you are a 3D segmentation algorithm and you're asked to segment a 3D scan of this room.
What are the contents of this room : (desks, chairs, people, ...)
Now imagine we arbitrarily rotated the desks.  You would still be able to identify the desks.
So can PointNet
{{% /note %}}

---

Pointnet solves this by attempting to align all input set to a canonical space prior to feature extraction.

{{% note %}}
There are a few techniques that have been applied to solve this problem.
1) Sort points into a canonical order
  - as mentioned prior sorting in high dimensions is undefined
2) Use a RNN with augmented training data
{{% /note %}}

---

## Apply a symmetric aggregation function

{{% note %}}
3) Apply a symmetric function to aggregate the information from each point
{{% /note %}}


Pointnet predicts an affine transformation matrix using a sub-network, 
### `T-Net`,
and applies the predicted transformation to the input points.

---


## T-Net

![T-Net flow](images/tnet.png)

![Use an affine matrix for transformation invariance](images/spat-trans.gif)

---

### The idea is extended to feature space as well

however, due to high dimensionality $(64\times64)$ a regularization term is added to the 
Softmax loss function

$$ L_{reg} = ||I - AA^T||_F^2 $$

to approximate an orthogonal transformation

{{% /section %}}

---

{{% section %}}

# Local ↔︎ Global Information Combination

---

For semantic segmentation the pointwise and global feature matrices are concatenated
and information is then extracted from this combined set.

{{% /section %}}

---

{{% section %}}

# Max pooling aggregation layer

---

Make the model invariant to input permutation (except that order matters and cannot be totally ignored!)

Approximate a general function defined on a point set by applying a symmetric function on transformed set elements
$ f(\{x_1, \ldots, x_n\}) \approx g(h(x_1), \ldots, h(x_n)) $ where $ f : 2^{R^N} \rightarrow R, h : R^N -> R^K $
and $ g : R^K_0 \times \cdots \times R^K_n \rightarrow R $ is a symmetric function.

- $ h $ is approximated by the MLP network
- $ g $ by a composition of a simple single variable function and a max pooling function

{{% /section %}}

---

# Questions?
### 🙋🏿‍♂️ 🙋🏻 🧟‍♀️ 🙋🏽‍♀️
