import { PIXEL_COOLDOWN } from "./constants.js";

export const canPlacePixel = (lastPlacementTime) => {
  return Date.now() - lastPlacementTime >= PIXEL_COOLDOWN;
};

export const timeRemaining = (lastPlacementTime) => {
  return Math.max(0, PIXEL_COOLDOWN - (Date.now() - lastPlacementTime));
};
