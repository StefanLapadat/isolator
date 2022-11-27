import * as BABYLON from '@babylonjs/core';
import { FreeCamera } from '@babylonjs/core';
import {Plan, ShowBuildingOrIsolation, } from './models';
import {HttpBackend} from './backendService';

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
    
    constructor(camera?: {position: {x: number, y: number, z: number}, target: {x: number, y: number, z: number}}) {
        this.canvas = this.getCanvas();
        this.engine = new BABYLON.Engine(this.canvas, true);

        new HttpBackend().get_plan(this.getRequestId(), this.getTileLength(), this.getTileHeight(), this.getTileWidth())
        .then((response) => response.json())
        .then((data) => {
            this.plan = data as Plan;
            
            this.scene = this.createScene();

            this.connectCamera(camera);
            this.connectLights();

            this.scheduleAnimations();

            if (this.getShowAxes()) {
                this.showAxis(50);
            }

            this.initGeneralGameStuff();
        });
    }

    scheduleAnimations() {
        for (let ev of this.plan.planExecution.events) {
            this.scheduleAnimation(ev.start, ev.end);
        }
    }

    scheduleAnimation(start: number, end: number) {
        setTimeout(() => {
            this.showAnimation(end - start);
        }, start);
    }

    showAnimation(duration: number) {
        const box = BABYLON.MeshBuilder.CreateBox("box" + Math.random(), {});
        console.log(duration + " *****************************");
        box.position.x = 2;

        const frameRate = 10;

        const xSlide = new BABYLON.Animation("xSlide", "position.x", frameRate, BABYLON.Animation.ANIMATIONTYPE_FLOAT, BABYLON.Animation.ANIMATIONLOOPMODE_CONSTANT);

        const keyFrames = []; 

        keyFrames.push({
            frame: 0,
            value: 2
        });

        keyFrames.push({
            frame: duration / 1000 * frameRate,
            value: -2
        });

        xSlide.setKeys(keyFrames);

        box.animations.push(xSlide);

        this.scene.beginAnimation(box, 0, duration / 1000 * frameRate, true);
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

