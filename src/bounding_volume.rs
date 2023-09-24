use std::ops::Range;

use crate::{hittable::{Hittable, AxisAlignedBoundingBox, HitRecord, HittableList}, ray::Ray};

pub struct BoundingVolumeHierarchyNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    bounding_box: AxisAlignedBoundingBox,
}

impl BoundingVolumeHierarchyNode {
    pub fn new(objects: Vec<Box<dyn Hittable>>, start: usize, end: usize) -> Self {
        todo!()
    }
}

impl From<HittableList> for BoundingVolumeHierarchyNode {
    fn from(value: HittableList) -> Self {
        let objects = value.objects();
        let length = objects.len();

        BoundingVolumeHierarchyNode::new(objects, 0, length)
    }
}

impl Hittable for BoundingVolumeHierarchyNode {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitRecord> {
        if self.bounding_box.hit(ray, t_range.clone()) {
            return None
        }

        let hit_left = if let Some(left) = &self.left {
            left.hit(ray, t_range.clone())
        } else {
            None
        };

        let hit_right = if let Some(right) = &self.right {
            if let Some(left) = &hit_left {
                right.hit(ray, t_range.start..left.t)
            } else {
                right.hit(ray, t_range)
            }
        } else {
            None
        };

        if hit_right.is_some() {
            hit_right
        } else {
            hit_left
        }
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}