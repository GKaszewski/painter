import { u32ToHex, u32ToRGBA } from "../domain/color.js";
import { WIDTH, HEIGHT } from "../domain/constants.js";

export const createCanvasRenderer = (canvas) => {
  const ctx = canvas.getContext("2d");

  const drawState = (state) => {
    const imageData = ctx.createImageData(WIDTH, HEIGHT);
    const data = imageData.data;
    for (let i = 0; i < state.length; i++) {
      const color = state[i];
      const offset = i * 4;
      data[offset] = (color >> 16) & 0xff;
      data[offset + 1] = (color >> 8) & 0xff;
      data[offset + 2] = color & 0xff;
      data[offset + 3] = 255;
    }
    ctx.putImageData(imageData, 0, 0);
  };

  const drawPixel = (x, y, colorU32) => {
    ctx.fillStyle = u32ToHex(colorU32);
    ctx.fillRect(x, y, 1, 1);
  };

  const drawPreview = (x, y, colorU32) => {
    ctx.fillStyle = u32ToRGBA(colorU32, 0.5);
    ctx.fillRect(x, y, 1, 1);
  };

  const restorePixel = (state, x, y) => {
    ctx.fillStyle = u32ToHex(state[y * WIDTH + x]);
    ctx.fillRect(x, y, 1, 1);
  };

  const toDataURL = () => canvas.toDataURL();

  return { drawState, drawPixel, drawPreview, restorePixel, toDataURL };
};
