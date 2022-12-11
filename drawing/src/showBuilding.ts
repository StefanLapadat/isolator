import * as BABYLON from '@babylonjs/core';
import { FreeCamera } from '@babylonjs/core';
import {Plan, ShowBuildingOrIsolation, } from './models';
import {HttpBackend} from './backendService';
import { addEventListeners, populateFieldsFromLocalStorage, showAxis } from './util/util';
import { BuildingWithVisibility } from './buildingWithVisibility';

export function reloadApp() {
    let camera = ((window as any).babylonApp as any)?.getCamera();
    ((window as any).babylonApp as ShowBuilding)?.dispose();
    (window as any).babylonApp = new ShowBuilding(camera);
    if (!(window as any).eventListenersForShowBuildingSet) {
        addEventListeners(reloadApp, ((window as any).babylonApp as ShowBuilding));
        (window as any).eventListenersForShowBuildingSet = true;
    }
}

class ShowBuilding implements BuildingWithVisibility {
    private plan: Plan;
    private readonly canvas: HTMLCanvasElement;
    private readonly engine: BABYLON.Engine;
    private scene: BABYLON.Scene;

    private buildingMeshVertexData: BABYLON.VertexData;
    private buildingMesh: BABYLON.Mesh;
    private buildingWireframeData: BABYLON.Vector3[][];
    private buildingWireframeMesh: BABYLON.LinesMesh;

    private isolationMesh: BABYLON.Mesh;
    private isolationMeshVertexData: BABYLON.VertexData;
    private isolationWireframeData: BABYLON.Vector3[][];
    private isolationWireframeMesh: BABYLON.LinesMesh;

