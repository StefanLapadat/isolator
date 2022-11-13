use serde::{Serialize, Deserialize};
use crate::general_geometry::{Point};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolygonPointsOnSides {
    rim: Vec<Point>,
    holes: Vec<Vec<Point>>
}

impl PolygonPointsOnSides {
    pub fn new(rim: Vec<Point>, holes: Vec<Vec<Point>>) -> PolygonPointsOnSides {
        PolygonPointsOnSides {
            rim: Self::remove_duplicates(rim),
            holes
        }
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
        let rim: Vec<Point> = Self::remove_points_not_on_corners_one_ring(self.rim());
        let holes: Vec<Vec<Point>> = self.holes().into_iter().map(|hole| Self::remove_points_not_on_corners_one_ring(hole)).collect::<Vec<_>>();

        (rim, holes)
    }   

    fn remove_points_not_on_corners_one_ring(rim: &Vec<Point>) -> Vec<Point> {
        let mut res: Vec<Point> = vec![];
        let mut i: usize = 0;
        let rl = rim.len();

        while i < rl {
            let next = &rim[(i+1)%rl];
            let prev = &rim[(i + rl -1)%rl];
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

    pub fn translate(&self, inc: &Point) -> PolygonPointsOnSides {
        let mut rim: Vec<Point> = vec![];
        let mut holes: Vec<Vec<Point>> = vec![];

        for point in self.rim() {
            rim.push(point.add(&inc));
        }

        for hole in self.holes() {
            let mut new_hole: Vec<Point> = vec![];
            for point in hole {
                new_hole.push(point.add(&inc));
            }
            holes.push(new_hole);
        }

        PolygonPointsOnSides {
            rim,
            holes
        }
    }

}




