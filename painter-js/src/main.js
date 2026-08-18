import { createSocketConnection } from "./infrastructure/socket-client.js";
import { checkServer } from "./infrastructure/api.js";
import { createEventBus } from "./infrastructure/event-bus.js";
import { createKeyboard } from "./infrastructure/keyboard.js";
import { createAppState } from "./domain/app-state.js";
import { createCanvasRenderer } from "./ui/canvas-renderer.js";
import { createColorPalette } from "./ui/color-palette.js";
import { startCooldownDisplay } from "./ui/cooldown-display.js";
import { createCanvasViewport } from "./ui/canvas-viewport.js";
import { createToolManager } from "./ui/tool-manager.js";
import { createPaintTool } from "./tools/paint-tool.js";
import { createHandTool } from "./tools/hand-tool.js";
import { setPixel } from "./domain/canvas-state.js";
import { getCanvasCoords } from "./domain/coords.js";

const els = {
  canvas: document.getElementById("canvas"),
  viewport: document.getElementById("canvas-viewport"),
  status: document.getElementById("connection-status"),
  countdown: document.getElementById("countdown"),
  placeBtn: document.getElementById("place-pixel"),
  soldiers: document.getElementById("current-soldiers"),
  coords: document.getElementById("coords"),
  zoomIn: document.getElementById("zoom-in"),
  zoomOut: document.getElementById("zoom-out"),
  zoomLabel: document.getElementById("zoom-level"),
  modePaint: document.getElementById("mode-paint"),
  modeHand: document.getElementById("mode-hand"),
  colorPicker: document.getElementById("color-picker"),
  colorSpan: document.getElementById("current-color-span"),
  saveCanvas: document.getElementById("save-canvas"),
};

const savedDisplay = els.canvas.style.display;
els.canvas.style.display = "none";

const appState = createAppState({
  canvasPixels: [],
  lastPlacementTime: parseInt(localStorage.getItem("lastPixelTime") || "0"),
  selectedColor: localStorage.getItem("currentColor") || "#000000",
  soldierCount: 0,
});

appState.subscribe("lastPlacementTime", (val) => {
  localStorage.setItem("lastPixelTime", val.toString());
});

appState.subscribe("selectedColor", (val) => {
  localStorage.setItem("currentColor", val);
});

const bus = createEventBus();
const keyboard = createKeyboard();
const renderer = createCanvasRenderer(els.canvas);

createColorPalette({
  colorPicker: els.colorPicker,
  colorSpan: els.colorSpan,
  appState,
});

startCooldownDisplay({
  countdown: els.countdown,
  placeBtn: els.placeBtn,
  appState,
});

createCanvasViewport({
  canvas: els.canvas,
  viewport: els.viewport,
  zoomIn: els.zoomIn,
  zoomOut: els.zoomOut,
  zoomLabel: els.zoomLabel,
});

const updateModeButtons = (name) => {
  els.modePaint.classList.toggle("ring-2", name === "paint");
  els.modePaint.classList.toggle("ring-offset-2", name === "paint");
  els.modePaint.classList.toggle("ring-cyan-400", name === "paint");
  els.modeHand.classList.toggle("ring-2", name === "hand");
  els.modeHand.classList.toggle("ring-offset-2", name === "hand");
  els.modeHand.classList.toggle("ring-cyan-400", name === "hand");
};

const toolManager = createToolManager({
  canvas: els.canvas,
  viewport: els.viewport,
  onActiveChange: updateModeButtons,
});

toolManager.register(
  createPaintTool({ canvas: els.canvas, renderer, appState, bus }),
);
toolManager.register(createHandTool({ viewport: els.viewport }));
toolManager.setActive("paint");

els.modePaint.addEventListener("click", () => toolManager.setActive("paint"));
els.modeHand.addEventListener("click", () => toolManager.setActive("hand"));
els.placeBtn.addEventListener("click", () => toolManager.confirm());

keyboard.register("Enter", "confirm-placement");
keyboard.register("Escape", "cancel-preview");
keyboard.register(" ", "hold-hand-mode", { hold: true });

keyboard.onAction("confirm-placement", () => toolManager.confirm());
keyboard.onAction("cancel-preview", () => toolManager.cancel());
keyboard.onAction("hold-hand-mode:down", () => toolManager.holdTool("hand"));
keyboard.onAction("hold-hand-mode:up", () => toolManager.releaseTool());

els.canvas.addEventListener("mousemove", (event) => {
  const { x, y } = getCanvasCoords(event, els.canvas);
  els.coords.textContent = `${x}, ${y}`;
});

els.saveCanvas.addEventListener("click", () => {
  const a = document.createElement("a");
  a.href = renderer.toDataURL();
  a.download = "canvas.png";
  a.click();
});

const showStatus = (message, isError) => {
  els.status.textContent = message;
  els.status.className = isError
    ? "text-red-500 text-sm"
    : "text-green-500 text-sm";
};

els.soldiers.addEventListener("animationend", () => {
  els.soldiers.classList.remove("soldier-pop");
});

appState.subscribe("soldierCount", (count, prev) => {
  els.soldiers.textContent = count;
  if (count > prev && prev > 0) {
    els.soldiers.classList.remove("soldier-pop");
    void els.soldiers.offsetWidth;
    els.soldiers.classList.add("soldier-pop");
  }
});

bus.on("canvas_state", (data) => {
  appState.set("canvasPixels", data);
  renderer.drawState(data);
});

bus.on("pixel-updated", (update) => {
  renderer.drawPixel(update.x, update.y, update.color);
  setPixel(appState.get("canvasPixels"), update.x, update.y, update.color);
});

bus.on("current_soldiers", (count) => {
  appState.set("soldierCount", count);
});

bus.on("connect", () => {
  els.canvas.style.display = savedDisplay;
  showStatus("Connected", false);
});

bus.on("disconnect", () => {
  showStatus("Disconnected — reconnecting...", true);
});

bus.on("error", (message) => showStatus(message, true));

checkServer()
  .then((response) => {
    if (!response.ok) throw new Error("Server unavailable");

    const socket = createSocketConnection();

    socket.on("connect", () => bus.emit("connect"));
    socket.on("canvas_state", (data) => bus.emit("canvas_state", data));
    socket.on("pixel-updated", (data) => bus.emit("pixel-updated", data));
    socket.on("current_soldiers", (count) => bus.emit("current_soldiers", count));
    socket.on("error", (message) => bus.emit("error", message));
    socket.on("disconnect", () => bus.emit("disconnect"));

    bus.on("pixel-placed", (update) => {
      socket.emit("place-pixel", update);
    });
  })
  .catch(() => {
    showStatus("Cannot connect to server", true);
  });
