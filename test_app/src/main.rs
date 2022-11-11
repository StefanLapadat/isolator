use geo_types::{Coordinate, LineString, Polygon};
use geo_clipper::Clipper;

fn main(){
let subject = Polygon::new(
    LineString(vec![
        Coordinate { x: -831530082855924.5,
            y: -555479721777149.6},
        Coordinate { x: -415765041427962.25,
            y: -277739860888574.8 },
        Coordinate {  x: -804600846671967.,
            y: 304331197110572.3 },
        Coordinate {  x: -1220365888099929.2,
            y: 26591336221997.52 },
    ]),
    vec![],
);

let clip = Polygon::new(
    LineString(vec![
        Coordinate { x: -1220365888099929.2,
            y: 26591336221997.52},
        Coordinate {  x: -388835805244004.67,
            y: 582071057999147.1 },
        Coordinate {x: -666575666132579.4,
            y: 997836099427109.2 },
        Coordinate {x: -1498105748988503.9,
            y: 442356377649959.7 },
    ]),
    vec![],
);

let result = subject.union(&clip, 1.0);
// let r = result.into_iter().next().unwrap();

println!("{:?}", result);
}