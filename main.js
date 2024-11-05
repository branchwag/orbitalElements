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

const arrowGroup = new THREE.Group();


const axesGroup = new THREE.Group();
axesGroup.rotation.x = THREE.MathUtils.degToRad(23.5);
scene.add(axesGroup);

const axisLength = 4;
//const axisWidth = 2;

const zAxisGeometry = new THREE.CylinderGeometry(0.03, 0.03, axisLength, 8);
const zAxisMaterial = new THREE.MeshBasicMaterial({ color: 0x0000FF });
const zAxis = new THREE.Mesh(zAxisGeometry, zAxisMaterial);
zAxis.position.y = axisLength / 2;
axesGroup.add(zAxis);

const zArrowGeometry = new THREE.ConeGeometry(0.1, 0.3, 8);
const zArrow = new THREE.Mesh(zArrowGeometry, zAxisMaterial);
zArrow.position.y = axisLength;
axesGroup.add(zArrow);

const yAxisGeometry = new THREE.CylinderGeometry(0.03, 0.03, axisLength, 8);
const yAxisMaterial = new THREE.MeshBasicMaterial({ color: 0xFF0000 });
const yAxis = new THREE.Mesh(yAxisGeometry, yAxisMaterial);
yAxis.rotation.z = THREE.MathUtils.degToRad(-90);
yAxis.position.x = axisLength / 2;
axesGroup.add(yAxis);

const yArrowGeometry = new THREE.ConeGeometry(0.1, 0.3, 8);
const yArrow = new THREE.Mesh(yArrowGeometry, yAxisMaterial);
yArrow.rotation.z = THREE.MathUtils.degToRad(-90);
yArrow.position.x = axisLength;
axesGroup.add(yArrow);

const xAxisGeometry = new THREE.CylinderGeometry(0.03, 0.03, axisLength, 8);
const xAxisMaterial = new THREE.MeshBasicMaterial({ color: 0x00FF00 });
const xAxis = new THREE.Mesh(xAxisGeometry, xAxisMaterial);
xAxis.rotation.x = THREE.MathUtils.degToRad(90);
xAxis.position.z = axisLength / 2;
axesGroup.add(xAxis);

const xArrowGeometry = new THREE.ConeGeometry(0.1, 0.3, 8);
const xArrow = new THREE.Mesh(xArrowGeometry, xAxisMaterial);
xArrow.rotation.x = THREE.MathUtils.degToRad(90);
xArrow.position.z = axisLength;
axesGroup.add(xArrow);


//pointer arrow
const arrowLineGeometry = new THREE.CylinderGeometry(0.03, 0.03, 1.8, 8);
const arrowMaterial = new THREE.MeshBasicMaterial({ color: 0xFFA500 });
const arrowPointGeometry = new THREE.ConeGeometry(0.1, 0.3, 8);
const arrowPoint = new THREE.Mesh(arrowPointGeometry, arrowMaterial);
const arrowLine = new THREE.Mesh(arrowLineGeometry, arrowMaterial);
arrowPoint.position.y = 1;
arrowGroup.add(arrowPoint);
arrowGroup.add(arrowLine);
arrowGroup.position.x = 3.8;
arrowGroup.position.y = 2.5;
arrowGroup.position.z = -1;
arrowGroup.rotation.x = THREE.MathUtils.degToRad(160);
arrowGroup.rotation.z = THREE.MathUtils.degToRad(30);
scene.add(arrowGroup);


//axis labels
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

//planes
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

//https://threejs.org/docs/index.html?q=arc#api/en/extras/curves/EllipseCurve
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

const startPoint = new THREE.Vector3(3.5, 0, 0);
const endPoint = new THREE.Vector3(0, -1.6, axisLength - 0.3);
const controlPoint = new THREE.Vector3(
  startPoint.x / 1.5,
  -3, //height
  axisLength - 1.2,
);

const curvePoints = [];
const segments = 32;
for (let i = 0; i <= segments; i++) {
  const t = i / segments;
  // Quadratic Bezier curve calculations
  const x = Math.pow(1 - t, 2) * startPoint.x + 2 * (1 - t) * t * controlPoint.x + Math.pow(t, 2) * endPoint.x;
  const y = Math.pow(1 - t, 2) * startPoint.y + 2 * (1 - t) * t * controlPoint.y + Math.pow(t, 2) * endPoint.y;
  const z = Math.pow(1 - t, 2) * startPoint.z + 2 * (1 - t) * t * controlPoint.z + Math.pow(t, 2) * endPoint.z;
  curvePoints.push(new THREE.Vector3(x, y, z));
}
const raanGeometry = new THREE.BufferGeometry().setFromPoints(curvePoints);
const raanMaterial = new THREE.LineBasicMaterial({
  color: 0xFFA500,
  linewidth: 2
});
const raanArc = new THREE.Line(raanGeometry, raanMaterial);
scene.add(raanArc);

