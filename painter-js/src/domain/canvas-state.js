import { WIDTH } from "./constants.js";

export const getPixelIndex = (x, y) => y * WIDTH + x;

export const setPixel = (state, x, y, color) => {
  state[getPixelIndex(x, y)] = color;
};

export const getPixel = (state, x, y) => {
  return state[getPixelIndex(x, y)];
};
