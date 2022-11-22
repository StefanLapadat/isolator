// TODO fn polygon_walls_to_triangulized_walls  

use crate::building_representations::levels::Levels;
use crate::building_representations::polygon_walls::PolygonWalls;
use crate::building_representations::triangulized_walls::TrianguizedWalls;
use general_geometry::{Polygon, Point};

pub fn levels_to_polygon_walls(levels: Levels) -> PolygonWalls {
    let mut walls: Vec<Polygon> = vec![];
    let mut total_height: f64 = 0.;

    for level in levels.levels() {
        let mut i: usize = 0;
        let lrl = level.rim().rim().len();
        let mut horizontal_wall_rim: Vec<Point> = vec![];

        while i < lrl {
            let vertical_wall_rim: Vec<Point> = vec![
                Point::new(level.rim().rim()[i].x, level.rim().rim()[i].y, total_height),
                Point::new(level.rim().rim()[(i+1) % lrl].x, level.rim().rim()[(i+1) % lrl].y, total_height),
                Point::new(level.rim().rim()[(i+1) % lrl].x, level.rim().rim()[(i+1) % lrl].y, total_height+level.height()),
                Point::new(level.rim().rim()[i].x, level.rim().rim()[i].y, total_height+level.height())
            ];
        
            walls.push(Polygon::new(vertical_wall_rim, vec![]));
            horizontal_wall_rim.push(Point::new(level.rim().rim()[i].x, level.rim().rim()[i].y, total_height + level.height()));

            i += 1;
        }

        walls.push(Polygon::new(horizontal_wall_rim, vec![]));

        total_height += level.height();
    }

    PolygonWalls::new(walls)
}

pub fn polygon_walls_to_triangulized_walls(polygon_walls: PolygonWalls) -> TrianguizedWalls {
    TrianguizedWalls::new(polygon_walls.triangulation(), polygon_walls.wireframe())
}
