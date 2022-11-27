
export interface Plan {
    building: Building,
    tiles: TilesWithAdhesive,
    planExecution: PlanExecution
}

export interface PlanExecution {
    events: PlanExecutionEvent[]
}

export interface PlanExecutionEvent {
    start: number,
    end: number
}

export interface Building {
    walls: TriangulizedWall[],
    wireframe: Point [][] 
}

export interface TilesWithAdhesive {
    triangulized_tiles: Tiles,
    triangulized_adhesive: Tiles
}

export interface Tiles {
    tiles: TriangulizedTile[],
    wireframe: Point [][] 
}

export interface TriangulizedWall {
    triangles: Triangle[]
}

export interface TriangulizedTile {
    triangles: Triangle[]
}

export interface Point {
    x: number, 
    y: number,
    z: number
}

export interface Triangle {
    t1: Point,
    t2: Point,
    t3: Point
}

export enum ShowBuildingOrIsolation {
    Show, 
    Wireframe,
    Hide
}

