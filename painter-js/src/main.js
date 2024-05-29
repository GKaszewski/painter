import { connectToWS } from "./socket.js";
import "./canvas.js";
import "./counter.js";
import { updateCountdown } from "./counter.js";
import { checkEndpoint, pixelSize } from "./constants.js";
import { handleSocketEvents } from "./canvas.js";
import "./challenge.js"

const isDebug = import.meta.env.VITE_IS_DEBUG === "true";

const currentSoldiersSpan = document.getElementById("current-soldiers");

let coords = [];
const canvas = document.getElementById("canvas");
const coordsText = document.getElementById("coords");
const ogCanvasStyle = canvas.style.display;
canvas.style.display = "none";

fetch(checkEndpoint)
  .then((response) => {
    if (response.ok) {
      const socket = connectToWS();

      socket.on("connect", () => {
        canvas.style.display = ogCanvasStyle;
        console.log("connect");
      });

      socket.on("error", (message) => {
        alert(message);
      });

      socket.on("current_soldiers", (currentSoldiers) => {
        currentSoldiersSpan.textContent = currentSoldiers;
      });

      handleSocketEvents(socket);

      requestAnimationFrame(updateCountdown);

      window.addEventListener("mousemove", (event) => {
        // get coordinates of the mouse inside the canvas
        const rect = canvas.getBoundingClientRect();
        const x = Math.floor((event.clientX - rect.left) / pixelSize);
        const y = Math.floor((event.clientY - rect.top) / pixelSize);
        coords = [x, y];

        coordsText.textContent = `${x}, ${y}`;
      });
    } else {
      throw new Error("Can't connect to the server");
    }
  })
  .catch((error) => {
    alert(
      "You have already connected to the server from another tab or window. Please close the other tab or window and refresh this page."
    );
  });
