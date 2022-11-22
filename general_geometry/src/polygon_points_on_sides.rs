use serde::{Serialize, Deserialize};
use crate::{Point, Polygon};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolygonPointsOnSides {
    rim: Vec<Point>,
    holes: Vec<Vec<Point>>
}

impl PolygonPointsOnSides {
    pub fn new(rim: Vec<Point>, holes: Vec<Vec<Point>>) -> PolygonPointsOnSides {
        PolygonPointsOnSides {
            rim,
            holes
        }
    }

    pub fn translate(&self, inc: &Point) -> PolygonPointsOnSides {
        let translate_vec_points = |x: &Vec<Point>| x.iter().map(|pt| pt.add(inc)).collect::<Vec<_>>();

        let rim = translate_vec_points(self.rim());
        let holes = self.holes().iter().map(|hole| translate_vec_points(hole)).collect::<Vec<_>>();

        PolygonPointsOnSides::new(rim, holes)
    }

    fn remove_duplicates(rim: Vec<Point>) -> Vec<Point> {
        let mut i = 1;
        let mut res: Vec<Point> = vec![rim[0].clone()];
        let mut prev = &rim[0];

        while i < rim.len() {
            while i< rim.len() && Point::are_points_simmilar(prev, &rim[i]) {
                i+=1;
            }
            if i < rim.len() {
                res.push(rim[i].clone());
                prev = &rim[i];
            }
            i+=1;
        }
        
        if Point::are_points_simmilar(&res[res.len() - 1], &res[0]) {
            res.pop();
        }

        res
    }

    pub fn to_polygon(&self) -> (Vec<Point>, Vec<Vec<Point>>) {
        let rim: Vec<Point> = Self::remove_duplicates(Self::remove_points_not_on_corners_one_ring(self.rim()));
        let holes: Vec<Vec<Point>> = self.holes().into_iter().map(|hole| Self::remove_duplicates(Self::remove_points_not_on_corners_one_ring(hole))).collect::<Vec<_>>();

        (rim, holes)
    }   

    pub fn to_polygon_true_type(&self) -> Polygon {
        let poly_comps = self.to_polygon();
        Polygon::new(poly_comps.0, poly_comps.1)
    }

    fn remove_points_not_on_corners_one_ring(rim: &Vec<Point>) -> Vec<Point> {
        let mut res: Vec<Point> = vec![];
        let mut i: usize = 0;
        let rl = rim.len();

        while i < rl {
            let mut t_next = (i+1)%rl;
            while Point::are_points_simmilar(&rim[i], &rim[t_next]){
                t_next = (t_next + 1)%rl;
            }
            let next = &rim[t_next];

            let mut t_prev = (i + rl - 1)%rl;
            while Point::are_points_simmilar(&rim[i], &rim[t_prev]){
                t_prev = (t_prev + rl - 1)%rl;
            }
            let prev = &rim[t_prev];

            if !Point::are_points_colinear(prev, &rim[i], next) {
                res.push(rim[i].clone());
            }
            i+=1;
        }

        res
    }

    pub fn rim<'a>(&'a self) -> & 'a Vec<Point> {
        &self.rim
    }

    pub fn holes<'a>(&'a self) -> & 'a Vec<Vec<Point>> {
        &self.holes
    }
}
