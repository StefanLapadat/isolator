import * as BABYLON from '@babylonjs/core';
import { FreeCamera } from '@babylonjs/core';
import {Create, Fix, Plan, PlanExecutionEvent, Point, ShowBuildingOrIsolation, Teleport, Translate, } from '../models';
import {HttpBackend} from '../backendService';
import {addEventListeners, allInputFieldsCorrect, backendPlanToBabylonPlan, getCanvas, getRequestId, getShowAxes, getShowBuilding, getTileHeight, getTileLength, getTileWidth, getVelocity, populateFieldsFromLocalStorage, showAxis} from '../util/util';
import { BuildingWithVisibility } from '../buildingWithVisibility';

export function reloadApp() {
    let camera = ((window as any).babylonApp as any)?.getCamera();
    ((window as any).babylonApp as ShowPlanExecution)?.dispose();
    (window as any).babylonApp = new ShowPlanExecution(camera);
    if (!(window as any).eventListenersForPlanExecutionSet) {
        addEventListeners(reloadApp, ((window as any).babylonApp as ShowPlanExecution));
        (window as any).eventListenersForPlanExecutionSet = true;
    }
}

class ShowPlanExecution implements BuildingWithVisibility {
    private plan: Plan;
    private readonly canvas: HTMLCanvasElement;
    private readonly engine: BABYLON.Engine;
    private scene: BABYLON.Scene;

    private buildingMeshVertexData: BABYLON.VertexData;
    private buildingMesh: BABYLON.Mesh;
    private buildingWireframeData: BABYLON.Vector3[][];
    private buildingWireframeMesh: BABYLON.LinesMesh;

    private styroUnionMesh: BABYLON.Mesh;
    private adhesiveUnionMesh: BABYLON.Mesh;

    private styroMat: BABYLON.StandardMaterial;
    private adhesiveMat: BABYLON.StandardMaterial;

    private meshMap: Map<String, {adhesiveMesh: BABYLON.Mesh, styroMesh: BABYLON.Mesh, }> = new Map();
    
    constructor(camera?: {position: {x: number, y: number, z: number}, target: {x: number, y: number, z: number}}) {
        populateFieldsFromLocalStorage();
        this.canvas = getCanvas();
        this.engine = new BABYLON.Engine(this.canvas, true);
        this.scene = this.createScene();

        this.initAnimationMeshes();

        new HttpBackend().get_plan(getRequestId(), getTileLength(), getTileHeight(), getTileWidth(), getVelocity())
        .then((response) => response.json())
        .then((data) => {
            this.plan = backendPlanToBabylonPlan(data as Plan);
            
            this.buildingMeshVertexData = this.getBuildingMeshVertexData();
            this.buildingWireframeData = this.getBuildingWireframeData();

            this.connectCamera(camera);
            this.connectLights();
            this.showBuilding();

            this.scheduleAnimations();

            if (getShowAxes()) {
                showAxis(50, this.scene);
            }

            this.initGeneralGameStuff();
        });
    }
    isolationMeshSetVisibility(): void {
        throw new Error('Method not implemented.');
    }

    initAnimationMeshes() {
        this.styroMat = new BABYLON.StandardMaterial("mat", this.scene);
        this.styroMat.backFaceCulling = false;
        this.styroMat.diffuseColor = BABYLON.Color3.White();

        this.adhesiveMat = new BABYLON.StandardMaterial("mat", this.scene);
        this.adhesiveMat.backFaceCulling = false;
        this.adhesiveMat.diffuseColor = BABYLON.Color3.Green();
        

        this.styroUnionMesh = new BABYLON.Mesh("custom", this.scene);
        
        var positions = [-0.05, 0.02, -0.03, -0.07, -0.02, -0.03, -0.03, -0.02, -0.03,];
        var indices = [0, 1, 2];
        
        var vertexData = new BABYLON.VertexData();

        vertexData.positions = positions;
        vertexData.indices = indices;


        vertexData.applyToMesh(this.styroUnionMesh);

        var positions = [-0.05, 0.02, -0.03, -0.07, -0.02, -0.03, -0.03, -0.02, -0.03,];
        indices = [0, 1, 2];
        
        vertexData = new BABYLON.VertexData();

        vertexData.positions = positions;
        vertexData.indices = indices;

        this.adhesiveUnionMesh = new BABYLON.Mesh("custom", this.scene);

        vertexData.applyToMesh(this.adhesiveUnionMesh);
 
        this.styroUnionMesh.material = this.styroMat;
        this.adhesiveUnionMesh.material = this.adhesiveMat;

    }

    scheduleAnimations() {
        for (let ev of this.plan.planExecution.events.map(a => new PlanExecutionEvent(a))) {
            this.scheduleAnimation(ev);
        }
    }

    scheduleAnimation(event: PlanExecutionEvent) {
        setTimeout(() => {
            this.showAnimation(event);
        }, event.start());
    }

