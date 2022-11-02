import * as BABYLON from '@babylonjs/core';
import * as earcut from "earcut";

class App {
    constructor() {

        (window as any).earcut = earcut.default;

        let canvas = document.getElementById("canvas") as HTMLCanvasElement;

        const engine = new BABYLON.Engine(canvas, true);

        let that = this;

        var scene: any;

        fetch('http://localhost:8080/abc.json')
        .then((response) => response.json())
        .then((data) => {

            scene = that.createScene(engine, canvas, data);

            engine.runRenderLoop(() => {
                scene.render();
            });

            window.addEventListener("keydown", (ev) => {
                if (ev.shiftKey && ev.ctrlKey && ev.altKey && ev.keyCode === 73) {
                    if (scene.debugLayer.isVisible()) {
                        scene.debugLayer.hide();
                    } else {
                        scene.debugLayer.show();
                    }
                }
            });
        });
    }

    createScene(engine: BABYLON.Engine, canvas: HTMLCanvasElement, data: any) {
        var scene = new BABYLON.Scene(engine);

        var camera = new BABYLON.FreeCamera("camera1", new BABYLON.Vector3(-20, 30, -40), scene);
        camera.invertRotation = true;
        camera.setTarget(new BABYLON.Vector3(20, 0, 30));
        camera.attachControl(canvas, true);

        var hemiLight = new BABYLON.HemisphericLight("hemiLight", new BABYLON.Vector3(0, 1, 0), scene);
        hemiLight.intensity = 0.5;

        var spotLight = new BABYLON.SpotLight("spotLight", new BABYLON.Vector3(10, 100, 10), new BABYLON.Vector3(0, -1, 0), Math.PI / 3, 2, scene);
        spotLight.intensity = 0.2;

        let totalTriangles = [];
        let indices = [];
        let i = 0;

        for(let wall of data.building.walls) {
            let wallTriangles: [] = wall.triangles;
            let wallTriangle: any;
            for(wallTriangle of wallTriangles){
                let wt = wallTriangle;
                totalTriangles.push(...[wt.t1.x, wt.t1.z, wt.t1.y, wt.t2.x, wt.t2.z, wt.t2.y, wt.t3.x, wt.t3.z, wt.t3.y]);
                indices.push(...[i++, i++, i++]);
            }
        }

        var customMesh = new BABYLON.Mesh("custom", scene);

        var vertexData = new BABYLON.VertexData();

        vertexData.positions = totalTriangles;
        vertexData.indices = indices;

        vertexData.applyToMesh(customMesh);
        
        var mat = new BABYLON.StandardMaterial("mat", scene);
        mat.wireframe = false;
        mat.backFaceCulling = false;
        mat.transparencyMode = 0;
        customMesh.material = mat;

        let wireframe = [];

        for(let lineSeq of data.building.wireframe) {
            let lineSeqFront = [];
            for (let point of lineSeq) {
                console.log(point.x, point.z, point.y);
                lineSeqFront.push(new BABYLON.Vector3(point.x, point.z, point.y));
            }
            wireframe.push(lineSeqFront);
        }
    
        const linesystem = BABYLON.MeshBuilder.CreateLineSystem("linesystem", {lines: wireframe}, scene); 
        linesystem.color = BABYLON.Color3.Black();

        // const ground = BABYLON.MeshBuilder.CreateGround("ground", {height: 100, width: 100, subdivisions: 4});

        var showAxis = function(size: number) {
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
                BABYLON.Vector3.Zero(), new BABYLON.Vector3(0, 0, size), new BABYLON.Vector3( 0 , -0.05 * size, size * 0.95),
                new BABYLON.Vector3(0, 0, size), new BABYLON.Vector3( 0, 0.05 * size, size * 0.95)
            ], scene, true);
            axisZ.color = new BABYLON.Color3(0, 0, 1);
            var zChar = makeTextPlane("Z", "blue", size / 10);
            zChar.position = new BABYLON.Vector3(0, 0.05 * size, 0.9 * size);
        };
	
	    // showAxis(100);
        return scene; 
    }
}

new App();
