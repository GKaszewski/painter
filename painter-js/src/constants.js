const isDebug = import.meta.env.VITE_IS_DEBUG === "true";

export const pixelSize = 10;
export const pixelCooldown = 10 * 1000; // 10 seconds
export const canvasEndpoint = isDebug
  ? "http://localhost:3000/canvas/"
  : "/canvas/";
export const checkEndpoint = isDebug
  ? "http://localhost:3000/check/"
  : "/check/";
export const WIDTH = 500;
export const HEIGHT = 500;
