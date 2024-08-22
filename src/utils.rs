use iced::{Point, Size, Vector};

pub struct Record<T: Clone> {
    pub cur: T,
    pub prev: T,
}

impl<T: Clone> Record<T> {
    pub fn new(val: T) -> Record<T> {
        Record {
            cur: val.clone(),
            prev: val,
        }
    }
}

impl<T: Clone> Record<T> {
    pub fn set(&mut self, val: T) {
        self.prev = self.cur.clone();
        self.cur = val;
    }
}

pub trait PointExt {
    fn to_world(
        &self,
        pixels_per_unit: f32,
        zoom: f32,
        translation: Vector,
        bounds: Size,
    ) -> Vector;
}

impl PointExt for Point {
    fn to_world(
        &self,
        pixels_per_unit: f32,
        zoom: f32,
        translation: Vector,
        bounds: Size,
    ) -> Vector {
        let space_between = pixels_per_unit * zoom;
        Vector {
            x: ((self.x - bounds.width / 2.0) + translation.x * space_between) / space_between,
            y: (-(self.y - bounds.height / 2.0) + translation.y * space_between) / space_between,
        }
    }
}
