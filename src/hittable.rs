use std::{ops::Range, sync::Arc};

use glam::DVec3;

use crate::{material::Material, ray::Ray, bounding_volume::BoundingVolumeHierarchyNode};

use self::util::combined;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitRecord>;

    fn bounding_box(&self) -> &AxisAlignedBoundingBox;
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        ray: Ray,
        point: DVec3,
        outward_normal: DVec3,
        t: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            material,
            front_face,
        }
    }
}

#[derive(Default, Clone)]
pub struct AxisAlignedBoundingBox {
    x: Range<f64>,
    y: Range<f64>,
    z: Range<f64>,
}

impl AxisAlignedBoundingBox {
    pub fn new(x: Range<f64>, y: Range<f64>, z: Range<f64>) -> Self {
        Self { x, y, z }
    }

    pub fn from_corners(corner0: DVec3, corner1: DVec3) -> Self {
        let min = corner0.min(corner1);
        let max = corner0.max(corner1);
        Self {
            x: min.x..max.x,
            y: min.y..max.y,
            z: min.z..max.z,
        }
    }

    pub fn from_boxes(box0: Self, box1: Self) -> Self {
        Self {
            x: combined(box0.x, box1.x),
            y: combined(box0.y, box1.y),
            z: combined(box0.z, box1.z),
        }
    }

    pub fn hit(&self, ray: Ray, mut t_range: Range<f64>) -> bool {

        let (t0, t1) = Self::calculate_t0_t1(&self.x, ray.origin.x, ray.direction.x);

        t_range.start = t0.max(t_range.start);
        t_range.end = t1.min(t_range.end);

        if t_range.end <= t_range.start {
            return false;
        }

        let (t0, t1) = Self::calculate_t0_t1(&self.y, ray.origin.y, ray.direction.y);

        t_range.start = t0.max(t_range.start);
        t_range.end = t1.min(t_range.end);

        if t_range.end <= t_range.start {
            return false;
        }

        let (t0, t1) = Self::calculate_t0_t1(&self.z, ray.origin.z, ray.direction.z);

        t_range.start = t0.max(t_range.start);
        t_range.end = t1.min(t_range.end);

        if t_range.end <= t_range.start {
            return false;
        }

        true
    }

    fn calculate_t0_t1(range: &Range<f64>, origin: f64, direction: f64) -> (f64, f64) {
        let t0 = ((range.start - origin) / direction).min((range.end - origin) / direction);
        let t1 = ((range.start - origin) / direction).max((range.end - origin) / direction);

        (t0, t1)
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    bounding_box: AxisAlignedBoundingBox,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        let mut bounding_box = AxisAlignedBoundingBox::default();

        for object in &objects {
            bounding_box =
                AxisAlignedBoundingBox::from_boxes(bounding_box, object.bounding_box().clone());
        }

        Self {
            objects,
            bounding_box,
        }
    }

    pub(crate) fn objects(self) -> Vec<Box<dyn Hittable>> {
        self.objects
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_range.clone()) {
            return None;
        }

        let mut closest_hit = None;
        let mut closest_so_far = t_range.end;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_range.start..closest_so_far) {
                closest_so_far = hit.t;

                closest_hit = Some(hit);
            }
        }

        closest_hit
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}

mod util {
    use std::ops::Range;

    pub fn expand(delta: f64, range: Range<f64>) -> Range<f64> {
        let padding = delta / 2.0;
        (range.start - padding)..(range.end + padding)
    }

    pub fn combined(range0: Range<f64>, range1: Range<f64>) -> Range<f64> {
        range0.start.min(range1.start)..(range1.end.max(range1.end))
    }
}
