use serde::{Serialize, Deserialize};
use crate::building_representations::triangulized_walls::TrianguizedWalls;
use crate::building_representations::polygon_walls::PolygonWalls;
use crate::plan_execution::PlanExecutionEvent;
use crate::request_for_isolation::Request;
use crate::tiling::tile_with_adhesive::TriangulizedTilesWithAdhesive;
use general_geometry::{Polygon, Point, PolygonPointsOnSides, Corner, LineSegment, Line3D, line3d, Plane};
use crate::building_representations::converters;
use crate::tiling::{Tile, tile, TileWithAdhesive};
use crate::request_for_isolation::PolygonWithIsolationDetails;
use std::cmp::Ordering;
use crate::{PlanExecution, PlanExecutionCreator};

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    pub building: TrianguizedWalls,
    pub tiles: TriangulizedTilesWithAdhesive,
    pub planExecution: PlanExecution
}

pub fn generate_plan(request: &Request) -> Plan {
    let building: PolygonWalls = polygon_walls_from_request(request);
    let tiles: Vec<TileWithAdhesive> = get_tiling(request);
    let planExecution = PlanExecutionCreator::new(10.).create_plan(&building, &tiles, request.hooks(), 50., 1);

    Plan {
        building: converters::polygon_walls_to_triangulized_walls(building),
        tiles: TriangulizedTilesWithAdhesive::from_tiles(tiles),
        planExecution
    }
}

fn polygon_walls_from_request(request: &Request) -> PolygonWalls {
    let mut walls: Vec<Polygon> = vec![];

    for wall in request.data() {
        walls.push(wall.polygon().clone())
    }
    
    PolygonWalls::new(walls)
}

fn get_tiling(request: &Request) -> Vec<TileWithAdhesive> {
    let mut tiles: Vec<Tile> = vec![];
    let mut i: usize = 0;

    while i < request.data().len() {
        match request.data()[i].isolation() {
            Some(detail) => {
                tiles.append(&mut get_tiles_from_wall_in_building(i, request, detail.width()));
            },
            None => {}
        }

        i+=1;
    }

    let splitted_tiles = tiles.into_iter().map(|t| tile::split_into_tiles(&t, &request.unit_tile()).unwrap()).flatten().collect::<Vec<_>>();
    tiles_into_tiles_with_adhesive(splitted_tiles)
}

fn tiles_into_tiles_with_adhesive(tiles: Vec<Tile>) -> Vec<TileWithAdhesive> {
    tiles.iter().map(|tile| tile_into_tile_with_adhesive(tile)).collect::<Vec<_>>()
}

fn tile_into_tile_with_adhesive(tile: &Tile) -> TileWithAdhesive {

    let adhesive_tile_base = tile.base_polygon().clone();
    let styro_tile_surface = tile.surface_polygon().clone();

    let adhesive_tile_surface = tile.split_surface(0.1);
    let styro_tile_base = adhesive_tile_surface.clone();

    let adhesive_tile = Tile::new(adhesive_tile_base, adhesive_tile_surface);
    let styro_tile = Tile::new(styro_tile_base, styro_tile_surface);

    TileWithAdhesive::new(styro_tile, adhesive_tile)
}

fn get_tiles_from_wall_in_building(ind: usize, request: &Request, isolation_width: f64) -> Vec<Tile> {
    let mut res: Vec<Tile> = vec![];

    let borders = get_borders_for_wall(ind, request);
    let wall = request.data()[ind].polygon();
    let wall_height_vec = &wall.normal().normalize().multiply(isolation_width);

    let mut base_rim: Vec<Vec<Point>> = vec![];
    let mut surface_rim: Vec<Vec<Point>> = vec![];

    let mut i = 0;
    while i<borders.len() {
        let one_side_borders = &borders[i];

        let mut one_side_base_rim: Vec<Point> = vec![];
        let mut one_side_surface_rim: Vec<Point> = vec![];

        for border in one_side_borders {
            match border.wall_ind {
                Some(val) => {
                    let solved_corner = solve_corner2(&LineSegment::new(border.point_a.clone(), border.point_b.clone()), &request.data()[ind], &request.data()[val]);
    
                    one_side_base_rim.push(solved_corner.0);
                    one_side_base_rim.push(solved_corner.1);
                    one_side_surface_rim.push(solved_corner.2);
                    one_side_surface_rim.push(solved_corner.3);
                },
                None => {
                    one_side_surface_rim.push(border.point_a.add(&wall.normal().normalize().multiply(isolation_width)));
                    one_side_surface_rim.push(border.point_b.add(&wall.normal().normalize().multiply(isolation_width)));
                    one_side_base_rim.push(border.point_a.clone());
                    one_side_base_rim.push(border.point_b.clone());
                }
            }
        }

        base_rim.push(one_side_base_rim);
        surface_rim.push(one_side_surface_rim);

        i+=1;
    }

    let surface_rim = further_process_base_or_surface_rim(&surface_rim);
    let base_rim = further_process_base_or_surface_rim(&base_rim);

    let flat_base_rim = base_rim.into_iter().flatten().collect::<Vec<_>>();
    let flat_surface_rim = surface_rim.into_iter().flatten().collect::<Vec<_>>();

    let base_holes: Vec<Vec<Point>> = request.data()[ind].polygon().holes().clone();
    let surface_holes = base_holes.clone().into_iter().map(|hole| hole.into_iter().map(|hole_point| hole_point.add(wall_height_vec)).collect::<Vec<_>>()).collect::<Vec<_>>();

    res.push(Tile::new(PolygonPointsOnSides::new(flat_base_rim, base_holes), PolygonPointsOnSides::new(flat_surface_rim, surface_holes)));

    res
}

