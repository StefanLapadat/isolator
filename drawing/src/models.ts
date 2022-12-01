
export interface Plan {
    building: Building,
    tiles: TilesWithAdhesive,
    planExecution: PlanExecution
}

export interface PlanExecution {
    start: number, 
    end: number,
    events: PlanExecutionEventInner[]
}

export type PlanExecutionEventInner = Translate | Create | Teleport | Fix;
export class PlanExecutionEvent {

    constructor(private fld: PlanExecutionEventInner) {
    }

    get field() {
        return this.fld;
    }

    start() {
        if ("Translate" in this.fld) {
            return this.fld.Translate.start;
        } else if ("Create" in this.fld) {
            return this.fld.Create.start;
        } else if ("Fix" in this.fld) {
            return this.fld.Fix.start;
        } else {
            return this.fld.Teleport.start;
        }
    }

    end() {
        if ("Translate" in this.fld) {
            return this.fld.Translate.end;
        } else if ("Create" in this.fld) {
            return this.fld.Create.end;
        } else if ("Fix" in this.fld) {
            return this.fld.Fix.end;
        } else {
            return this.fld.Teleport.end;
        }
    }
}

export interface Translate {
    Translate: {
        tile_id: String,
        start: number,
        end: number,
        start_position: Point,
        end_position: Point
    }
}

export interface Create {
    Create: {
        tile_id: String,
        start: number, 
        end: number,
        position: Point,
        styro_tile: TriangulizedTile,
        adhesive_tile: TriangulizedTile
    }
}

export interface Teleport {
    Teleport: {
        tile_id: String,
        start: number, 
        end: number,
        end_position: Point
    }
}

export interface Fix {
    Fix: {
        tile_id: String,
        start: number,
        end: number,
        end_position: Point,
    }
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

