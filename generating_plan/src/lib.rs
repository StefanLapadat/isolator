use crate::request_for_isolation::Request;
use crate::plan_generation::Plan;

pub mod triangulation;
pub mod building_representations;
pub mod request_for_isolation;
pub mod tiling;
pub mod plan_generation;

pub use self::plan_execution::PlanExecution;
mod plan_execution;

pub fn create_plan(request: &Request) -> Plan {
    plan_generation::generate_plan(request)
}
