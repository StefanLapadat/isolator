use general_geometry::{LineSegment, Point};
use serde::{Serialize, Deserialize};
use crate::request_for_isolation::HookSystem;
use crate::tiling::{TileWithAdhesive, TriangulizedTile, tile::tile_to_triangulized_tile};
use crate::building_representations::polygon_walls::PolygonWalls;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanExecution {
    start: usize,
    end: usize,
    events: Vec<PlanExecutionEvent>
}

impl PlanExecution {
    pub fn new(events: Vec<PlanExecutionEvent>, start: usize, end: usize) -> Self {
        Self {events, start, end}
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn events(&self) -> &Vec<PlanExecutionEvent> {
        &self.events
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateExecutionEventData {
    tile_id: String,
    start: usize,
    end: usize,
    styro_start_position: Point,
    styro_end_position: Point,
    adhesive_start_position: Point,
    adhesive_end_position: Point
}

impl TranslateExecutionEventData {
    pub fn new(tile_id: String, start: usize, end: usize, styro_start_position: Point, styro_end_position: Point, adhesive_start_position: Point, adhesive_end_position: Point) -> Self {
        Self {tile_id, start, end, styro_end_position, styro_start_position, adhesive_end_position, adhesive_start_position}
    }

    pub fn duration(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateExecutionEventData {
    tile_id: String,
    start: usize,
    end: usize,
    styro_position: Point,
    adhesive_position: Point,
    styro_tile: TriangulizedTile,
    adhesive_tile: TriangulizedTile
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeleportExecutionEventData {
    tile_id: String,
    start: usize,
    end: usize,
    end_position: Point,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FixExecutionEventData {
    tile_id: String,
    start: usize,
    end: usize,
    end_position: Point,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum PlanExecutionEvent {
    Create(CreateExecutionEventData),
    Translate(TranslateExecutionEventData),
    Teleport(TeleportExecutionEventData),
    Fix(FixExecutionEventData)
}

impl PlanExecutionEvent {
    pub fn start(&self) -> usize {
        match self {
            PlanExecutionEvent::Create(data) => data.start,
            PlanExecutionEvent::Translate(data) => data.start,
            PlanExecutionEvent::Teleport(data) => data.start,
            PlanExecutionEvent::Fix(data) => data.start,
        }
    }

    pub fn end(&self) -> usize {
        match self {
            PlanExecutionEvent::Create(data) => data.end,
            PlanExecutionEvent::Translate(data) => data.end,
            PlanExecutionEvent::Teleport(data) => data.end,
            PlanExecutionEvent::Fix(data) => data.end,
        }
    }
}

pub struct PlanExecutionCreator {
    velocity: f64,
}

impl PlanExecutionCreator {
    pub fn new(velocity: f64) -> Self {
        Self { velocity }
    }

    pub fn create_plan(&self, building: &PolygonWalls, tiles: &Vec<TileWithAdhesive>, hooks: &Vec<HookSystem>, max_distance: f64, number_of_systems: usize) -> PlanExecution {
        let tile_groups = Self::split_tiles_on_hooks(tiles, hooks, max_distance);

        self.create_plan_from_tile_groups(building, tiles, hooks, number_of_systems, &tile_groups)
    }

    fn create_plan_from_tile_groups(&self, building: &PolygonWalls, tiles: &Vec<TileWithAdhesive>, hooks: &Vec<HookSystem>, number_of_systems: usize, tile_groups: &Vec<(usize, Vec<usize>)>) -> PlanExecution {
        let mut events: Vec<PlanExecutionEvent> = vec![];
        let mut start: usize = 0;

        for group in tile_groups {
            let mut tile_group_plan = self.create_plan_from_tile_group(building, tiles, &group.1, start, &hooks[group.0]);
            start = tile_group_plan.end;
            events.append(&mut tile_group_plan.events);
        }

        let end: usize = if events.len() > 0 { events.iter().max_by(|a, b| a.end().cmp(&b.end())).unwrap().end() } else { start };

        PlanExecution { events, start, end }
    }

    fn create_plan_from_tile_group(&self, building: &PolygonWalls, tiles: &Vec<TileWithAdhesive>, group: &Vec<usize>, start: usize, hook_system: &HookSystem) -> PlanExecution {
        let mut events: Vec<PlanExecutionEvent> = vec![];
        let mut start = start;

        for tile in group {
            let mut tile_plan = self.create_plan_for_tile(building, *tile, &tiles[*tile], start, hook_system);
            events.append(&mut tile_plan.events);
            start = tile_plan.end;
        }

        let start: usize = 0;
        let end: usize = if events.len() > 0 { events.iter().max_by(|a, b| a.end().cmp(&b.end())).unwrap().end() } else { start };

        PlanExecution { events, start, end }
    }

    fn create_plan_for_tile(&self, building: &PolygonWalls, tile_ind: usize, tile: &TileWithAdhesive, start: usize, hook_system: &HookSystem) -> PlanExecution {
        let mut events: Vec<PlanExecutionEvent> = vec![];

        let distance = Self::distance_between_base_of_hook_system_and_tile_end_position(hook_system, tile);
        let duration = (distance / self.velocity) as usize;
        let end: usize = start + duration;
        let id = Uuid::new_v4().to_string();

        let avg = tile.average_point();
        // let avg_styro = tile.styro_tile().average_point();
        // let avg_adhesive = tile.adhesive_tile().average_point();

        // let tile_translated_to_origin_styro = tile.translate(&avg_styro.multiply(-1.));
        // let tile_translated_to_origin_adhesive = tile.translate(&avg_adhesive.multiply(-1.));
        let tile_translated = tile.translate(&avg.multiply(-1.));

        events.push(PlanExecutionEvent::Create(CreateExecutionEventData { 
                tile_id: id.to_owned(), 
                start, 
                end, 
                styro_position: hook_system.carrier_hook_ground().to_owned(),
                adhesive_position: hook_system.carrier_hook_ground().to_owned(),
                styro_tile: tile_to_triangulized_tile(tile_translated.styro_tile()).0,
                adhesive_tile: tile_to_triangulized_tile(tile_translated.adhesive_tile()).0,
            })
        );

        events.push(PlanExecutionEvent::Translate(TranslateExecutionEventData { 
                tile_id: id.to_owned(), 
                start, 
                end, 
                styro_start_position: hook_system.carrier_hook_ground().to_owned(), 
                styro_end_position: tile.average_point().to_owned(), 
                adhesive_start_position: hook_system.carrier_hook_ground().to_owned(), 
                adhesive_end_position: tile.average_point().to_owned(), 
            })
        );

        PlanExecution { events, start, end }
    }

    fn distance_between_base_of_hook_system_and_tile_end_position(hook_system: &HookSystem, tile: &TileWithAdhesive) -> f64 {
        let p1 = hook_system.carrier_hook_ground();
        let p2 = tile.average_point();

        p1.subtract(&p2).modulo()
    }

    fn split_tiles_on_hooks(tiles: &Vec<TileWithAdhesive>, hooks: &Vec<HookSystem>, max_distance: f64) -> Vec<(usize, Vec<usize>)> {
        let mut res = vec![];
        for (hooks_index, hook_system) in hooks.iter().enumerate() {
            let mut inner_res = vec![];

            for (tile_index, tile) in tiles.iter().enumerate() {
                if Self::tile_can_be_set_from_hook_system(tile, hook_system, max_distance) {
                    inner_res.push(tile_index);
                }
            }
            res.push((hooks_index, inner_res));
        }

        Self::remove_duplicate_tiles(res)
    }

    fn remove_duplicate_tiles(tile_groups: Vec<(usize, Vec<usize>)>) -> Vec<(usize, Vec<usize>)> {
        let mut res = vec![];

        for group in tile_groups.iter() {
            let mut res_group: Vec<usize> = vec![];

            for tile_ind in group.1.iter() {
                if !Self::groups_contain_tile(&res, *tile_ind) {
                    res_group.push(tile_ind.to_owned());
                }
            }

            res.push((group.0, res_group));
        }

        res
    }

    fn groups_contain_tile(tile_groups: &Vec<(usize, Vec<usize>)>, tile_ind: usize) -> bool {
        for group in tile_groups {
            for tile in group.1.iter() {
                if *tile == tile_ind {
                    return true;
                }
            }
        }

        false 
    }

    fn tile_can_be_set_from_hook_system(tile: &TileWithAdhesive, hook_system: &HookSystem, max_distance: f64) -> bool {
        let base_end_points = tile.base_rim();
        let surface_end_points = tile.surface_rim();

        let robot_segment = LineSegment::new(hook_system.robot_hook_ground().to_owned(), hook_system.robot_hook().to_owned());

        for pt in base_end_points {
            if robot_segment.distance_from_point(pt) > max_distance {
                return false;
            }
        }

        for pt in surface_end_points {
            if robot_segment.distance_from_point(pt) > max_distance {
                return false;
            }
        }

        true
    }
}
