use serde::{Serialize, Deserialize};
use crate::building_representations::triangulized_walls::TrianguizedWalls;
use crate::building_representations::polygon_walls::PolygonWalls;
use crate::request_for_isolation::Request;
use crate::general_geometry::{Polygon, Point, PolygonPointsOnSides, Corner, LineSegment, PositiveF64, Angle};
use crate::building_representations::converters;
use crate::tiling::{Tile, TriangulizedTiles, corner_handling};
use crate::request_for_isolation::PolygonWithIsolationDetails;
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    pub building: TrianguizedWalls,
    pub tiles: TriangulizedTiles,
}

pub fn generate_plan(request: &Request) -> Plan {
    let building: PolygonWalls = polygon_walls_from_request(request);

    Plan {
        building: converters::polygon_walls_to_triangulized_walls(building),
        tiles: triangulized_tiles(get_tiling(request))
    }
}

fn polygon_walls_from_request(request: &Request) -> PolygonWalls {
    let mut walls: Vec<Polygon> = vec![];

    for wall in request.data() {
        walls.push(wall.polygon().clone())
    }
    
    PolygonWalls::new(walls)
}

fn triangulized_tiles(tiles: Vec<Tile>) -> TriangulizedTiles {
    TriangulizedTiles::from_tiles(tiles)
}

fn get_tiling(request: &Request) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = vec![];
    let mut i: usize = 0;

    while i < request.data().len() {
        match request.data()[i].isolation() {
            Option::Some(detail) => {
                tiles.append(&mut get_tiles_from_wall_in_building(i, request, detail.width()));
            },
            Option::None => {

            }
        }

        i+=1;
    }

    tiles
}

fn get_tiles_from_wall_in_building(ind: usize, request: &Request, isolation_width: f64) -> Vec<Tile> {
    let mut res: Vec<Tile> = vec![];

    let borders = get_borders_for_wall(ind, request);
    let wall = request.data()[ind].polygon();

    let mut base_rim: Vec<Point> = vec![];
    let mut surface_rim: Vec<Point> = vec![];

    for border in borders {
        match border.wall_ind {
            Some(val) => {
                let solved_corner = solve_corner(&LineSegment::new(border.point_a, border.point_b), &request.data()[ind], &request.data()[val]);

                surface_rim.push(solved_corner.2);
                surface_rim.push(solved_corner.3);
                base_rim.push(solved_corner.0);
                base_rim.push(solved_corner.1);
            },
            None => {
                surface_rim.push(border.point_a.add(&wall.normal().normalize().multiply(isolation_width)));
                surface_rim.push(border.point_b.add(&wall.normal().normalize().multiply(isolation_width)));
                base_rim.push(border.point_a);
                base_rim.push(border.point_b);
            }
        }
    }

    // let (base_rim, surface_rim) = further_process_base_and_surface_rims(&base_rim, &surface_rim);

    res.push(Tile::new(PolygonPointsOnSides::new(base_rim, vec![]), PolygonPointsOnSides::new(surface_rim, vec![])));
    
    res
}

fn further_process_base_and_surface_rims(base_rim: &Vec<Point>, surface_rim: &Vec<Point>) -> (Vec<Point>, Vec<Point>) {
    (vec![], vec![])
}

fn solve_corner(shared_segment: &LineSegment, observing_wall: &PolygonWithIsolationDetails, bordering_wall: &PolygonWithIsolationDetails) -> (Point, Point, Point, Point) {

    let angle = observing_wall.polygon().normal().angle_to(&bordering_wall.polygon().normal()).val() - std::f64::consts::PI;
    let angle = Angle::new(angle);
    let width1 = PositiveF64::new(observing_wall.isolation().as_ref().unwrap().width()).unwrap();
    let width2 = PositiveF64::new(bordering_wall.isolation().as_ref().unwrap().width()).unwrap();

    let res = corner_handling::handle_corner(angle, width1, width2);

    // first return base points and then surface points
    let pt1 = shared_segment.p1().clone();
    let pt2 = shared_segment.p2().clone();
    let pt3 = shared_segment.p1().add(&observing_wall.polygon().normal().normalize().multiply(observing_wall.isolation().as_ref().unwrap().width()));
    let pt4 = shared_segment.p2().add(&observing_wall.polygon().normal().normalize().multiply(observing_wall.isolation().as_ref().unwrap().width()));

    (pt1, pt2, pt3, pt4)
}

fn get_angle_between_walls() {
    
}

fn get_borders_for_wall(ind: usize, request: &Request) -> Vec<Border> {
    let walls = request.data().into_iter().map(|a| a.polygon().clone()).collect::<Vec<_>>();

    let corners = Polygon::get_all_corners_on_polygon(ind, &walls);
    corners_to_borders(&corners, &walls[ind])
}

fn corners_to_borders(corners: &Vec<Corner>, wall: &Polygon) -> Vec<Border> {
    let mut res = vec![];

    let rl = wall.rim().len();
    let mut i = 0;

    while i < rl {
        let tmp = &wall.rim()[i];
        let next = &wall.rim()[(i+1)%rl];
        let mut corners_on_this_side: Vec<Corner> = corners.into_iter().filter(|corner| corner.ind_of_side_in_this_polygon == i).map(|corner| corner.clone()).collect::<Vec<Corner>>();

        res.append(&mut corners_on_one_side_to_borders(&mut corners_on_this_side, tmp, next));
        i+=1;
    }

    res
}

fn corners_on_one_side_to_borders(corners: &mut Vec<Corner>, start: &Point, end: &Point) -> Vec<Border> {
    corners.sort_by(|a, b| a.pt.subtract(&start).modulo().partial_cmp(&b.pt.subtract(&start).modulo()).unwrap_or(Ordering::Equal));

    let mut i = 0;
    let mut res = vec![];
    let mut prev = start;

    if corners.len() == 0 {
        res.push(Border {
            point_a: start.clone(),
            point_b: end.clone(),
            wall_ind: None
        });
    }

    while i < corners.len() {
        let seg_start = &corners[i].pt;
        let seg_end = &corners[i+1].pt;
        if !Point::are_points_simmilar(seg_start, prev) {
            res.push(Border {
                point_a: prev.clone(),
                point_b: seg_start.clone(),
                wall_ind: None
            });
        }
        
        res.push(Border {
            point_a: seg_start.clone(),
            point_b: seg_end.clone(),
            wall_ind: Some(corners[i].ind_of_bordering_polygon)
        });
        
        prev = seg_end;
        i+=2;
    }

    res
}

struct Border {
    point_a: Point,
    point_b: Point, 
    wall_ind: Option<usize>
}
