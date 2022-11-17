use crate::request_for_isolation::Request;
use crate::plan_generation::Plan;

pub mod triangulation;
pub mod general_geometry;
pub mod building_representations;
pub mod request_for_isolation;
pub mod tiling;
pub mod plan_generation;

pub fn create_plan<F>(request: &Request, alert: F) -> Plan where F: Fn(String) -> () {
    alert(String::from("Pera kojot suhi genije"));
    plan_generation::generate_plan(request)
}
