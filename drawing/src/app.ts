import * as BABYLON from '@babylonjs/core';

import (("./index.js") as any).catch(e => console.error("Error importing `index.js`:", e)).then(
    () => {
        setTimeout(() => {
            new App();
            document.getElementById("request-id")?.addEventListener('input', (event) => {
                localStorage.setItem("requestId", (event as any).data);
                location.reload();
            })
        }, 300);
    }
);

class App {
    private readonly plan: Plan;
    private readonly backend: Backend;
    private readonly canvas: HTMLCanvasElement;
    private readonly engine: BABYLON.Engine;
    private readonly scene: BABYLON.Scene;
    private readonly buildingMeshVertexData: BABYLON.VertexData;
    private readonly buildingWireframeData: BABYLON.Vector3[][];
    private readonly isolationMeshVertexData: BABYLON.VertexData;
    private readonly isolationWireframeData: BABYLON.Vector3[][];

    constructor() {
        this.canvas = this.getCanvas();
        this.engine = new BABYLON.Engine(this.canvas, true);

        this.backend = (window as any).wasm as Backend;
        this.plan = JSON.parse(this.backend.get_plan(this.getRequestId()));
    
        this.buildingMeshVertexData = this.getBuildingMeshVertexData();
        this.buildingWireframeData = this.getBuildingWireframeData();

        this.isolationMeshVertexData = this.getIsolationMeshVertexData();
        this.isolationWireframeData = this.getIsolationWireframeData();

        this.scene = this.createScene();

        this.connectCamera();
        this.connectLights();
        this.showBuilding();
        this.showIsolation();
        this.showAxis(50);

        this.initGeneralGameStuff();
    }

    getRequestId(): number {
        return parseInt((document.getElementById('request-id') as any)?.value ?? localStorage.getItem('requestId') ?? '1');
    }

    getCanvas(): HTMLCanvasElement {
        return document.getElementById("canvas") as HTMLCanvasElement;
    }

    createScene() {
        return new BABYLON.Scene(this.engine)
    }

    connectCamera() {
        var camera = new BABYLON.FreeCamera("camera1", new BABYLON.Vector3(-20, 30, -40), this.scene);
        camera.invertRotation = true;
        camera.setTarget(new BABYLON.Vector3(20, 0, 30));
        camera.attachControl(this.canvas, true);
    }

    connectLights() {
        var hemiLight = new BABYLON.HemisphericLight("hemiLight", new BABYLON.Vector3(0, 1, 0), this.scene);
        hemiLight.intensity = 0.5;

        var spotLight = new BABYLON.SpotLight("spotLight", new BABYLON.Vector3(10, 100, 10), new BABYLON.Vector3(0, -1, 0), Math.PI / 3, 2, this.scene);
        spotLight.intensity = 0.2;
    }

    showBuilding() {
        var buildingMesh = new BABYLON.Mesh("buildingMesh", this.scene);
        this.buildingMeshVertexData.applyToMesh(buildingMesh);

        const buildingWireframe = BABYLON.MeshBuilder.CreateLineSystem("linesystem", {lines: this.buildingWireframeData}, this.scene); 
        buildingWireframe.color = BABYLON.Color3.Red();

        var mat = new BABYLON.StandardMaterial("matBuildingMesh", this.scene);
        mat.wireframe = false;
        mat.backFaceCulling = false;
        mat.transparencyMode = 0;
        buildingMesh.material = mat;
    }

    showIsolation() {
        var isolationMesh = new BABYLON.Mesh("isolationMesh", this.scene);
        this.isolationMeshVertexData.applyToMesh(isolationMesh);

        const isolationWireframe = BABYLON.MeshBuilder.CreateLineSystem("linesystem", {lines: this.isolationWireframeData}, this.scene); 
        isolationWireframe.color = BABYLON.Color3.Black();

        var mat = new BABYLON.StandardMaterial("mat", this.scene);
        mat.wireframe = false;
        mat.backFaceCulling = false;
        mat.transparencyMode = 0;
        mat.alpha = 1;
        mat.diffuseColor = BABYLON.Color3.Green();
        isolationMesh.material = mat;
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

    getIsolationMeshVertexData(): BABYLON.VertexData {
        var vertexData = new BABYLON.VertexData();

        let totalTriangles = [];
        let indices = [];
        let i = 0;

        for(let tile of this.plan.tiles.tiles) {
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

        for(let lineSeq of this.plan.tiles.wireframe) {
            let lineSeqFront = [];
            for (let point of lineSeq) {
                lineSeqFront.push(new BABYLON.Vector3(point.x, point.z, point.y));
            }
            wireframe.push(lineSeqFront);
        }

        return wireframe;
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
            new BABYLON.Vector3(0, -0.5, 0), new BABYLON.Vector3(0, 0, size), new BABYLON.Vector3( 0 , -0.05 * size, size * 0.95),
            new BABYLON.Vector3(0, 0, size), new BABYLON.Vector3( 0, 0.05 * size, size * 0.95)
        ], scene, true);
        axisZ.color = new BABYLON.Color3(0, 0, 1);
        var zChar = makeTextPlane("Z", "blue", size / 10);
        zChar.position = new BABYLON.Vector3(0, 0.05 * size, 0.9 * size);
    }

    showGround() {
        BABYLON.MeshBuilder.CreateGround("ground", {height: 100, width: 100, subdivisions: 4});
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



interface Backend {
    get_plan(request_id: number): string,
}

interface Plan {
    building: Building,
    tiles: Tiles
}

interface Building {
    walls: TriangulizedWall[],
    wireframe: Point [][] 
}

interface Tiles {
    tiles: TriangulizedTile[],
    wireframe: Point [][] 
}

interface TriangulizedWall {
    triangles: Triangle[]
}

interface TriangulizedTile {
    triangles: Triangle[]
}

interface Point {
    x: number, 
    y: number,
    z: number
}

interface Triangle {
    t1: Point,
    t2: Point,
    t3: Point
}
