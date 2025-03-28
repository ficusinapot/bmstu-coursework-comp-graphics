use crate::visitor::{Visitable, Visitor};
use glam::Vec3;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct BoundingBox {
    /// One of the corners of the rectangle, usually the left top one.
    pub min: Vec3,

    /// The other corner, opposing [`Self::min`]. Usually the right bottom one.
    pub max: Vec3,
}

impl BoundingBox {
    pub fn move_center(&mut self, new_center: Vec3) {
        let offset = new_center - self.center();
        self.min += offset;
        self.max += offset;
    }
}

impl From<(Vec3, Vec3)> for BoundingBox {
    fn from(value: (Vec3, Vec3)) -> Self {
        Self::from_two_pos(value.0, value.1)
    }
}

impl BoundingBox {
    #[inline]
    pub fn from_two_pos(a: Vec3, b: Vec3) -> Self {
        Self {
            min: Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
            max: Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
        }
    }

    #[inline]
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    #[inline]
    pub fn center(&self) -> Vec3 {
        0.5 * (self.max + self.min)
    }

    #[inline]
    pub fn contains(&self, position: Vec3) -> bool {
        (self.min.x..=self.max.x).contains(&position.x)
            && (self.min.y..=self.max.y).contains(&position.y)
            && (self.min.z..=self.max.z).contains(&position.z)
    }

    pub fn dst(&self, ray_origin: Vec3, ray_dir: Vec3) -> glam::Vec2 {
        let t0 = (self.min - ray_origin) / ray_dir;
        let t1 = (self.max - ray_origin) / ray_dir;

        let tmin = t0.min(t1);
        let tmax = t0.max(t1);

        let dst_a = tmin.x.max(tmin.y).max(tmin.z);
        let dst_b = tmax.x.min(tmax.y).min(tmax.z);

        let dst_to_box = 0.0_f32.max(dst_a);
        let dst_inside_box = 0.0_f32.max(dst_b - dst_to_box);

        (dst_to_box, dst_inside_box).into()
    }

    #[inline]
    pub const fn corners(&self) -> [Vec3; 8] {
        let [x1, y1, z1] = self.min.to_array();
        let [x2, y2, z2] = self.max.to_array();
        [
            Vec3::new(x1, y1, z1),
            Vec3::new(x2, y1, z1),
            Vec3::new(x1, y1, z2),
            Vec3::new(x2, y2, z1),
            Vec3::new(x1, y2, z1),
            Vec3::new(x2, y1, z2),
            Vec3::new(x1, y2, z2),
            Vec3::new(x2, y2, z2),
        ]
    }

    pub const fn edges(&self) -> [(usize, usize); 12] {
        [
            (0, 1),
            (1, 5),
            (5, 4),
            (4, 0),
            (2, 3),
            (3, 7),
            (7, 6),
            (6, 2),
            (0, 2),
            (1, 3),
            (4, 6),
            (5, 7),
        ]
    }
}

impl Visitable for BoundingBox {
    #[inline]
    fn accept(&self, visitor: &mut impl Visitor) {
        visitor.visit_bounding_box(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bb() {
        let bb = BoundingBox::from_two_pos(Vec3::new(-1.0, 1.0, -1.0), Vec3::new(1.0, 1.5, 1.0));
        assert_eq!(
            glam::Vec2::new(4.0, 2.0),
            bb.dst(Vec3::new(5.0, 1.25, 0.0), Vec3::new(-1.0, 0.0, 0.0))
        );
    }

    #[test]
    fn test_bb_contains() {
        let bb = BoundingBox::from_two_pos(Vec3::new(-1.0, 1.0, -1.0), Vec3::new(1.0, 1.5, 1.0));

        assert!(bb.contains(bb.center()));
        assert!(bb.contains(Vec3::new(0.0, 1.25, 0.0)));
        assert!(bb.contains(Vec3::new(-0.5, 1.25, 0.5)));
        assert!(bb.contains(Vec3::new(0.5, 1.25, -0.5)));

        assert!(!bb.contains(Vec3::new(1.1, 1.25, 0.0)));
        assert!(!bb.contains(Vec3::new(0.0, 1.6, 0.0)));
        assert!(!bb.contains(Vec3::new(0.0, 1.25, 1.1)));
        assert!(!bb.contains(Vec3::new(-2.0, 1.25, 0.0)));
    }
}
