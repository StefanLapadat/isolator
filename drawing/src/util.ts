import {Point, Plan, PlanExecution, PlanExecutionEvent, Building, TriangulizedWall, Triangle} from './models';

export function backendPointToBabylonPoint(p: Point): Point {
    return {x: p.x, y: p.z, z: p.y};
}

export function backendPlanToBabylonPlan(p: Plan): Plan {
    return {...p, planExecution: backendPlanExecutionToBabylonPlanExecution(p.planExecution)};
}

export function backendPlanExecutionToBabylonPlanExecution(pe: PlanExecution): PlanExecution {
    return {...pe, events: pe.events.map(peev => backendPlanExecutionEventToBabylonPlanExecutionEvent(peev))};
}

export function backendPlanExecutionEventToBabylonPlanExecutionEvent(peev: PlanExecutionEvent): PlanExecutionEvent {
    return {...peev, start_position: backendPointToBabylonPoint(peev.start_position), end_position: backendPointToBabylonPoint(peev.end_position)};
}

export function backendBuildingToBabylonBuilding(building: Building): Building {
    return ({
        walls: building.walls.map(wall => backendWalltoBabylongWall(wall)), 
        wireframe: building.wireframe.map(lineStr => lineStr.map(pt => backendPointToBabylonPoint(pt)))
    });
}

export function backendWalltoBabylongWall(wall: TriangulizedWall): TriangulizedWall {
    return { triangles: wall.triangles.map(t => backendTriangleToBabylongTriangle(t))};
}

export function backendTriangleToBabylongTriangle(t: Triangle): Triangle {
    return {t1: backendPointToBabylonPoint(t.t1), t2: backendPointToBabylonPoint(t.t2), t3: backendPointToBabylonPoint(t.t3)};
}