const inclinationArcPoints = [];
const inclinationSegments = 32;

const equatorialPoint = new THREE.Vector3(
  3.5,
  3.5 * Math.cos(THREE.MathUtils.degToRad(115)),
  3.5 * Math.sin(THREE.MathUtils.degToRad(115))
);

const orbitalPoint = new THREE.Vector3(
  3.5 * Math.cos(THREE.MathUtils.degToRad(45)),
  4.5 * Math.sin(THREE.MathUtils.degToRad(45)) * Math.cos(THREE.MathUtils.degToRad(160)),
  3.5 * Math.sin(THREE.MathUtils.degToRad(160))
);

const startPointInc = equatorialPoint;
const endPointInc = orbitalPoint;
const controlPointInc = new THREE.Vector3(
  4.5,
  (startPointInc.y + endPointInc.y) / 2,
  (startPointInc.z + endPointInc.z) / 2
);

for (let i = 0; i <= inclinationSegments; i++) {
  const t = i / inclinationSegments;
  const x = Math.pow(1 - t, 2) * startPointInc.x + 2 * (1 - t) * t * controlPointInc.x + Math.pow(t, 2) * endPointInc.x;
  const y = Math.pow(1 - t, 2) * startPointInc.y + 2 * (1 - t) * t * controlPointInc.y + Math.pow(t, 2) * endPointInc.y;
  const z = Math.pow(1 - t, 2) * startPointInc.z + 2 * (1 - t) * t * controlPointInc.z + Math.pow(t, 2) * endPointInc.z;
  inclinationArcPoints.push(new THREE.Vector3(x, y, z));
}

const inclinationGeometry = new THREE.BufferGeometry().setFromPoints(inclinationArcPoints);
const inclinationMaterial = new THREE.LineBasicMaterial({
  color: 0x800080,
  linewidth: 3
});
const inclinationArc = new THREE.Line(inclinationGeometry, inclinationMaterial);
scene.add(inclinationArc);

const nodeGeometry = new THREE.SphereGeometry(0.1, 16, 16);
const nodeMaterial = new THREE.MeshBasicMaterial({ color: 0xFFFF00 });
const perigeeMaterial = new THREE.MeshBasicMaterial({ color: 0xFFFFFF });

const descendingNode = new THREE.Mesh(nodeGeometry, nodeMaterial);
descendingNode.position.set(-3.5, 0, 0);
scene.add(descendingNode);

const ascendingNode = new THREE.Mesh(nodeGeometry, nodeMaterial);
ascendingNode.position.set(3.5, 0, 0);
scene.add(ascendingNode);

const perigeePoint = new THREE.Mesh(nodeGeometry, perigeeMaterial);
perigeePoint.position.set(2.75, 2, -0.75);
scene.add(perigeePoint);

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

const raanLabelPos = new THREE.Vector3(
  startPoint.x / 2,
  -2, //height
  (axisLength - 0.3) / 3
);

const inclinationText = createLabelSprite('Inclination', new THREE.Vector3(3, -2, 2));

const perigeeText = createLabelSprite('Perigee', new THREE.Vector3(2.6, 2.6, -0.5));
const argPerigeeText = createLabelSprite('Argument of Perigee', new THREE.Vector3(5.3, 2.5, -0.5));
const raanText = createLabelSprite('RAAN', raanLabelPos);

const descendingText = createLabelSprite('Descending Node', new THREE.Vector3(-3.5, 0.5, 0));
const ascendingText = createLabelSprite('Ascending Node', new THREE.Vector3(4.5, 0.5, 0));
const lineOfNodesText = createLabelSprite('Line of Nodes', new THREE.Vector3(0, -0.5, 0));
scene.add(inclinationText);
scene.add(perigeeText);
scene.add(argPerigeeText);
scene.add(raanText);
scene.add(descendingText);
scene.add(ascendingText);
scene.add(lineOfNodesText);

//const earthRotationSpeed = (2 * Math.PI) / 86400; //radians per second for 24hr rotation

function animate() {
  const currentTime = performance.now();
  //const deltaTime = (currentTime - lastTime) / 1000; //convert to seconds
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
  raanText.material.rotation = 0;

  descendingText.lookAt(cameraPosition);
  ascendingText.lookAt(cameraPosition);
  lineOfNodesText.lookAt(cameraPosition);
  raanText.lookAt(cameraPosition);
  perigeeText.lookAt(cameraPosition);
  argPerigeeText.lookAt(cameraPosition);
  xLabel.lookAt(cameraPosition);
  yLabel.lookAt(cameraPosition);
  zLabel.lookAt(cameraPosition);
  inclinationText.lookAt(cameraPosition);

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
