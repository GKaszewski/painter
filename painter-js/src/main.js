import { createSocketConnection } from "./infrastructure/socket-client.js";
import { checkServer } from "./infrastructure/api.js";
import { createCanvasRenderer } from "./ui/canvas-renderer.js";
import { createColorPalette } from "./ui/color-palette.js";
import { createPixelPlacer } from "./ui/pixel-placer.js";
import { startCooldownDisplay } from "./ui/cooldown-display.js";
import { createCanvasViewport } from "./ui/canvas-viewport.js";
import { setPixel } from "./domain/canvas-state.js";
import { getCanvasCoords } from "./domain/coords.js";

const canvasEl = document.getElementById("canvas");
const coordsText = document.getElementById("coords");
const currentSoldiersSpan = document.getElementById("current-soldiers");
const statusEl = document.getElementById("connection-status");

const savedDisplay = canvasEl.style.display;
canvasEl.style.display = "none";

let canvasState = [];

const renderer = createCanvasRenderer(canvasEl);
const palette = createColorPalette();

startCooldownDisplay();
createCanvasViewport(canvasEl);

canvasEl.addEventListener("mousemove", (event) => {
  const { x, y } = getCanvasCoords(event, canvasEl);
  coordsText.textContent = `${x}, ${y}`;
});

document.getElementById("save-canvas").addEventListener("click", () => {
  const a = document.createElement("a");
  a.href = renderer.toDataURL();
  a.download = "canvas.png";
  a.click();
});

const showStatus = (message, isError) => {
  if (!statusEl) return;
  statusEl.textContent = message;
  statusEl.className = isError
    ? "text-red-500 text-sm"
    : "text-green-500 text-sm";
};

checkServer()
  .then((response) => {
    if (!response.ok) throw new Error("Server unavailable");

    const socket = createSocketConnection();

    socket.on("connect", () => {
      canvasEl.style.display = savedDisplay;
      showStatus("Connected", false);
    });

    socket.on("canvas_state", (data) => {
      canvasState = data;
      renderer.drawState(data);
    });

    socket.on("error", (message) => showStatus(message, true));

    socket.on("current_soldiers", (count) => {
      currentSoldiersSpan.textContent = count;
    });

    socket.on("pixel-updated", (update) => {
      renderer.drawPixel(update.x, update.y, update.color);
      setPixel(canvasState, update.x, update.y, update.color);
    });

    socket.on("disconnect", () => {
      showStatus("Disconnected — reconnecting...", true);
    });

    createPixelPlacer({
      canvas: canvasEl,
      renderer,
      getColor: palette.getColor,
      getState: () => canvasState,
      socket,
    });
  })
  .catch(() => {
    showStatus("Cannot connect to server", true);
  });
