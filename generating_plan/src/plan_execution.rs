use serde::{Serialize, Deserialize};

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
