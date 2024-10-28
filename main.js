import './style.css'
import * as THREE from 'three';

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer({
  canvas: document.querySelector('#bg'),
});

renderer.setPixelRatio(window.devicePixelRatio);
renderer.setSize(window.innerWidth, window.innerHeight);

camera.position.z = 10;

const earthGeo = new THREE.SphereGeometry(2, 32, 32);
const earthMaterial = new THREE.MeshBasicMaterial({
  color: 0x00FF41,
  wireframe: true,
});
const earth = new THREE.Mesh(earthGeo, earthMaterial);
earth.rotation.x = THREE.MathUtils.degToRad(23.5);
scene.add(earth);

const eplaneGeometry = new THREE.PlaneGeometry(6, 6);
const eplaneMaterial = new THREE.MeshBasicMaterial({
  color: 0x4287f5,
  transparent: true,
  opacity: 0.3,
  side: THREE.DoubleSide,
});
const equatorialPlane = new THREE.Mesh(eplaneGeometry, eplaneMaterial);
equatorialPlane.rotation.x = THREE.MathUtils.degToRad(110);
scene.add(equatorialPlane);

const earthRotationSpeed = (2 * Math.PI) / 86400; //radians per second for 24hr rotation

function animate() {
  const currentTime = performance.now();
  const deltaTime = (currentTime - lastTime) / 1000; //convert to seconds
  lastTime = currentTime;

  requestAnimationFrame(animate);

  //earth.rotation.y += earthRotationSpeed * deltaTime;
  earth.rotation.y += 0.001;

  renderer.render(scene, camera);
}

let lastTime = performance.now();
animate();

window.addEventListener('resize', () => {
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();
  renderer.setSize(window.innerWidth, window.innerHeight);
});
