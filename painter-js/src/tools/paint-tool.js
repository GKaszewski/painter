import { hexToU32 } from "../domain/color.js";
import { canPlacePixel } from "../domain/cooldown.js";
import { setPixel } from "../domain/canvas-state.js";
import { getCanvasCoords } from "../domain/coords.js";

export const createPaintTool = ({ canvas, renderer, appState, bus }) => {
  let previewPixel = null;

  const clearPreview = () => {
    if (!previewPixel) return;
    renderer.restorePixel(
      appState.get("canvasPixels"),
      previewPixel.x,
      previewPixel.y,
    );
    previewPixel = null;
  };

  return {
    name: "paint",
    cursor: "crosshair",
    onClick: (event) => {
      const { x, y } = getCanvasCoords(event, canvas);
      const color = hexToU32(appState.get("selectedColor"));
      clearPreview();
      previewPixel = { x, y, color };
      renderer.drawPreview(x, y, color);
    },
    confirm: () => {
      if (!previewPixel) return;
      if (!canPlacePixel(appState.get("lastPlacementTime"))) return;

      bus.emit("pixel-placed", previewPixel);
      setPixel(
        appState.get("canvasPixels"),
        previewPixel.x,
        previewPixel.y,
        previewPixel.color,
      );
      appState.set("lastPlacementTime", Date.now());
      clearPreview();
    },
    cancel: () => clearPreview(),
    onDeactivate: () => clearPreview(),
  };
};