fn further_process_base_or_surface_rim(rim: &Vec<Vec<Point>>) -> Vec<Vec<Point>> {
    let mut res = vec![];

    let mut i = 0;
    let srl = rim.len();
    while i < srl {
        let prev_side = &rim[(i + srl - 1) % srl];
        let last_point_on_prev_side = &prev_side[prev_side.len() - 1];
        let prev_last_point_on_prev_side = &prev_side[prev_side.len() - 2];
        let prev_side_last_line = Line3D::from_2_points(prev_last_point_on_prev_side, last_point_on_prev_side).unwrap();

        let this_side = &rim[i];
        let first_line_this_side = Line3D::from_2_points(&this_side[0], &this_side[1]).unwrap();

        let last_line_this_side = Line3D::from_2_points(&this_side[this_side.len() - 1], &this_side[this_side.len() - 2]).unwrap();

        let next_side = &rim[(i + 1) % srl];
        let first_point_on_next_side = &next_side[0];
        let second_point_on_next_side = &next_side[1];
        let next_side_first_line = Line3D::from_2_points(first_point_on_next_side, second_point_on_next_side).unwrap();
        
        let first_point_in_result: Point;
        match line3d::intersection(&prev_side_last_line, &first_line_this_side) {
            line3d::Intersection::Point(pt) => {
                first_point_in_result = pt;
            },
            _ => panic!("Unexpected")
        }

        let last_point_in_result: Point;
        match line3d::intersection(&last_line_this_side, &next_side_first_line) {
            line3d::Intersection::Point(pt) => {
                last_point_in_result = pt;
            },
            _ => panic!("Unexpected")
        }

        let mut res_elem = vec![];

        res_elem.push(first_point_in_result);
        let mut j = 1;
        while j < this_side.len() - 1 {
            res_elem.push(this_side[j].clone());
            j+=1;
        }
        res_elem.push(last_point_in_result);

        res.push(res_elem);

        i+=1;
    }

    res
}

fn solve_corner1(shared_segment: &LineSegment, observing_wall: &PolygonWithIsolationDetails, bordering_wall: &PolygonWithIsolationDetails) -> (Point, Point, Point, Point) {

    let inc2: Point; 
    let shared_segment_vec = shared_segment.to_point();

    match bordering_wall.isolation() {
        Some(val) => {
            inc2 = bordering_wall.polygon().normal().normalize().multiply(val.width());
        },
        None => {
            return solve_corner_isolated_next_to_non_isolated_wall(shared_segment, observing_wall, bordering_wall);
        }
    }

    let inc1 = observing_wall.polygon().normal().normalize().multiply(observing_wall.isolation().as_ref().unwrap().width());


    let line1 = Line3D::new(Point::cross_prod(&shared_segment_vec, &inc1), shared_segment.p1().add(&inc1));
    let line2 = Line3D::new(Point::cross_prod(&shared_segment_vec, &inc2), shared_segment.p1().add(&inc2));

    let intersection = line3d::intersection(&line1.unwrap(), &line2.unwrap());

    match intersection {
        line3d::Intersection::None => panic!("No intersection found where it was expected."),
        line3d::Intersection::Line(_) => panic!("Whole line found as an intersection where single point was expected."),
        line3d::Intersection::Point(pt) => {
            let pt1 = shared_segment.p1().clone();
            let pt2 = shared_segment.p2().clone();
            let pt3 = pt.clone();
            let pt4 = pt.clone().add(&shared_segment_vec);

            (pt1, pt2, pt3, pt4)
        }
    }
}

