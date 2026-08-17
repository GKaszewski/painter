import { hexToU32 } from "../domain/color.js";
import { canPlacePixel } from "../domain/cooldown.js";
import { setPixel } from "../domain/canvas-state.js";
import { getCanvasCoords } from "../domain/coords.js";

export const createPixelPlacer = ({
  canvas,
  renderer,
  getColor,
  getState,
  socket,
}) => {
  let previewPixel = null;

  const place = (update) => {
    const lastPixelTime = parseInt(
      localStorage.getItem("lastPixelTime") || "0",
    );
    if (!canPlacePixel(lastPixelTime)) return;

    socket.emit("place-pixel", update);
    setPixel(getState(), update.x, update.y, update.color);
    localStorage.setItem("lastPixelTime", Date.now().toString());
  };

  const clearPreview = () => {
    if (!previewPixel) return;
    renderer.restorePixel(getState(), previewPixel.x, previewPixel.y);
    previewPixel = null;
  };

  const confirmPlacement = () => {
    if (!previewPixel) return;
    place(previewPixel);
    clearPreview();
  };

  const selectPixel = (event) => {
    const { x, y } = getCanvasCoords(event, canvas);
    const color = hexToU32(getColor());

    clearPreview();
    previewPixel = { x, y, color };
    renderer.drawPreview(x, y, color);
  };

  canvas.addEventListener("click", selectPixel);

  window.addEventListener("keydown", (event) => {
    if (event.key === "Enter") confirmPlacement();
    if (event.key === "Escape") clearPreview();
  });

  document
    .getElementById("place-pixel")
    .addEventListener("click", confirmPlacement);
};
