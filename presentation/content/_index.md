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

# Previous Work

---

#### Voxel-Based Approach

![The point cloud of an airplane is voxelized to a 30×30×30 volumetric occupancy grid](images/VBA.png)

<div class="text-sm">
3D CNNs applied to voxelized data have shown good performance, but they suffer from high memory consumption due to the sparsity.
</div>

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

{{% /section %}}

--- 

# Deep Learning Directly with Point Clouds

---

## PointNet
PointNet is a **deep learning** approach for **scene segmentation** and **object classification** of 3D structures.

{{< figure src="images/tasks.png" title="PointNet can find all of the unique objects in the scan and identify them!" >}}

---

![PointNet arch](images/architecture.png)

---

{{% section %}}

## How does PointNet overcome the challenges of Unordered input and Transformation Invariance?

---

## Unordered Input

Approximate a general function defined on a point set by applying a symmetric function on transformed set elements
$ f(\{x_1, \ldots, x_n\}) \approx g(h(x_1), \ldots, h(x_n)) $ where $ f : 2^{R^N} \rightarrow R, h : R^N -> R^K $
and $ g : R^K_0 \times \cdots \times R^K_n \rightarrow R $ is a symmetric function.

- $ h $ is approximated by the MLP network
- $ g $ by a composition of a simple single variable function and a max pooling function

---

### Labeling an object should be invariant to rigid transformations of that object

{{% note %}}
Imagine you are a 3D segmentation algorithm and you're asked to segment a 3D scan of this room.
What are the contents of this room : (desks, chairs, people, ...)
Now imagine we arbitrarily rotated the desks.  You would still be able to identify the desks.
So can PointNet
{{% /note %}}

![Use an affine matrix for transformation invariance](images/spat-trans.gif)

---

Pointnet predicts an affine transformation matrix using a sub-network
### `T-Net`
and applies the predicted transformation to the input points.

![T-Net flow](images/tnet.png)

---

### The idea is extended to feature space as well

however, due to high dimensionality $(64\times64)$ a regularization term is added to the 
Softmax loss function

$$ L_{reg} = ||I - AA^T||_F^2 $$

to approximate an orthogonal transformation
{{% /section %}}

---

<div style="font-size: 25px;">
The classification network extracts embeddings from Point featurees
</div>

![PointNet arch](images/architecture.png)

<div style="font-size: 25px;">
In semantic segmentation the pointwise and global feature matrices are concatenated
and information is then extracted from this combined set.
</div>

---

{{% section %}}

# Evaluating PointNet

<div class="text-left" data-markdown>
- Object Classification
- Object Part Segmentation
- Semantic Scene Segmentation
</div>

---

### 3D Object Classification

- Evaluated on ModelNet40
- 12311 CAD models from 40 object categories
  - 80/20 train-test split
  - 89% Accuracy on Test set

---

### 3D Object Part Segmentation

<div class="text-sm" data-markdown>
- Evaluated on ShapeNet
- 16,881 shapes from 16 categories annotated with 50 parts
  - Most objects have 2-5 parts
  - 83.7 mIoU(%)
    - 2.3% mIoU improvement over compared methods
</div>

<div class="container" data-markdown>
<div class="col">
![Network architecture for part segmentation.](images/Network-Arch.png)
</div>
<div class="col">
![Qualitative results for part segmentation.](images/Partial.png)
</div>
</div>

---

### Scene Segmentation

<div class="text-sm" data-markdown>
- Evaluated on Stanford 3D Semantic Parsing dataset
  - 3D scans from Matterport scanners in 271 rooms
  - 47.71 mIoU(%)
  - 78.62% Accuracy
  - *A major improvement over baseline*
    - 20.12 mIoU / 53.19% Accuracy
</div>

![Qualitative results for semantic segmentation](images/Semantic-Segmentation.png)

{{% /section %}}

---

### Robustness Test

<div class="text-sm" data-markdown>
- PointNet is robust to various kinds of input corruptions.
- As to missing points, when there are 50% points missing,the accuracy only drops by 2.4% and 3.8% w.r.t. furthest and random input sampling.
- As to the outliers, the net has more than 80% accuracy even when 20% of the points are outliers.
</div>

![PointNet robustness test](images/Robust.png)

---

### Visualizing PointNet

<div style="font-size: 25px;">
While critical points jointly determine the global shape feature for a given shape, any point cloud that falls between the critical points set and the upper bound shape gives exactly the same feature.
</div>

![Critical points and upper bound shape](images/Critical-point.png)
 
---

### Conclusion:
- This paper proposes a novel deep neural network PointNet that directly consumes point cloud. 
- This network provides a unified approach to a number of 3D recognition tasks including object classification, part segmentation, and semantic segmentation.  
- Obtaining better results than state of the arts on standard benchmarks.
- Providing theoretical analysis and visualizations towards understanding of the network.

---
# Questions?
### 🙋🏿‍♂️ 🙋🏻 🧟‍♀️ 🙋🏽‍♀️