    showAnimation(event: PlanExecutionEvent) {
        if ("Translate" in event.field) {
            this.showTranslateAnimation(event.field);
        } else if ("Create" in event.field) {
            this.showCreateAnimation(event.field);
        } else if ("Fix" in event.field) {
            this.showFixAnimation(event.field);
        } else {
            this.showTeleportAnimation(event.field);
        }
    }

    showTranslateAnimation(event: Translate) {
        let tileMesh = this.meshMap.get(event.Translate.tile_id);

        let adhesiveStartP = event.Translate.adhesive_start_position;
        tileMesh.adhesiveMesh.position = new BABYLON.Vector3(adhesiveStartP.x, adhesiveStartP.y, adhesiveStartP.z);

        let styroStartP = add_to_point(adhesiveStartP, {x: 0.5, y: 0.5, z: 0.5});
        tileMesh.styroMesh.position = new BABYLON.Vector3(styroStartP.x, styroStartP.y, styroStartP.z);
        
        const frameRate = 10;
        const duration = event.Translate.end - event.Translate.start;

        let adhesiveAnimations = this.createTranslationAnimation(event.Translate.adhesive_start_position, event.Translate.adhesive_end_position, duration, frameRate);
        let styroAnimations = this.createTranslationAnimation(event.Translate.styro_start_position, event.Translate.styro_end_position, duration, frameRate);

        this.scene.beginDirectAnimation(tileMesh.adhesiveMesh, adhesiveAnimations, 0, duration / 1000 * frameRate, false, 1, () => {
            this.adhesiveUnionMesh = BABYLON.Mesh.MergeMeshes([this.adhesiveUnionMesh, tileMesh.adhesiveMesh], true);
        });

        this.scene.beginDirectAnimation(tileMesh.styroMesh, styroAnimations, 0, duration / 1000 * frameRate, false, 1, () => {
            this.styroUnionMesh = BABYLON.Mesh.MergeMeshes([this.styroUnionMesh, tileMesh.styroMesh], true);
        });
    }

    showCreateAnimation(event: Create) {  
        var vertexData = new BABYLON.VertexData();

        let totalTriangles = [];
        let indices = [];
        let i = 0;

        for(let wt of event.Create.adhesive_tile.triangles){
            totalTriangles.push(...[wt.t1.x, wt.t1.y, wt.t1.z, wt.t2.x, wt.t2.y, wt.t2.z, wt.t3.x, wt.t3.y, wt.t3.z]);
            indices.push(...[i++, i++, i++]);
        }
    
        vertexData.positions = totalTriangles;
        vertexData.indices = indices;

        let adhesiveMesh = new BABYLON.Mesh("adhesiveMesh", this.scene);
        vertexData.applyToMesh(adhesiveMesh);

        adhesiveMesh.material = this.adhesiveMat;

        let createAt = event.Create.adhesive_position;
        adhesiveMesh.position = new BABYLON.Vector3(createAt.x, createAt.y, createAt.z);
        
        vertexData = new BABYLON.VertexData();

        totalTriangles = [];
        indices = [];
        i = 0;

        for(let wt of event.Create.styro_tile.triangles){
            totalTriangles.push(...[wt.t1.x, wt.t1.y, wt.t1.z, wt.t2.x, wt.t2.y, wt.t2.z, wt.t3.x, wt.t3.y, wt.t3.z]);
            indices.push(...[i++, i++, i++]);
        }
    
        vertexData.positions = totalTriangles;
        vertexData.indices = indices;

        let styroMesh = new BABYLON.Mesh("adhesiveMesh", this.scene);
        vertexData.applyToMesh(styroMesh);

        styroMesh.material = this.styroMat;
        
        createAt = event.Create.styro_position;
        styroMesh.position = new BABYLON.Vector3(createAt.x, createAt.y, createAt.z);

        this.meshMap.set(event.Create.tile_id, {styroMesh, adhesiveMesh});
    }

    showFixAnimation(event: Fix) {
        
    }

    showTeleportAnimation(event: Teleport) {

    }

    createTranslationAnimation(start_position: Point, end_position: Point, duration: number, frameRate: number): BABYLON.Animation[] {
        return  [
                this.createTranslationAnimationOnAxis("xSlide", "position.x", start_position.x, end_position.x, duration, frameRate),
                this.createTranslationAnimationOnAxis("ySlide", "position.y", start_position.y, end_position.y, duration, frameRate),
                this.createTranslationAnimationOnAxis("zSlide", "position.z", start_position.z, end_position.z, duration, frameRate),
            ];
    }

    createTranslationAnimationOnAxis(name: string, axis: string, start_position: number, end_position: number, duration: number, frameRate: number): BABYLON.Animation {
        const slide = new BABYLON.Animation(name, axis, frameRate, BABYLON.Animation.ANIMATIONTYPE_FLOAT, BABYLON.Animation.ANIMATIONLOOPMODE_CONSTANT);

        const keyFrames = [{
            frame: 0,
            value: start_position
        },
        {
            frame: duration / 1000 * frameRate,
            value: end_position
        }];

        slide.setKeys(keyFrames);

        return slide;
    }

