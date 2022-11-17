use crate::general_geometry::{Point};

#[derive(Debug)]
pub struct CoordinateSystem3D {
    o: Point,
    x: Point, 
    y: Point,
    z: Point
}

impl CoordinateSystem3D {

    pub const BASE: CoordinateSystem3D = CoordinateSystem3D::new(
        Point::new(0., 0., 0.), Point::new(1., 0., 0.), Point::new(1., 0., 0.), Point::new(1., 0., 0.)
    );

    pub const fn new(o: Point, x: Point, y: Point, z: Point) -> Self {
        Self {o, x, y, z}
    }
    
    pub fn translate(&self, v: &Point) -> CoordinateSystem3D {
        Self::new(self.o.add(v), self.x.clone(), self.y.clone(), self.z.clone())
    }

    pub fn o(&self) -> &Point {
        &self.o
    }

    pub fn x(&self) -> &Point {
        &self.x
    }

    pub fn y(&self) -> &Point{
        &self.y
    }

    pub fn z(&self) -> &Point{
        &self.z
    }

    pub fn origin_at_zero(&self) -> bool {
        Point::are_points_simmilar(self.o(), &Point::ZERO)
    }

    pub fn inverse_system(&self) -> CoordinateSystem3D {
        if !self.origin_at_zero() {
            panic!("Origin should be at zero. Actually was: {:?}", self.o());
        }

        let m = self.to_matrix();
        let inverse_m = Point::inverse_mat(&m);

        CoordinateSystem3D::new(Point::ZERO, inverse_m[0].clone(), inverse_m[1].clone(), inverse_m[2].clone())
    }

    fn to_matrix(&self) -> Vec<Point> {
        vec![
            self.x.clone(),self.y.clone(), self.z.clone()
        ]
    }

}

