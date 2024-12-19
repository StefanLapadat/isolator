use general_geometry::{Point, Polygon};
use generating_plan::building_representations::{polygon_walls::PolygonWalls, levels::Levels, levels::Level, converters};
use generating_plan::request_for_isolation::HookSystem;

pub(crate) fn create_building_polygon_walls() -> PolygonWalls {
    converters::levels_to_polygon_walls(create_building_levels())
}

fn create_building_levels() -> Levels {
    let r: Point = Point { x: 1., y: 0., z: 0.};
    let u: Point = Point { x: 0., y: 1., z: 0.};
    let l: Point = Point { x: -1., y: 0., z: 0.};
    let d: Point = Point { x: 0., y: -1., z: 0.};

    let num_levels = 10;
    let base_start_coord = 2. * (num_levels as f64);
    let res = Levels::new((0..num_levels).map(|e| {e as f64}).map(|e| {
            Level::new(1., Polygon::in_xy_plane_no_holes_from_increments_points(
                (-(10. - e), -(10. - e)), 
                vec![
                    r.multiply(base_start_coord - 2.*e),
                    u.multiply(base_start_coord - 2.*e),
                    l.multiply(base_start_coord - 2.*e),
                    d.multiply(base_start_coord - 2.*e)]
            ))
        }).collect()
    );

    res
}

pub(crate) fn hooks() -> Vec<HookSystem> {
    vec![]
}
