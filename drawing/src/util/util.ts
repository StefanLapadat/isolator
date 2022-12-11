import { Plan, ShowBuildingOrIsolation } from "../models";
import * as BABYLON from '@babylonjs/core';
import { BuildingWithVisibility } from "../buildingWithVisibility";

export function backendPlanToBabylonPlan(plan: Plan): Plan {
    return backendObjToBabylonObj(plan) as Plan;
}

function backendObjToBabylonObj(obj: any): any {
    let res: any;

    if (Array.isArray(obj)){
        res = [...obj];
    } else if (typeof obj === 'object') {
        res = {...obj};
    } else {
        res = obj;
    }

    if (objectIsPoint(obj)) {
        res.x = obj.x;
        res.y = obj.z;
        res.z = obj.y;
    } else {
        if (typeof obj !== 'string') {
            for (var prop in res) {
                if (Object.prototype.hasOwnProperty.call(obj, prop)) {
                    res[prop] = backendObjToBabylonObj(obj[prop]);
                }
            }
        }
    }

    return res;
}

function objectIsPoint(obj: any) {
    let buff = Object.keys(obj);
    return buff.includes('x') && buff.includes('y') && buff.includes('z') && buff.length === 3;
}

export function allInputFieldsCorrect() {
    return Number.isInteger(getRequestId()) && typeof getTileLength() === "number" && getTileLength() > 0 && 
    typeof getTileHeight() === "number" && getTileHeight() > 0 && typeof getTileWidth() === "number" && getTileWidth() > 0 && 
    typeof getVelocity() === "number" && getVelocity() > 0;
}

export function getRequestId(): number {
    return parseInt((document.getElementById('request-id') as any)?.value ?? localStorage.getItem('requestId') ?? '1');
}

export function getTileLength(): number {
    return parseFloat((document.getElementById('tile-length') as any)?.value ?? localStorage.getItem('tileLength') ?? '5');
}

export function getTileHeight(): number {
    return parseFloat((document.getElementById('tile-height') as any)?.value ?? localStorage.getItem('tileHeight') ?? '2.5');
}

export function getTileWidth(): number {
    return parseFloat((document.getElementById('tile-width') as any)?.value ?? localStorage.getItem('tileWidth') ?? '0.3');
}

export function getVelocity(): number {
    return parseFloat((document.getElementById('tile-setting-velocity') as any)?.value ?? localStorage.getItem('tileSettingVelocity') ?? '0.001');
}

export function getShowBuilding(): ShowBuildingOrIsolation {
    return parseInt((document.getElementById('building') as any)?.value ?? localStorage.getItem('building') ?? '1');
}

export function getShowIsolation(): ShowBuildingOrIsolation {
    return parseInt((document.getElementById('isolation') as any)?.value ?? localStorage.getItem('isolation') ?? '1');
}

export function getShowAxes(): boolean {
    return document.querySelector('#show-axes' as any).checked;
}

export function getCanvas(): HTMLCanvasElement {
    return document.getElementById("canvas") as HTMLCanvasElement;
}