    private adhesiveMesh: BABYLON.Mesh;
    private adhesiveMeshVertexData: BABYLON.VertexData;
    private adhesiveWireframeData: BABYLON.Vector3[][];
    private adhesiveWireframeMesh: BABYLON.LinesMesh;

    
    constructor(camera?: {position: {x: number, y: number, z: number}, target: {x: number, y: number, z: number}}) {
        populateFieldsFromLocalStorage();
        this.canvas = this.getCanvas();
        this.engine = new BABYLON.Engine(this.canvas, true);

        new HttpBackend().get_plan(this.getRequestId(), this.getTileLength(), this.getTileHeight(), this.getTileWidth(), this.getVelocity())
        .then((response) => response.json())
        .then((data) => {
            this.plan = data as Plan;

            this.buildingMeshVertexData = this.getBuildingMeshVertexData();
            this.buildingWireframeData = this.getBuildingWireframeData();

            this.isolationMeshVertexData = this.getIsolationMeshVertexData();
            this.isolationWireframeData = this.getIsolationWireframeData();

            this.adhesiveMeshVertexData = this.getAdhesiveMeshVertexData();
            this.adhesiveWireframeData = this.getAdhesiveWireframeData();

            this.scene = this.createScene();

            this.connectCamera(camera);
            this.connectLights();
            this.showBuilding();
            this.showIsolation();

            if (this.getShowAxes()) {
                showAxis(50, this.scene);
            }

            this.initGeneralGameStuff();
        });
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

    getBuildingMeshVertexData(): BABYLON.VertexData {
        var vertexData = new BABYLON.VertexData();

        let totalTriangles = [];
        let indices = [];
        let i = 0;

        for(let wall of this.plan.building.walls) {
            for(let wt of wall.triangles){
                totalTriangles.push(...[wt.t1.x, wt.t1.z, wt.t1.y, wt.t2.x, wt.t2.z, wt.t2.y, wt.t3.x, wt.t3.z, wt.t3.y]);
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
                lineSeqFront.push(new BABYLON.Vector3(point.x, point.z, point.y));
            }
            wireframe.push(lineSeqFront);
        }

        return wireframe;
    }


    showIsolation() {
        let mode = this.getShowIsolation();

        var isolationMesh = new BABYLON.Mesh("isolationMesh", this.scene);
        this.isolationMeshVertexData.applyToMesh(isolationMesh);
        this.isolationMesh = isolationMesh;
        
        const isolationWireframe = BABYLON.MeshBuilder.CreateLineSystem("linesystem", {lines: this.isolationWireframeData}, this.scene); 
        isolationWireframe.color = BABYLON.Color3.Black();
        this.isolationWireframeMesh = isolationWireframe;

        var mat = new BABYLON.StandardMaterial("mat", this.scene);
        mat.backFaceCulling = false;
        mat.transparencyMode = 0;
        mat.alpha = 1;
        mat.diffuseColor = BABYLON.Color3.Yellow();
        isolationMesh.material = mat;

        this.showAdhesive();

        this.isolationMeshSetVisibility();
    }

    isolationMeshSetVisibility() {
        let mode = this.getShowIsolation();
        if (mode !== ShowBuildingOrIsolation.Show) {
            this.isolationMesh.setEnabled(false);
            this.adhesiveMesh.setEnabled(false);
        } else {
            this.isolationMesh.setEnabled(true);
            this.adhesiveMesh.setEnabled(true);
        }

        if(mode !== ShowBuildingOrIsolation.Hide) {
            this.isolationWireframeMesh.setEnabled(true);
            this.adhesiveWireframeMesh.setEnabled(true);
        } else {
            this.isolationWireframeMesh.setEnabled(false);
            this.adhesiveWireframeMesh.setEnabled(false);
        }
    }

    getIsolationMeshVertexData(): BABYLON.VertexData {
        var vertexData = new BABYLON.VertexData();

        let totalTriangles = [];
        let indices = [];
        let i = 0;

        for(let tile of this.plan.tiles.triangulized_tiles.tiles) {
            for(let wt of tile.triangles){
                totalTriangles.push(...[wt.t1.x, wt.t1.z, wt.t1.y, wt.t2.x, wt.t2.z, wt.t2.y, wt.t3.x, wt.t3.z, wt.t3.y]);
                indices.push(...[i++, i++, i++]);
            }
        }

        vertexData.positions = totalTriangles;
        vertexData.indices = indices;

        return vertexData;
    }

    getIsolationWireframeData(): BABYLON.Vector3[][] {
        let wireframe: BABYLON.Vector3[][] = [];

        for(let lineSeq of this.plan.tiles.triangulized_tiles.wireframe) {
            let lineSeqFront = [];
            for (let point of lineSeq) {
                lineSeqFront.push(new BABYLON.Vector3(point.x, point.z, point.y));
            }
            wireframe.push(lineSeqFront);
        }

        return wireframe;
    }


    showAdhesive() {
        let mode = this.getShowIsolation();

        var adhesiveMesh = new BABYLON.Mesh("adhesiveMesh", this.scene);
        this.adhesiveMeshVertexData.applyToMesh(adhesiveMesh);
        this.adhesiveMesh = adhesiveMesh;

        const adhesiveWireframe = BABYLON.MeshBuilder.CreateLineSystem("linesystem", {lines: this.adhesiveWireframeData}, this.scene); 
        adhesiveWireframe.color = BABYLON.Color3.Black();
        this.adhesiveWireframeMesh = adhesiveWireframe;

        var mat = new BABYLON.StandardMaterial("mat", this.scene);
        mat.backFaceCulling = false;
        mat.transparencyMode = 0;
        mat.alpha = 1;
        mat.diffuseColor = BABYLON.Color3.Green();
        adhesiveMesh.material = mat;
    }

    getAdhesiveMeshVertexData(): BABYLON.VertexData {
        var vertexData = new BABYLON.VertexData();

        let totalTriangles = [];
        let indices = [];
        let i = 0;

        for(let tile of this.plan.tiles.triangulized_adhesive.tiles) {
            for(let wt of tile.triangles){
                totalTriangles.push(...[wt.t1.x, wt.t1.z, wt.t1.y, wt.t2.x, wt.t2.z, wt.t2.y, wt.t3.x, wt.t3.z, wt.t3.y]);
                indices.push(...[i++, i++, i++]);
            }
        }

        vertexData.positions = totalTriangles;
        vertexData.indices = indices;

        return vertexData;
    }

    getAdhesiveWireframeData(): BABYLON.Vector3[][] {
        let wireframe: BABYLON.Vector3[][] = [];

        for(let lineSeq of this.plan.tiles.triangulized_adhesive.wireframe) {
            let lineSeqFront = [];
            for (let point of lineSeq) {
                lineSeqFront.push(new BABYLON.Vector3(point.x, point.z, point.y));
            }
            wireframe.push(lineSeqFront);
        }

        return wireframe;
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