    getBuildingMeshVertexData(): BABYLON.VertexData {
        var vertexData = new BABYLON.VertexData();

        let totalTriangles = [];
        let indices = [];
        let i = 0;

        for(let wall of this.plan.building.walls) {
            for(let wt of wall.triangles){
                totalTriangles.push(...[wt.t1.x, wt.t1.y, wt.t1.z, wt.t2.x, wt.t2.y, wt.t2.z, wt.t3.x, wt.t3.y, wt.t3.z]);
                indices.push(...[i++, i++, i++]);
            }
        }

        vertexData.positions = totalTriangles;
        vertexData.indices = indices;

        return vertexData;
    }

    getBuildingWireframeData(): BABYLON.Vector3[][] {
        let wireframe: BABYLON.Vector3[][] = [];

        for(let lineSeq of this.plan.building.wireframe) {
            let lineSeqFront = [];
            for (let point of lineSeq) {
                lineSeqFront.push(new BABYLON.Vector3(point.x, point.y, point.z));
            }
            wireframe.push(lineSeqFront);
        }

        return wireframe;
    }

    showBuilding() {
        let mode = getShowBuilding();

        var buildingMesh = new BABYLON.Mesh("buildingMesh", this.scene);
        this.buildingMeshVertexData.applyToMesh(buildingMesh);
        this.buildingMesh = buildingMesh;

        const buildingWireframe = BABYLON.MeshBuilder.CreateLineSystem("linesystem", {lines: this.buildingWireframeData}, this.scene); 
        buildingWireframe.color = BABYLON.Color3.Black();
        this.buildingWireframeMesh = buildingWireframe;
        
        var mat = new BABYLON.StandardMaterial("matBuildingMesh", this.scene);
        mat.backFaceCulling = false;
        mat.transparencyMode = 0;
        mat.diffuseColor = BABYLON.Color3.Yellow();
        buildingMesh.material = mat;

        this.buildingMeshSetVisibility();
    }

    buildingMeshSetVisibility() {
        let mode = getShowBuilding();
        if (mode !== ShowBuildingOrIsolation.Show) {
            this.buildingMesh.setEnabled(false);
        } else {
            this.buildingMesh.setEnabled(true);
        }

        if(mode !== ShowBuildingOrIsolation.Hide) {
            this.buildingWireframeMesh.setEnabled(true);
        } else {
            this.buildingWireframeMesh.setEnabled(false);
        }
    }

    dispose() {
        this.engine.dispose();
    }
    
    createScene() {
        return new BABYLON.Scene(this.engine);
    }

    connectCamera(cam?: {position: {x: number, y: number, z: number}, target: {x: number, y: number, z: number}}) {
        let px = -20, py = 30, pz  = -40, tx = 20, ty = 0, tz = 30;
        if (cam) {
            px = cam.position.x;
            py = cam.position.y;
            pz = cam.position.z;
            tx = cam.target.x;
            ty = cam.target.y;
            tz = cam.target.z;
        }
        var camera = new BABYLON.FreeCamera("camera1", new BABYLON.Vector3(px, py, pz), this.scene);
        camera.invertRotation = true;
        camera.setTarget(new BABYLON.Vector3(tx, ty, tz));
        camera.attachControl(this.canvas, true);
    }

    connectLights() {
        var hemiLight = new BABYLON.HemisphericLight("hemiLight", new BABYLON.Vector3(0, 1, 0), this.scene);
        hemiLight.intensity = 0.5;

        var spotLight = new BABYLON.SpotLight("spotLight", new BABYLON.Vector3(10, 100, 10), new BABYLON.Vector3(0, -1, 0), Math.PI / 3, 2, this.scene);
        spotLight.intensity = 0.2;
    }

    getCamera(): {position: {x: number, y: number, z: number}, target: {x: number, y: number, z: number}} {
        let p =  this.scene.cameras[0].position;
        let t = (this.scene.cameras[0] as FreeCamera).target;
        
        return {position: {x: p._x, y: p._y, z: p._z}, target:{x: t._x, y: t._y, z: t._z}};
    }
    
    initGeneralGameStuff() {
            this.engine.runRenderLoop(() => {
            this.scene.render();
        });

        window.addEventListener("keydown", (ev) => {
            if (ev.shiftKey && ev.ctrlKey && ev.altKey && ev.keyCode === 73) {
                if (this.scene.debugLayer.isVisible()) {
                    this.scene.debugLayer.hide();
                } else {
                    this.scene.debugLayer.show();
                }
            }
        });
    }
}

function add_to_point(p: Point, inc: Point): Point {
    return {x: p.x + inc.x, y: p.y + inc.y, z: p.z + inc.z};
}
