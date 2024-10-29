import './style.css'
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer({
  canvas: document.querySelector('#bg'),
});

renderer.setPixelRatio(window.devicePixelRatio);
renderer.setSize(window.innerWidth, window.innerHeight);

camera.position.z = 10;
camera.position.y = 1;

const controls = new OrbitControls(camera, renderer.domElement);
controls.enableDamping = true;
controls.dampingFactor = 0.05;
controls.minDistance = 5;
controls.maxDistance = 20;

const earthGeo = new THREE.SphereGeometry(2, 32, 32);
const earthMaterial = new THREE.MeshBasicMaterial({
  color: 0x00FF41,
  wireframe: true,
});
const earth = new THREE.Mesh(earthGeo, earthMaterial);
earth.rotation.x = THREE.MathUtils.degToRad(23.5);
scene.add(earth);

const eplaneGeometry = new THREE.PlaneGeometry(7, 7);
const eplaneMaterial = new THREE.MeshBasicMaterial({
  color: 0x4287f5,
  transparent: true,
  opacity: 0.3,
  side: THREE.DoubleSide,
});
const equatorialPlane = new THREE.Mesh(eplaneGeometry, eplaneMaterial);
equatorialPlane.rotation.x = THREE.MathUtils.degToRad(90);
scene.add(equatorialPlane);

const orbitalPlaneGeometry = new THREE.PlaneGeometry(8, 8);
const orbitalPlaneMaterial = new THREE.MeshBasicMaterial({
  color: 0xF54242,
  transparent: true,
  opacity: 0.3,
  side: THREE.DoubleSide,
});
const orbitalPlane = new THREE.Mesh(orbitalPlaneGeometry, orbitalPlaneMaterial);
orbitalPlane.rotation.x = THREE.MathUtils.degToRad(160);
orbitalPlane.rotation.z = THREE.MathUtils.degToRad(45);
scene.add(orbitalPlane);

const nodeGeometry = new THREE.SphereGeometry(0.1, 16, 16);
const nodeMaterial = new THREE.MeshBasicMaterial({ color: 0xFFFF00 });

const descendingNode = new THREE.Mesh(nodeGeometry, nodeMaterial);
descendingNode.position.set(-3.5, 0, 0);
scene.add(descendingNode);

const ascendingNode = new THREE.Mesh(nodeGeometry, nodeMaterial);
ascendingNode.position.set(3.5, 0, 0);
scene.add(ascendingNode);

function createTextTexture(text) {
  const canvas = document.createElement('canvas');
  const context = canvas.getContext('2d');
  canvas.width = 256;
  canvas.height = 64;

  context.fillStyle = 'white';
  context.font = 'bold 16px Arial';
  context.fillText(text, 0, 40);

  const texture = new THREE.CanvasTexture(canvas);
  return texture;
}

const descendingSpriteMaterial = new THREE.SpriteMaterial({
  map: createTextTexture('Descending Node'),
  sizeAttenuation: false
});

const ascendingSpriteMaterial = new THREE.SpriteMaterial({
  map: createTextTexture('Ascending Node'),
  sizeAttenuation: false
});

const descendingText = new THREE.Sprite(descendingSpriteMaterial);
descendingText.position.set(-3.5, 0.5, 0);
descendingText.scale.set(0.5, 0.2, 0.3);
scene.add(descendingText);

const ascendingText = new THREE.Sprite(ascendingSpriteMaterial);
ascendingText.position.set(4.5, 0.5, 0);
ascendingText.scale.set(0.5, 0.2, 0.3);
scene.add(ascendingText);

const earthRotationSpeed = (2 * Math.PI) / 86400; //radians per second for 24hr rotation

function animate() {
  const currentTime = performance.now();
  const deltaTime = (currentTime - lastTime) / 1000; //convert to seconds
  lastTime = currentTime;

  requestAnimationFrame(animate);

  //earth.rotation.y += earthRotationSpeed * deltaTime;
  earth.rotation.y += 0.001;

  descendingText.quaternion.copy(camera.quaternion);
  ascendingText.quaternion.copy(camera.quaternion);


  controls.update();

  renderer.render(scene, camera);
}

let lastTime = performance.now();
animate();

window.addEventListener('resize', () => {
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();
  renderer.setSize(window.innerWidth, window.innerHeight);
});
