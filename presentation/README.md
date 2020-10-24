---
marp: true
--- 
<!--
theme: gaia
class:
 - invert
headingDivider: 2 
-->

<!--
_class:
 - lead
 - invert
-->
# PointNet
Segmentation and Classification of point-clouds with Deep Learning

`Alex Launi & Tareq Alghamdi `
` ODU CS722 Fall 2020`

# What is PointNet?
<!-- paginate: true -->
<!-- _class: lead invert-->
PointNet is a **deep learning** approach for **scene segmentation** and **object classification** of 3D structures.

![w:500 drop-shadow:0,20px,10px,rgba(0,0,0,.4)](images/tasks.png)

*for example:* Take a Microsoft Kinect scan of a room. We can use PointNet to find all of the unique objects in the scan, and identify them!

---
PointNet is a novel approach to 3D-structure classification. Many prior approaches transformed data into structured formats like
 - 3D Voxels
 - Image grids

but PointNet operates directly on **sets of points** using a *convolution network* and *max pooling function*.

**PointNet learns to summarize a point cloud by a spare set of key points, which is approximately an object's skeleton** 💀

# Questions?
<!--
_class:
 - lead
 - invert
-->
### 🙋🏿‍♂️ 🙋🏻 🙋🏽‍♀️ 🙋‍♂️