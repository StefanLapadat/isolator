use general_geometry::{Polygon};
use generating_plan::building_representations::{polygon_walls::PolygonWalls, levels::Levels, levels::Level, converters};
use generating_plan::{request_for_isolation::HookSystem};

pub(crate) fn create_building_polygon_walls() -> PolygonWalls {
    let right0 = (5.0, 0.0);
    let up0 = (0.0, 5.0);
    let down0 = (0., -5.);
    let left0 = (-5., 0.);
    let ld = (-5.0, -5.0);
    let rd = (5.0, -5.0);

    let levels = Levels::new(vec![
        Level::new(7., Polygon::in_xy_plane_no_holes_from_increments((-10., -10.), 
        vec![right0, up0, right0, down0, right0, up0, right0, down0, right0, up0, up0, up0, up0, left0, down0, left0, up0, left0, down0, left0, up0, left0, ld, down0 ])),
        Level::new(5., Polygon::in_xy_plane_no_holes_from_increments((0., -10.), vec![right0, up0, right0, down0, right0, up0, right0, down0, right0, up0, up0, up0, up0, left0, down0, left0, up0, left0, down0, left0, up0, left0, ld, down0 ])),
    ]);

    converters::levels_to_polygon_walls(levels)
}

pub(crate) fn hooks() -> Vec<HookSystem> {
    vec![]
}