fn solve_corner2(shared_segment: &LineSegment, observing_wall: &PolygonWithIsolationDetails, bordering_wall: &PolygonWithIsolationDetails) -> (Point, Point, Point, Point) {

    let bor_wall_width_vec: Point; 

    match bordering_wall.isolation() {
        Some(val) => {
            bor_wall_width_vec = bordering_wall.polygon().normal().normalize().multiply(val.width());
        },
        None => {
            return solve_corner_isolated_next_to_non_isolated_wall(shared_segment, observing_wall, bordering_wall);
        }
    }

    let obs_wall_width_vec = observing_wall.polygon().normal().normalize().multiply(observing_wall.isolation().as_ref().unwrap().width());

    let shared_segment_vec = shared_segment.to_point();

    let obs_wall_surface_line: Line3D;
    
    match Line3D::new(Point::cross_prod(&shared_segment_vec, &obs_wall_width_vec), shared_segment.p1().add(&obs_wall_width_vec)) {
        Some(val) => obs_wall_surface_line = val,
        None => panic!("What the heck!? shared_seg_vec: {:?} {:?}", &shared_segment, obs_wall_width_vec)
    }



    let bor_wall_surface_line = Line3D::new(Point::cross_prod(&shared_segment_vec, &bor_wall_width_vec), shared_segment.p1().add(&bor_wall_width_vec)).unwrap();

    let surface_lines_intersection = line3d::intersection(&obs_wall_surface_line, &bor_wall_surface_line);

    match surface_lines_intersection {
        line3d::Intersection::None => panic!("No intersection found where it was expected."),
        line3d::Intersection::Line(_) => panic!("Whole line found as an intersection where single point was expected."),
        line3d::Intersection::Point(pt) => {
            let ordering = order_planes(&observing_wall.polygon().plane(), &bordering_wall.polygon().plane());
            let (pt1, pt2, pt3, pt4);

            let base_line: Line3D;
            let surface_line: Line3D;

            if ordering > 0 {
                base_line = Line3D::new(Point::cross_prod(&shared_segment_vec, &bor_wall_width_vec), shared_segment.p1().to_owned()).unwrap();
                surface_line = obs_wall_surface_line; 
            } else {
                base_line = Line3D::new(Point::cross_prod(&shared_segment_vec, &obs_wall_width_vec), shared_segment.p1().to_owned()).unwrap();
                surface_line = bor_wall_surface_line;
            }
        
            let base_surface_intersection = line3d::intersection(&base_line, &surface_line);

            match base_surface_intersection {
                line3d::Intersection::None => panic!("No intersection found where it was expected."),
                line3d::Intersection::Line(_) => panic!("Whole line found as an intersection where single point was expected."),
                line3d::Intersection::Point(base_surface_intersection_pt) => {
                    if ordering > 0 {
                        pt1 = shared_segment.p1().clone();
                        pt2 = shared_segment.p2().clone();
                        pt3 = base_surface_intersection_pt.clone();
                        pt4 = base_surface_intersection_pt.clone().add(&shared_segment_vec);
                    } else {
                        pt1 = base_surface_intersection_pt.clone();
                        pt2 = base_surface_intersection_pt.clone().add(&shared_segment_vec);
                        pt3 = pt.clone();
                        pt4 = pt.clone().add(&shared_segment_vec);
                    }
                }
            }
                        
            (pt1, pt2, pt3, pt4)
        }
    }
}

fn order_planes(p1: &Plane, p2: &Plane) -> isize {
    let n1 = p1.normal_vector().normalize();
    let n2 = p2.normal_vector().normalize();

    let n1_num = n1.x * 100. + n1.y * 10. + n1.z;
    let n2_num = n2.x * 100. + n2.y * 10. + n2.z;

    if n1_num > n2_num {
        1
    } else if n1_num < n2_num {
        -1
    } else {
        panic!("Cannot order parallel planes. {:?} {:?}", p1, p2);
    }
}

fn solve_corner_isolated_next_to_non_isolated_wall(shared_segment: &LineSegment, observing_wall: &PolygonWithIsolationDetails, bordering_wall: &PolygonWithIsolationDetails) -> (Point, Point, Point, Point) {
    let obs_wall_iso_w = observing_wall.isolation().as_ref().unwrap().width();

    let pt1 = shared_segment.p1().clone();
    let pt2 = shared_segment.p2().clone();
    let pt3 = pt1.add(&observing_wall.polygon().normal().normalize().multiply(obs_wall_iso_w));
    let pt4 = pt2.add(&observing_wall.polygon().normal().normalize().multiply(obs_wall_iso_w));

    return (pt1, pt2, pt3, pt4);
}

fn get_borders_for_wall(ind: usize, request: &Request) -> Vec<Vec<Border>> {
    let walls = request.data().into_iter().map(|a| a.polygon().clone()).collect::<Vec<_>>();

    let corners = Polygon::get_all_corners_on_polygon(ind, &walls);
    
    corners_to_borders(&corners, &walls[ind])
}

fn corners_to_borders(corners: &Vec<Corner>, wall: &Polygon) -> Vec<Vec<Border>> {
    let mut res = vec![];

    let rl = wall.rim().len();
    let mut i = 0;

    while i < rl {
        let tmp = &wall.rim()[i];
        let next = &wall.rim()[(i+1)%rl];
        let mut corners_on_this_side: Vec<Corner> = corners.into_iter().filter(|corner| corner.ind_of_side_in_this_polygon == i).map(|corner| corner.clone()).collect::<Vec<Corner>>();

        res.push(corners_on_one_side_to_borders(&mut corners_on_this_side, tmp, next));
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

#[derive(Clone, Debug)]
struct Border {
    point_a: Point,
    point_b: Point, 
    wall_ind: Option<usize>
}
