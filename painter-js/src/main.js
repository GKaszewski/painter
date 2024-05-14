import socket from "./socket.js";
import "./canvas.js";
import "./counter.js";
import { updateCountdown } from "./counter.js";
import { pixelSize } from "./constants.js";

const isDebug = import.meta.env.VITE_IS_DEBUG === "true";

const currentSoldiersSpan = document.getElementById("current-soldiers");

let coords = [];
const canvas = document.getElementById("canvas");
const coordsText = document.getElementById("coords");

socket.on("connect", () => {
  console.log("connect");
});

socket.on("error", (message) => {
  alert(message);
});

socket.on("current_soldiers", (currentSoldiers) => {
  currentSoldiersSpan.textContent = currentSoldiers;
});

requestAnimationFrame(updateCountdown);

window.addEventListener("mousemove", (event) => {
  // get coordinates of the mouse inside the canvas
  const rect = canvas.getBoundingClientRect();
  const x = Math.floor((event.clientX - rect.left) / pixelSize);
  const y = Math.floor((event.clientY - rect.top) / pixelSize);
  coords = [x, y];

  coordsText.textContent = `${x}, ${y}`;
});
