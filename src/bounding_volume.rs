use std::ops::Range;

use rand::Rng;

use crate::{
    hittable::{AxisAlignedBoundingBox, HitRecord, Hittable, HittableList},
    ray::Ray,
};

#[derive(Debug)]
pub struct BoundingVolumeHierarchyNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    bounding_box: AxisAlignedBoundingBox,
}

impl BoundingVolumeHierarchyNode {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        let compare_function = match rand::thread_rng().gen_range(0..2) {
            0 => AxisAlignedBoundingBox::compare_x,
            1 => AxisAlignedBoundingBox::compare_y,
            _ => AxisAlignedBoundingBox::compare_z,
        };

        if objects.len() == 1 {
            let bounding_box = objects[0].bounding_box().clone();

            Self {
                left: Some(objects.remove(0)),
                right: None,
                bounding_box,
            }
        } else if objects.len() == 2 {
            let first = objects.remove(0);
            let second = objects.remove(0);

            let bounding_box = AxisAlignedBoundingBox::from_boxes(
                first.bounding_box().clone(),
                second.bounding_box().clone(),
            );

            if compare_function(first.bounding_box(), second.bounding_box()).is_lt() {
                Self {
                    left: Some(first),
                    right: Some(second),
                    bounding_box,
                }
            } else {
                Self {
                    left: Some(second),
                    right: Some(first),
                    bounding_box,
                }
            }
        } else {
            objects
                .sort_by(|box0, box1| compare_function(box0.bounding_box(), box1.bounding_box()));
            let middle_index = objects.len() / 2;

            let right_objects = objects.split_off(middle_index);
            let mut left_objects = objects;

            let left_bounding_volume = if left_objects.len() == 1 {
                left_objects.remove(0)
            } else {
                Box::new(Self::new(left_objects))
            };
            let right_bounding_volume = Box::new(Self::new(right_objects));
            let bounding_box = AxisAlignedBoundingBox::from_boxes(
                left_bounding_volume.bounding_box().clone(),
                right_bounding_volume.bounding_box.clone(),
            );

            Self {
                left: Some(left_bounding_volume),
                right: Some(right_bounding_volume),
                bounding_box,
            }
        }
    }
}

impl From<HittableList> for BoundingVolumeHierarchyNode {
    fn from(value: HittableList) -> Self {
        let objects = value.objects();

        BoundingVolumeHierarchyNode::new(objects)
    }
}

impl Hittable for BoundingVolumeHierarchyNode {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_range.clone()) {
            return None;
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

        match (hit_right, hit_left) {
            (Some(right), Some(left)) => {
                if right.t < left.t {
                    Some(right)
                } else {
                    Some(left)
                }
            }
            (Some(right), None) => Some(right),
            (None, Some(left)) => Some(left),
            _ => None,
        }
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}
