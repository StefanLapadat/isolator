use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    // let walls: Vec<RectangleInSpace> = create_walls();

    let objects_to_draw: Vec<DrawableObject> = vec![
        create_simple_polygon2()
    ];

    let mut file = File::create("/home/stefan/Documents/cia/projects/isolator/drawing/public/abc.json")?;

    let serialized_points = serde_json::to_string(&objects_to_draw).unwrap();

    file.write(serialized_points.as_bytes())?;

    Ok(())
}


#[derive(Serialize, Deserialize, Debug)]
enum DrawableObject {
    Point(Point),
    PolygonWithHoles(PolygonWithHoles)
}

struct RectangleInSpace {
    point1: Point,
    point2: Point,
    point3: Point
}

impl RectangleInSpace {
    fn point4(&self) -> Point {
        self.point3.subtract(&self.point2.subtract(&self.point1))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PolygonWithHoles {
    start_point: PolygonWithHolesLine,
    elements: Vec<PolygonWithHolesElement>
}

#[derive(Serialize, Deserialize, Debug)]
enum PolygonWithHolesElement {
    PolygonWithHolesLine(PolygonWithHolesLine),
    PolygonWithHolesArc(PolygonWithHolesArc)
}

#[derive(Serialize, Deserialize, Debug)]
struct PolygonWithHolesLine {
    x: f64,
    y: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct PolygonWithHolesArc {
    mid_x: f64,
    mid_y: f64,
    end_x: f64,
    end_y: f64,
    number_of_segments: i32
}

fn create_walls() -> Vec<RectangleInSpace> {
    vec![
        RectangleInSpace {
            point1: Point::new(0.0, 0.0, 0.0),
            point2: Point::new(100.0, 0.0, 0.0),
            point3: Point::new(100.0, 100.0, 0.0)
        },
        RectangleInSpace {
            point1: Point::new(0.0, 0.0, 0.0),
            point2: Point::new(100.0, 0.0, 0.0),
            point3: Point::new(100.0, 0.0, 100.0)
        },
        RectangleInSpace {
            point1: Point::new(0.0, 0.0, 0.0),
            point2: Point::new(0.0, 0.0, 100.0),
            point3: Point::new(0.0, 100.0, 100.0)
        },
    ]
}

fn create_simple_polygon1() -> DrawableObject {
    DrawableObject::PolygonWithHoles(PolygonWithHoles {
        start_point: PolygonWithHolesLine {
            x: 2.0, 
            y: 0.0
        },
        elements: vec![
            PolygonWithHolesElement::PolygonWithHolesLine(
            PolygonWithHolesLine {
                x: 2.0,
                y: 2.0
            }), 
            PolygonWithHolesElement::PolygonWithHolesLine(
            PolygonWithHolesLine {
                x: -5.0,
                y: 5.0
            }), 
            PolygonWithHolesElement::PolygonWithHolesLine(
            PolygonWithHolesLine {
                x: -3.0,
                y: 1.0
            }),
            PolygonWithHolesElement::PolygonWithHolesLine(
            PolygonWithHolesLine {
                x: -4.0,
                y: -4.0
            }), 
            PolygonWithHolesElement::PolygonWithHolesArc(
            PolygonWithHolesArc {
                mid_x: 0.0,
                mid_y: -2.0,
                end_x: 4.0,
                end_y: -4.0,
                number_of_segments: 100
            }), 
        ]
    })
}

fn create_simple_polygon2() -> DrawableObject {
    DrawableObject::PolygonWithHoles(PolygonWithHoles {
        start_point: PolygonWithHolesLine {
            x: 0.0, 
            y: 0.0
        },
        elements: vec![
            PolygonWithHolesElement::PolygonWithHolesLine(
            PolygonWithHolesLine {
                x: 100.0,
                y: 0.0
            }), 
            PolygonWithHolesElement::PolygonWithHolesLine(
            PolygonWithHolesLine {
                x: 100.0,
                y: 100.0
            }), 
            PolygonWithHolesElement::PolygonWithHolesLine(
            PolygonWithHolesLine {
                x: 0.0,
                y: 100.0
            }),
        ]
    })
}

fn get_points_from_walls(walls: Vec<RectangleInSpace>) -> Vec<DrawableObject> {
    let mut points:Vec<DrawableObject>  = vec![];

    for wall in &walls {
        let sampled_wall = sample(wall);

        for point in sampled_wall {
            points.push(DrawableObject::Point(point));
        }
    }

    points
}

fn sample(rectangle: &RectangleInSpace) -> Vec<Point> {
    let t1 = &rectangle.point1;
    let t2 = &rectangle.point2;
    let t3 = &rectangle.point3;
    let t4 = &rectangle.point4();

    let resolution = 1.0;

    let unit_direction1to2 = get_unit_direction(t1, t2);
    let unit_direction2to3 = get_unit_direction(t2, t3);

    let mut res = vec![];

    let mut temp_out1 = Point::copy_new(t1);
    let mut temp_out2 = Point::copy_new(t2);

    while distance(t1, &temp_out1) < distance(t1, t4) {

        let mut temp_in = Point::copy_new(&temp_out1);

        while distance(&temp_out1, &temp_in) < distance(&temp_out1, &temp_out2) {
            res.push(Point::copy_new(&temp_in));
            temp_in = temp_in.add(&unit_direction1to2.multiply(resolution));
        }

        temp_out1 = temp_out1.add(&unit_direction2to3.multiply(resolution));
        temp_out2 = temp_out2.add(&unit_direction2to3.multiply(resolution));
    }

    res
}

fn get_unit_direction(p1: &Point, p2: &Point) -> Point {
    let direction = p2.subtract(p1);
    direction.divide(get_point_modulo(&direction))
}

fn get_point_modulo(p: &Point) -> f64 {
    (p.x * p.x + p.y * p.y + p.z * p.z).sqrt()
}

fn distance(p1: &Point, p2: &Point) -> f64{
    get_point_modulo(&p2.subtract(p1))
}
