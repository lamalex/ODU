+++
title = "PointNet: Segmentation and Classification of point-clouds with Deep Learning"
outputs = ["Reveal"]
katex = true
markup = "mmark"

[reveal_hugo]
slide_number = true
theme = "moon"
#custom_theme = "theme-overrides.scss"
#custom_theme_compile = true
+++
# PointNet
Segmentation and Classification of point-clouds with Deep Learning

> Alex Launi & Tareq Alghamdi
> ODU CS722 Fall 2020

---
## What is a point cloud?
![bg left:45% Torus gif from WikiPedia](images/Point_cloud_torus.gif)
> A set of 3D points
> $$ \{ P_{i} | i = 1,\ldots,n \} $$, where each point $P_i$ is a vector of its  $(x,y,z)$ coordinates plus extra feature channels such as color, normal, etc.

---
## Point Clouds
<style>
.container{
    display: flex;
}
.col{
    flex: 1;
}
</style>

<div class="container" data-markdown>
<div class="col">
#### are generated from
 - MS Kinect 
 - LiDAR Scans
 - 3D modelling/CAD software
</div>

<div class="col">
- are unordered
- exhibit local structure
  - adjacency has meaning
- are invariant to (certain) transformations
</div>
</div>
---
## Challenges of Deep Learning with Point Clouds:
-  Irregularity
-  Unstructured
-  Unorderdness
{{< figure src="Desktop/MLP.png" title="Challenges of point cloud data!" >}}
---
# What is PointNet?
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

## Joint Alignment Network
Labeling an object should be invariant to rigid transformations of that object.
Pointnet solves this by attempting to align all input set to a canonical space prior to feature extraction.

Pointnet predicts an affine transformation matrix using a sub-network, `T-Net`, and applying the predicted transformation
to the input points.

The idea is extended to feature space as well, however due to high dimensionality a regularization term is added to the 
Softmax loss function.

$$ L_{reg} = ||I - AA^T||_F^2 $$

---

## T-Net

---

## Local <> Global Information Combination

---

## Max pooling aggregation layer

---

# Questions?
### 🙋🏿‍♂️ 🙋🏻 🙋🏽‍♀️ 🙋‍♂️
