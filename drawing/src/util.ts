import {Point, Plan, PlanExecution, PlanExecutionEvent, Building, TriangulizedWall, Triangle, TriangulizedTile} from './models';

export function backendPointToBabylonPoint(p: Point): Point {
    return {x: p.x, y: p.z, z: p.y};
}

export function backendPlanExecutionToBabylonPlanExecution(pe: PlanExecution): PlanExecution {
    return {...pe, events: pe.events.map(peev => backendPlanExecutionEventToBabylonPlanExecutionEvent(new PlanExecutionEvent(peev)).field)};
}

export function backendPlanToBabylonPlan(p: Plan): Plan {
    return {...p, planExecution: backendPlanExecutionToBabylonPlanExecution(p.planExecution)};
}

export function backendPlanExecutionEventToBabylonPlanExecutionEvent(peev: PlanExecutionEvent): PlanExecutionEvent {
    if ("Translate" in peev.field) {
        return new PlanExecutionEvent(({
            "Translate": {...peev.field.Translate, start_position: backendPointToBabylonPoint(peev.field.Translate.start_position), end_position: backendPointToBabylonPoint(peev.field.Translate.end_position)}
        }));
    } else if ("Create" in peev.field) {
        return new PlanExecutionEvent(({
            "Create": {...peev.field.Create, 
                position: backendPointToBabylonPoint(peev.field.Create.position), 
                adhesive_tile: backendTriangulizedTileToBabylongTriangulizedTile(peev.field.Create.adhesive_tile),
                styro_tile: backendTriangulizedTileToBabylongTriangulizedTile(peev.field.Create.styro_tile),
            }
        }))
    }
}

export function backendBuildingToBabylonBuilding(building: Building): Building {
    return ({
        walls: building.walls.map(wall => backendWalltoBabylongWall(wall)), 
        wireframe: building.wireframe.map(lineStr => lineStr.map(pt => backendPointToBabylonPoint(pt)))
    });
}

export function backendWalltoBabylongWall(wall: TriangulizedWall): TriangulizedWall {
    return { triangles: wall.triangles.map(t => backendTriangleToBabylonTriangle(t))};
}

export function backendTriangleToBabylonTriangle(t: Triangle): Triangle {
    return {t1: backendPointToBabylonPoint(t.t1), t2: backendPointToBabylonPoint(t.t2), t3: backendPointToBabylonPoint(t.t3)};
}

export function backendTriangulizedTileToBabylongTriangulizedTile(tile: TriangulizedTile): TriangulizedTile {
    return {triangles: tile.triangles.map(t => backendTriangleToBabylonTriangle(t))};
}