import * as three from 'three';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader.js';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

let scene, camera, renderer, character, mixer, clock;
let walkAction, idleAction;
let moving = false;
const keys = {}

let boxes = [];

const container = document.getElementById('challenge');

const onWindowResize = () => {
    camera.aspect = container.clientWidth / container.clientHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(container.clientWidth, container.clientHeight);
}

const onKeyDown = (e) => {
    keys[e.code] = true;
}

const onKeyUp = (e) => {
    keys[e.code] = false;
}

const init = () => {
    scene = new three.Scene();
    camera = new three.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    renderer = new three.WebGLRenderer({ antialias: true});
    renderer.setSize(window.innerWidth, window.innerHeight);
    container.appendChild(renderer.domElement);

    clock = new three.Clock();
    
    // Adjust light positions and intensity
    const light = new three.DirectionalLight(0xffffff, 1);
    light.position.set(5, 10, 7.5);
    light.castShadow = true;
    scene.add(light);



    for (let i = 0; i < 10; i++) {
        const box = new three.Mesh(
            new three.BoxGeometry(1, 1, 1),
            new three.MeshStandardMaterial({ color: 0x00ff00 })
        );
        box.position.set(Math.random() * 10 - 5, 0.5, Math.random() * 10 - 5);
        box.castShadow = true;
        scene.add(box);
        boxes.push(box);
    }

    const loader = new GLTFLoader();
    loader.load('src/Astronaut.glb',(gltf) => {
        character = gltf.scene;
        scene.add(character);

        mixer = new three.AnimationMixer(character);
        gltf.animations.forEach((clip) => {
            if (clip.name === 'CharacterArmature|Walk') {
                walkAction = mixer.clipAction(clip);
            }
            if (clip.name === 'CharacterArmature|Idle') {
                idleAction = mixer.clipAction(clip);
            }
        });

        if (walkAction) walkAction.play();
        if (idleAction) idleAction.play();

        character.position.set(-10, 0, -60);
        character.rotation.y = Math.PI;
    })

    //blue sky
    scene.background = new three.Color(0x87ceeb);

    camera.position.set(20, 10, 10)
    camera.lookAt(0, 10, 0)

    window.addEventListener('resize', onWindowResize, false);
    document.addEventListener('keydown', onKeyDown, false);
    document.addEventListener('keyup', onKeyUp, false);
}

const animate = () => {
    requestAnimationFrame(animate);

    const delta = clock.getDelta();
    if (mixer) mixer.update(delta);

    if (character) {
        if (keys['KeyW']) {
            character.position.z -= 0.1;
            character.rotation.y = Math.PI; // North
        }

        if (keys['KeyS']) {
            character.position.z += 0.1;
            character.rotation.y = 0; // South
        }

        if (keys['KeyA']) {
            character.position.x -= 0.1;
            character.rotation.y = Math.PI / 2; // West
        }

        if (keys['KeyD']) {
            character.position.x += 0.1;
            character.rotation.y = -Math.PI / 2; // East
        }

        // check if any key is pressed
        if (keys['KeyW'] || keys['KeyS'] || keys['KeyA'] || keys['KeyD']) {
            moving = true;
        } else {
            moving = false;
        }

         // Play walking animation if moving
         if (mixer) {
            if (walkAction && idleAction) {
                walkAction.enabled = moving;
                idleAction.enabled = !moving;
            }
        }
    }

    renderer.render(scene, camera);
}

const threejs = import.meta.env.VITE_THREE_JS === "true";
if (threejs) {
    init();
    animate();
}
