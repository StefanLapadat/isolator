use general_geometry::{Point, Polygon, Triangle, Plane};
use earcutr::earcut;

struct PolygonForTriangulation<'a> { 
    points: Vec<&'a Point>,
    holes: Vec<usize>
}

impl PolygonForTriangulation<'_> {
    pub fn from_polygon(polygon: &Polygon) -> PolygonForTriangulation{
        PolygonForTriangulation {
            holes: PolygonForTriangulation::indices_of_holes_in_merged_points_and_holes(polygon),
            points: PolygonForTriangulation::merge_points_and_holes(polygon)
        }
    }

    pub fn triangulate_3d(&self) -> Vec<Triangle> {
        let tri = self.triangulate_3d_indices_result();
            
        let mut triangles: Vec<Triangle> = vec![];
        
        let mut i = 0;
        while i<tri.len() {
            let pts = &self.points;
            triangles.push(Triangle::new(&pts[tri[i]], &pts[tri[i+1]], &pts[tri[i+2]]));
            i += 3;
        }

        triangles
    }

    fn indices_of_holes_in_merged_points_and_holes(polygon: &Polygon) -> Vec<usize> {
        let mut res = vec![];
    
        let mut acc: usize = polygon.rim().len();
    
        for hole in polygon.holes() {
            res.push(acc);
            acc+=hole.len();
        }
    
        res
    }

    fn merge_points_and_holes(polygon: &Polygon) -> Vec<&Point> {
        let mut res: Vec<&Point> = vec![];
    
        for point in polygon.rim() {
            res.push(point);
        }
    
        for hole in polygon.holes() {
            for hole_point in hole {
                res.push(hole_point);
            }
        }

        res
    }

    fn triangulate_3d_indices_result(&self) -> Vec<usize> {
        let plane = Plane::from_points_references_vector(&self.points).unwrap();
        let system = plane.coordinate_system_normal_to_plane_origin_at_base();

        return earcut(&Polygon::flatten_points(&self.points, &system), &self.holes, 2);
    }
}


pub fn triangulate_polygon(poly: &Polygon) -> Vec<Triangle> {
    PolygonForTriangulation::from_polygon(poly).triangulate_3d()
}
