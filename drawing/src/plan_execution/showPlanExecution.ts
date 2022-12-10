import * as BABYLON from '@babylonjs/core';
import { FreeCamera } from '@babylonjs/core';
import {Create, Fix, Plan, PlanExecutionEvent, Point, ShowBuildingOrIsolation, Teleport, Translate, } from '../models';
import {HttpBackend} from '../backendService';
import {backendPlanToBabylonPlan} from '../util/util';

export function reloadApp() {
    let camera = ((window as any).babylonApp as any)?.getCamera();
    ((window as any).babylonApp as App)?.dispose();
    (window as any).babylonApp = new App(camera);
    if (!(window as any).eventListenersForShowBuildingSet) {
        ((window as any).babylonApp as App)?.addEventListeners();
        (window as any).eventListenersForShowBuildingSet = true;
    }
}

class App {
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
        this.canvas = this.getCanvas();
        this.engine = new BABYLON.Engine(this.canvas, true);
        this.scene = this.createScene();

        this.initAnimationMeshes();

        new HttpBackend().get_plan(this.getRequestId(), this.getTileLength(), this.getTileHeight(), this.getTileWidth(), this.getVelocity())
        .then((response) => response.json())
        .then((data) => {
            this.plan = backendPlanToBabylonPlan(data as Plan);
            
            this.buildingMeshVertexData = this.getBuildingMeshVertexData();
            this.buildingWireframeData = this.getBuildingWireframeData();

            this.connectCamera(camera);
            this.connectLights();
            this.showBuilding();

            this.scheduleAnimations();

            if (this.getShowAxes()) {
                this.showAxis(50);
            }

            this.initGeneralGameStuff();
        });
    }

    initAnimationMeshes() {
        this.styroMat = new BABYLON.StandardMaterial("mat", this.scene);
        this.styroMat.backFaceCulling = false;
        this.styroMat.diffuseColor = BABYLON.Color3.Blue();

        this.adhesiveMat = new BABYLON.StandardMaterial("mat", this.scene);
        this.adhesiveMat.backFaceCulling = false;
        this.adhesiveMat.diffuseColor = BABYLON.Color3.Green();
        

        this.styroUnionMesh = new BABYLON.Mesh("custom", this.scene);
        
        var positions = [-5, 2, -3, -7, -2, -3, -3, -2, -3,];
        var indices = [0, 1, 2];
        
        var vertexData = new BABYLON.VertexData();

        vertexData.positions = positions;
        vertexData.indices = indices;


        vertexData.applyToMesh(this.styroUnionMesh);

        positions = [-5, 2, -3, -7, -2, -3, -3, -2, -3,];
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
        let mode = this.getShowBuilding();

        var buildingMesh = new BABYLON.Mesh("buildingMesh", this.scene);
        this.buildingMeshVertexData.applyToMesh(buildingMesh);
        this.buildingMesh = buildingMesh;

        const buildingWireframe = BABYLON.MeshBuilder.CreateLineSystem("linesystem", {lines: this.buildingWireframeData}, this.scene); 
        buildingWireframe.color = BABYLON.Color3.Black();
        this.buildingWireframeMesh = buildingWireframe;
        
        var mat = new BABYLON.StandardMaterial("matBuildingMesh", this.scene);
        mat.backFaceCulling = false;
        mat.transparencyMode = 0;
        buildingMesh.material = mat;

        this.buildingMeshSetVisibility();
    }

    buildingMeshSetVisibility() {
        let mode = this.getShowBuilding();
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

    getRequestId(): number {
        return parseInt((document.getElementById('request-id') as any)?.value ?? localStorage.getItem('requestId') ?? '1');
    }

    getTileLength(): number {
        return parseFloat((document.getElementById('tile-length') as any)?.value ?? localStorage.getItem('tileLength') ?? '5');
    }

    getTileHeight(): number {
        return parseFloat((document.getElementById('tile-height') as any)?.value ?? localStorage.getItem('tileHeight') ?? '2.5');
    }

    getTileWidth(): number {
        return parseFloat((document.getElementById('tile-width') as any)?.value ?? localStorage.getItem('tileWidth') ?? '0.3');
    }

    getVelocity(): number {
        return parseFloat((document.getElementById('tile-setting-velocity') as any)?.value ?? localStorage.getItem('tileSettingVelocity') ?? '0.001');
    }

    getShowBuilding(): ShowBuildingOrIsolation {
        return parseInt((document.getElementById('building') as any)?.value ?? localStorage.getItem('building') ?? '1');
    }

    getShowIsolation(): ShowBuildingOrIsolation {
        return parseInt((document.getElementById('isolation') as any)?.value ?? localStorage.getItem('isolation') ?? '1');
    }

    getShowAxes(): boolean {
        return document.querySelector('#show-axes' as any).checked;
    }

    getCanvas(): HTMLCanvasElement {
        return document.getElementById("canvas") as HTMLCanvasElement;
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
    

    showAxis(size: number) {
        var scene = this.scene;
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
        var xChar = makeTextPlane("X", "red", size / 10);
        xChar.position = new BABYLON.Vector3(0.9 * size, -0.05 * size, 0);
        var axisY = BABYLON.Mesh.CreateLines("axisY", [
            BABYLON.Vector3.Zero(), new BABYLON.Vector3(0, size, 0), new BABYLON.Vector3( -0.05 * size, size * 0.95, 0), 
            new BABYLON.Vector3(0, size, 0), new BABYLON.Vector3( 0.05 * size, size * 0.95, 0)
        ], scene, true);
        axisY.color = new BABYLON.Color3(0, 1, 0);
        var yChar = makeTextPlane("Y", "green", size / 10);
        yChar.position = new BABYLON.Vector3(0, 0.9 * size, -0.05 * size);
        var axisZ = BABYLON.Mesh.CreateLines("axisZ", [
            new BABYLON.Vector3(0, 0, 0), new BABYLON.Vector3(0, 0, size), new BABYLON.Vector3( 0 , -0.05 * size, size * 0.95),
            new BABYLON.Vector3(0, 0, size), new BABYLON.Vector3( 0, 0.05 * size, size * 0.95)
        ], scene, true);
        axisZ.color = new BABYLON.Color3(0, 0, 1);
        var zChar = makeTextPlane("Z", "blue", size / 10);
        zChar.position = new BABYLON.Vector3(0, 0.05 * size, 0.9 * size);
    }

    showGround() {
        BABYLON.MeshBuilder.CreateGround("ground", {height: 100, width: 100, subdivisions: 4});
    }

    addEventListeners() {
        document.getElementById("request-id")?.addEventListener('input', (event) => {
            localStorage.setItem("requestId", (event as any).data);
            reloadApp();
        })
        
        document.getElementById("tile-length")?.addEventListener('input', (event) => {
            localStorage.setItem("tileLength", (event as any).data);
            reloadApp();
        })
        
        document.getElementById("tile-height")?.addEventListener('input', (event) => {
            localStorage.setItem("tileHeight", (event as any).data);
            reloadApp();
        })
        
        document.getElementById("tile-width")?.addEventListener('input', (event) => {
            localStorage.setItem("tileWidth", (event as any).data);
            reloadApp();
        })
        
        document.getElementById("show-axes")?.addEventListener('input', (event) => {
            localStorage.setItem("showAxes", document.querySelector('#show-axes' as any).checked.toString());                
            reloadApp();
        })  

        document.getElementById("tile-setting-velocity")?.addEventListener('input', (event) => {
            localStorage.setItem("tileSettingVelocity", (event as any).data);
            reloadApp();
        })  
        
        document.getElementById("building")?.addEventListener('input', (event) => {
            localStorage.setItem("building", (event as any).data);
            ((window as any).babylonApp as App).buildingMeshSetVisibility();
        })
        
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