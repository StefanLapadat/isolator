use serde::{Serialize, Deserialize};
use crate::request_for_isolation::HookPair;
use crate::tiling::{TileWithAdhesive,};
use crate::building_representations::polygon_walls::PolygonWalls;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanExecution {
    events: Vec<PlanExecutionEvent>
}

impl PlanExecution {
    pub fn new(events: Vec<PlanExecutionEvent>) -> Self {
        Self {events}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanExecutionEvent {
    start: u32,
    end: u32
}

impl PlanExecutionEvent {
    pub fn new(start: u32, end: u32) -> Self {
        Self {start, end}
    }
}

pub struct PlanExecutionCreator {

}

impl PlanExecutionCreator {
    pub fn create_plan(building: &PolygonWalls, tiles: &Vec<TileWithAdhesive>, hooks: &Vec<HookPair>) -> PlanExecution {
        let mut events: Vec<PlanExecutionEvent> = vec![];
        
        PlanExecution { events }
    }

    pub fn split_tiles_on_hooks(tiles: &Vec<TileWithAdhesive>, hooks: &Vec<HookPair>) -> Vec<Vec<usize>> {
        vec![]
    }
}
