import { WIDTH, HEIGHT } from "./constants.js";

export const getCanvasCoords = (event, canvas) => {
  const rect = canvas.getBoundingClientRect();
  const clientX = event.touches ? event.touches[0].clientX : event.clientX;
  const clientY = event.touches ? event.touches[0].clientY : event.clientY;
  const scaleX = canvas.width / rect.width;
  const scaleY = canvas.height / rect.height;
  return {
    x: Math.min(
      Math.max(Math.floor((clientX - rect.left) * scaleX), 0),
      WIDTH - 1,
    ),
    y: Math.min(
      Math.max(Math.floor((clientY - rect.top) * scaleY), 0),
      HEIGHT - 1,
    ),
  };
};
