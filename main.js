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

const axesGroup = new THREE.Group();
axesGroup.rotation.x = THREE.MathUtils.degToRad(23.5);
scene.add(axesGroup);

const axisLength = 4;
const axisWidth = 2;

const zAxisGeometry = new THREE.CylinderGeometry(0.03, 0.03, axisLength, 8);
const zAxisMaterial = new THREE.MeshBasicMaterial({ color: 0x0000FF });
const zAxis = new THREE.Mesh(zAxisGeometry, zAxisMaterial);
zAxis.position.y = axisLength / 2;
axesGroup.add(zAxis);

const zArrowGeometry = new THREE.ConeGeometry(0.1, 0.3, 8);
const zArrow = new THREE.Mesh(zArrowGeometry, zAxisMaterial);
zArrow.position.y = axisLength;
axesGroup.add(zArrow);

const xAxisGeometry = new THREE.CylinderGeometry(0.03, 0.03, axisLength, 8);
const xAxisMaterial = new THREE.MeshBasicMaterial({ color: 0xFF0000 });
const xAxis = new THREE.Mesh(xAxisGeometry, xAxisMaterial);
xAxis.rotation.z = THREE.MathUtils.degToRad(-90);
xAxis.position.x = axisLength / 2;
axesGroup.add(xAxis);

const xArrowGeometry = new THREE.ConeGeometry(0.1, 0.3, 8);
const xArrow = new THREE.Mesh(xArrowGeometry, xAxisMaterial);
xArrow.rotation.z = THREE.MathUtils.degToRad(-90);
xArrow.position.x = axisLength;
axesGroup.add(xArrow);

const yAxisGeometry = new THREE.CylinderGeometry(0.03, 0.03, axisLength, 8);
const yAxisMaterial = new THREE.MeshBasicMaterial({ color: 0x00FF00 });
const yAxis = new THREE.Mesh(yAxisGeometry, yAxisMaterial);
yAxis.rotation.x = THREE.MathUtils.degToRad(90);
yAxis.position.z = axisLength / 2;
axesGroup.add(yAxis);

const yArrowGeometry = new THREE.ConeGeometry(0.1, 0.3, 8);
const yArrow = new THREE.Mesh(yArrowGeometry, yAxisMaterial);
yArrow.rotation.x = THREE.MathUtils.degToRad(90);
yArrow.position.z = axisLength;
axesGroup.add(yArrow);

const createAxisLabel = (text, position) => {
  const spriteMaterial = new THREE.SpriteMaterial({
    map: createTextTexture(text),
    sizeAttenuation: false,
    transparent: true,
    depthTest: false
  });

  const sprite = new THREE.Sprite(spriteMaterial);
  sprite.position.copy(position);
  sprite.scale.set(0.4, 0.2, 1);
  return sprite;
};

const yLabel = createAxisLabel('Y', new THREE.Vector3(axisLength + 0.5, 0, 0));
const xLabel = createAxisLabel('X', new THREE.Vector3(0, 0, axisLength + 0.5));
const zLabel = createAxisLabel('Z', new THREE.Vector3(0, axisLength + 0.5, 0));
axesGroup.add(xLabel);
axesGroup.add(yLabel);
axesGroup.add(zLabel);

const eplaneGeometry = new THREE.PlaneGeometry(7, 7);
const eplaneMaterial = new THREE.MeshBasicMaterial({
  color: 0x4287f5,
  transparent: true,
  opacity: 0.3,
  side: THREE.DoubleSide,
});
const equatorialPlane = new THREE.Mesh(eplaneGeometry, eplaneMaterial);
equatorialPlane.rotation.x = THREE.MathUtils.degToRad(115);
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

const curve = new THREE.EllipseCurve(
  0, 0, //center
  3.5, 3.5, //xradius, yradius
  0, 2 * Math.PI, //startAngle, endAngle
  false, //clockwise
  0 //rotation
);

const points = curve.getPoints(50);
const orbitGeometry = new THREE.BufferGeometry().setFromPoints(points);

const orbitMaterial = new THREE.LineDashedMaterial({
  color: 0xFFFFFF,
  linewidth: 3,
  scale: 1,
  dashSize: 0.5,
  gapSize: 0.3,
});

const orbitalPath = new THREE.Line(orbitGeometry, orbitMaterial);
orbitalPath.computeLineDistances();

orbitalPath.rotation.x = THREE.MathUtils.degToRad(160);
orbitalPath.rotation.z = THREE.MathUtils.degToRad(45);

scene.add(orbitalPath);

const lineofNodesGeometry = new THREE.BufferGeometry().setFromPoints([
  new THREE.Vector3(-3.5, 0, 0),
  new THREE.Vector3(3.5, 0, 0)
]);

const lineOfNodesMaterial = new THREE.LineBasicMaterial({
  color: 0xFFFF00,
  linewidth: 1
});

const lineOfNodes = new THREE.Line(lineofNodesGeometry, lineOfNodesMaterial);
scene.add(lineOfNodes);

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
  canvas.width = 512;
  canvas.height = 128;

  context.clearRect(0, 0, canvas.width, canvas.height);

  context.fillStyle = 'white';
  context.font = 'bold 26px Arial';
  context.textBaseline = 'middle';
  context.textAlign = 'center';

  context.shadowColor = 'rgba(0, 0, 0, 0.5)';
  context.shadowBlur = 4;
  context.shadowOffsetX = 2;
  context.shadowOffsetY = 2;

  context.fillText(text, canvas.width / 2, canvas.height / 2);

  const texture = new THREE.CanvasTexture(canvas);

  texture.minFilter = THREE.LinearFilter;
  texture.magFilter = THREE.LinearFilter;
  texture.format = THREE.RGBAFormat;

  return texture;
}

const createLabelSprite = (text, position) => {
  const spriteMaterial = new THREE.SpriteMaterial({
    map: createTextTexture(text),
    sizeAttenuation: false,
    transparent: true,
    depthTest: false // Ensures text renders on top
  });

  const sprite = new THREE.Sprite(spriteMaterial);
  sprite.position.copy(position);
  sprite.scale.set(0.60, 0.25, 1);

  return sprite;
};

const descendingText = createLabelSprite('Descending Node', new THREE.Vector3(-3.5, 0.5, 0));
const ascendingText = createLabelSprite('Ascending Node', new THREE.Vector3(4.5, 0.5, 0));
const lineOfNodesText = createLabelSprite('Line of Nodes', new THREE.Vector3(0, -0.5, 0));
scene.add(descendingText);
scene.add(ascendingText);
scene.add(lineOfNodesText);

const earthRotationSpeed = (2 * Math.PI) / 86400; //radians per second for 24hr rotation

function animate() {
  const currentTime = performance.now();
  const deltaTime = (currentTime - lastTime) / 1000; //convert to seconds
  lastTime = currentTime;

  requestAnimationFrame(animate);

  //earth.rotation.y += earthRotationSpeed * deltaTime;
  earth.rotation.y += 0.001;

  const cameraPosition = camera.position.clone();
  descendingText.position.y = 0.5;
  ascendingText.position.y = 0.5;

  descendingText.material.rotation = 0;
  ascendingText.material.rotation = 0;
  lineOfNodesText.material.rotation = 0;

  descendingText.lookAt(cameraPosition);
  ascendingText.lookAt(cameraPosition);
  lineOfNodesText.lookAt(cameraPosition);
  xLabel.lookAt(cameraPosition);
  yLabel.lookAt(cameraPosition);
  zLabel.lookAt(cameraPosition);

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