export function showAxis(size: number, scene: BABYLON.Scene) {
    var makeTextPlane = function(text: any, color: any, size: any) {
        var dynamicTexture = new BABYLON.DynamicTexture("DynamicTexture", 50, scene, true);
        dynamicTexture.hasAlpha = true;
        dynamicTexture.drawText(text, 5, 40, "bold 36px Arial", color , "transparent", true);
        var plane = BABYLON.Mesh.CreatePlane("TextPlane", size, scene, true);
        plane.material = new BABYLON.StandardMaterial("TextPlaneMaterial", scene);
        plane.material.backFaceCulling = false;
        // plane.material.specularColor = new BABYLON.Color3(0, 0, 0);
        // plane.material.diffuseTexture = dynamicTexture;
        return plane;
    };

    var axisX = BABYLON.Mesh.CreateLines("axisX", [ 
        BABYLON.Vector3.Zero(), new BABYLON.Vector3(size, 0, 0), new BABYLON.Vector3(size * 0.95, 0.05 * size, 0), 
        new BABYLON.Vector3(size, 0, 0), new BABYLON.Vector3(size * 0.95, -0.05 * size, 0)
    ], scene, true);
    axisX.color = new BABYLON.Color3(1, 0, 0);
    // var xChar = makeTextPlane("X", "red", size / 10);
    // xChar.position = new BABYLON.Vector3(0.9 * size, -0.05 * size, 0);
    var axisY = BABYLON.Mesh.CreateLines("axisY", [
        BABYLON.Vector3.Zero(), new BABYLON.Vector3(0, size, 0), new BABYLON.Vector3( -0.05 * size, size * 0.95, 0), 
        new BABYLON.Vector3(0, size, 0), new BABYLON.Vector3( 0.05 * size, size * 0.95, 0)
    ], scene, true);
    axisY.color = new BABYLON.Color3(0, 1, 0);
    // var yChar = makeTextPlane("Y", "green", size / 10);
    // yChar.position = new BABYLON.Vector3(0, 0.9 * size, -0.05 * size);
    var axisZ = BABYLON.Mesh.CreateLines("axisZ", [
        new BABYLON.Vector3(0, 0, 0), new BABYLON.Vector3(0, 0, size), new BABYLON.Vector3( 0 , -0.05 * size, size * 0.95),
        new BABYLON.Vector3(0, 0, size), new BABYLON.Vector3( 0, 0.05 * size, size * 0.95)
    ], scene, true);
    axisZ.color = new BABYLON.Color3(0, 0, 1);
    // var zChar = makeTextPlane("Z", "blue", size / 10);
    // zChar.position = new BABYLON.Vector3(0, 0.05 * size, 0.9 * size);
}

export function showGround() {
    BABYLON.MeshBuilder.CreateGround("ground", {height: 100, width: 100, subdivisions: 4});
}

export function populateFieldsFromLocalStorage() {
    (document.getElementById('request-id') as any).value = localStorage.getItem('requestId') ?? '0';
    (document.getElementById('mode') as any).value = localStorage.getItem('mode') ?? '0';
    (document.getElementById('tile-length') as any).value = localStorage.getItem('tileLength') ?? '3';
    (document.getElementById('tile-height') as any).value = localStorage.getItem('tileHeight') ?? '2';
    (document.getElementById('tile-width') as any).value = localStorage.getItem('tileWidth') ?? '0.3';
    document.querySelector('#show-axes' as any).checked = localStorage.getItem('showAxes') === 'true';
    (document.getElementById('tile-setting-velocity') as any).value = localStorage.getItem('tileSettingVelocity') ?? '0.1';
    (document.getElementById('building') as any).value = localStorage.getItem('building') ?? '0';
    (document.getElementById('isolation') as any).value = localStorage.getItem('isolation') ?? '0';
}

export function addEventListeners(reloadApp: any, building: BuildingWithVisibility) {
    document.getElementById("request-id")?.addEventListener('input', (event) => {
        if(allInputFieldsCorrect()){
            localStorage.setItem("requestId", getRequestId().toString());
            reloadApp();
        }
    })
    
    document.getElementById("tile-length")?.addEventListener('input', (event) => {
        if(allInputFieldsCorrect()){
            localStorage.setItem("tileLength", getTileLength().toString());
            reloadApp();
        }
    })
    
    document.getElementById("tile-height")?.addEventListener('input', (event) => {
        if(allInputFieldsCorrect()){
            localStorage.setItem("tileHeight", getTileHeight().toString());
            reloadApp();
        }
    })
    
    document.getElementById("tile-width")?.addEventListener('input', (event) => {
        if(allInputFieldsCorrect()){
            localStorage.setItem("tileWidth", getTileWidth().toString());
            reloadApp();
        }
    })
    
    document.getElementById("show-axes")?.addEventListener('input', (event) => {
        if(allInputFieldsCorrect()){
            localStorage.setItem("showAxes", document.querySelector('#show-axes' as any).checked.toString());                
            reloadApp();
        }
    })

    document.getElementById("tile-setting-velocity")?.addEventListener('input', (event) => {
        if(allInputFieldsCorrect()){
            localStorage.setItem("tileSettingVelocity", getVelocity().toString());
            reloadApp();
        }
    })
    
    document.getElementById("building")?.addEventListener('input', (event) => {
        if(allInputFieldsCorrect()){
            localStorage.setItem("building", getShowBuilding().toString());
            building.buildingMeshSetVisibility();
        }
    })

    document.getElementById("isolation")?.addEventListener('input', (event) => {
        localStorage.setItem("isolation", getShowIsolation().toString());
        building.isolationMeshSetVisibility();
    })
}