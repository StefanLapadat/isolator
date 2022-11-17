use serde::{Serialize, Deserialize};
use crate::general_geometry::{Point, Triangle, Polygon, PolygonPointsOnSides, Simmilar, Polygon2D};
use crate::triangulation::PolygonForTriangulation;

#[derive(Debug, Clone)]
pub struct Tile {
    base_polygon: PolygonPointsOnSides,
    surface_polygon: PolygonPointsOnSides,
}

impl Tile {
    pub fn new(base_polygon: PolygonPointsOnSides, surface_polygon: PolygonPointsOnSides) -> Tile {
        Tile {
            base_polygon, surface_polygon
        }
    }

    fn base_polygon(&self) -> &PolygonPointsOnSides {
        &self.base_polygon
    }

    fn surface_polygon(&self) -> &PolygonPointsOnSides {
        &self.surface_polygon
    }

    fn width(&self) -> f64 {
        let base_data = self.base_polygon().to_polygon();
        let surface_data = self.surface_polygon().to_polygon();

        let p1 = Polygon::new(base_data.0, base_data.1).plane();
        let p2 = Polygon::new(surface_data.0, surface_data.1).plane();

        (p1.d() - p2.d()).abs() / p1.normal_vector().modulo()
    }

    fn width_vec(&self) -> Point {
        let p_args = self.base_polygon().to_polygon();
        Polygon::new(p_args.0, p_args.1).normal().normalize().multiply(self.width())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TriangulizedTile {
    triangles: Vec<Triangle>
}

impl TriangulizedTile {
    fn new(triangles: Vec<Triangle>) -> Self {
        Self {
            triangles
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TriangulizedTiles {
    tiles: Vec<TriangulizedTile>,
    wireframe: Vec<Vec<Point>>
}

impl TriangulizedTiles {
    pub fn from_tiles(tiles: Vec<Tile>) -> TriangulizedTiles {
        let mut triangulized_tiles: Vec<TriangulizedTile> = vec![];
        let mut wireframe: Vec<Vec<Point>> = vec![];

        for tile in tiles {
            let mut triangulized_tile_with_wireframe = tile_to_triangulized_tile(&tile);
            triangulized_tiles.push(triangulized_tile_with_wireframe.0);
            wireframe.append(&mut triangulized_tile_with_wireframe.1);
        }

        TriangulizedTiles {
            tiles: triangulized_tiles,
            wireframe: wireframe
        }
    }
}

fn tile_to_polygons(tile: &Tile) -> Vec<Polygon> {
    let mut res = vec![];

    res.append(&mut parallel_rims_to_polygons(tile.base_polygon().rim(), tile.surface_polygon().rim()));

    let mut i = 0;
    while i < tile.base_polygon().holes().len() {
        res.append(&mut parallel_rims_to_polygons(&tile.base_polygon().holes()[i], &tile.surface_polygon().holes()[i]));
        i+=1;
    }

    res.push(Polygon::from_polygon_points_on_sides(tile.base_polygon().clone()));
    res.push(Polygon::from_polygon_points_on_sides(tile.surface_polygon().clone()));
    
    Polygon::merge_multiple_polygons(&res)
}

fn parallel_rims_to_polygons(base_rim: &Vec<Point>, surface_rim: &Vec<Point>) -> Vec<Polygon> {
    let mut res = vec![];

    let mut i: usize = 0;
    let rl = base_rim.len();

    while i < rl {
        if Point::are_points_simmilar(&base_rim[i], &base_rim[(i+1)%rl]) && Point::are_points_simmilar(&surface_rim[i], &surface_rim[(i+1)%rl]) {
            i+=1; 
            continue;
        }

        if !Point::are_points_simmilar(&base_rim[i], &base_rim[(i+1)%rl]) && !Point::are_points_simmilar(&surface_rim[i], &surface_rim[(i+1)%rl]) {
            res.push(Polygon::new(vec![base_rim[i].clone(), base_rim[(i+1)%rl].clone(), surface_rim[(i+1)%rl].clone(), surface_rim[i].clone()], vec![]));
        }

        if Point::are_points_simmilar(&base_rim[i], &base_rim[(i+1)%rl]) && !Point::are_points_simmilar(&surface_rim[i], &surface_rim[(i+1)%rl]) {
            res.push(Polygon::new(vec![base_rim[i].clone(), surface_rim[(i+1)%rl].clone(), surface_rim[i].clone()], vec![]));
        }

        if !Point::are_points_simmilar(&base_rim[i], &base_rim[(i+1)%rl]) && Point::are_points_simmilar(&surface_rim[i], &surface_rim[(i+1)%rl]) {
            res.push(Polygon::new(vec![base_rim[i].clone(), base_rim[(i+1)%rl].clone(), surface_rim[(i+1)%rl].clone()], vec![]));
        }

        i+=1;
    }

    res
}

fn tile_to_triangulized_tile(tile: &Tile) -> (TriangulizedTile, Vec<Vec<Point>>) {
    let mut triangles: Vec<Triangle> = vec![];
    let mut wireframe: Vec<Vec<Point>> = vec![];

    for side in tile_to_polygons(tile) {
        for triangle in PolygonForTriangulation::from_polygon(&side).triangulate_3d() {
            triangles.push(triangle)
        }

        wireframe.append(&mut side.wireframe());
    }

    (TriangulizedTile::new(triangles), wireframe)
}

pub fn split_into_tiles(tile: &Tile, unit_tile: &UnitTile) -> Option<Vec<Tile>> {
    // So how exactly could I do this?? 
    // And is it sensible to implement this first without glue, and then to somehow try to add glue into the story?? 
    // How do I even specify where should glue be? 
    // I am not sure if it will be that easy to just shove glue into the solution, to hack it.. but anyway, as always, I will 
    // of course not give any though to this problem, but, for the sake of getting the feel for the problem, will just strat hacking stuff. 
    // ..
    // Weeell, this doesn't seem to be that hard?? 
    // First of all I need unit tile to have one dimension equal to tile width? I could start philosophy on how it doesn't have to be like that, but.. 
    // KISS :D 

    // So for first iteration, what do I need? 
    // I should find a bounding box arround baseand surface polygons first, that seems like a reasonable first step :) 
    // After that, what? Fuck it, I need a good way to project stuff.. And I don't have it. I have some hacked stuff, but I should have a good, good 
    // way to project any polygon to any plane.. That should be a must.. And also, to return that projected polygon into original state. 
    // Now, ok, if I had that, what would I do? I guess that I would find the bounding box of both polygons and then a single bounding 
    // box of both polygons together, and in the end, I would need to find an intersection between all the tiles in that bounding box and my original tile. 
    // That's it :) Great! I will complete this tonight. What else? Maybe, smarter way to solve corners! Yep, great as well. 

    let args_base= tile.base_polygon.to_polygon();
    let args_surface = tile.surface_polygon.to_polygon();

    let base = Polygon::new(args_base.0, args_base.1);
    let surface = Polygon::new(args_surface.0, args_surface.1);
    
    let system = base.coordinate_system_xy_parallel_to_self();
    
    let base_2d = base.to_2d(&system);
    let surface_2d = surface.to_2d(&system);

    let base_union_box = Polygon2D::union_box_many(vec![base_2d, surface_2d]);
    let base_union_box_3d = base_union_box.to_3d(&system);
    let surface_union_box = base_union_box_3d.translate(&tile.width_vec());

    let base_comps = base_union_box_3d.destruct_to_components();
    let surface_comps = surface_union_box.destruct_to_components();

    let res_tile = Tile::new(PolygonPointsOnSides::new(base_comps.0, base_comps.1),
                                    PolygonPointsOnSides::new(surface_comps.0, surface_comps.1));

    // Option::Some(vec![tile.clone()])
    Option::Some(vec![res_tile])
}


pub fn are_tile_and_unit_tile_compatible(tile: &Tile, unit_tile: &UnitTile) -> bool {
    let tile_width = tile.width();

    unit_tile.d.x.simmilar_to(tile_width, 0.0001) || 
    unit_tile.d.y.simmilar_to(tile_width, 0.0001) || 
    unit_tile.d.z.simmilar_to(tile_width, 0.0001)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitTile {
    d: Point
}

impl UnitTile {
    pub fn new(d: Point) -> Option<Self> {
        if !d.same_oktant(&Point::new(1., 1., 1.)) || d.close_to_zero() {
            None
        } else {
            Some(UnitTile { d })
        }
    }
}
