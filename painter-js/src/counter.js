import { pixelCooldown } from "./constants.js";

const countdownDiv = document.getElementById("countdown");

export const updateCountdown = () => {
  setInterval(() => {
    const lastPixelTime = parseInt(
      localStorage.getItem("lastPixelTime") || "0"
    );
    const now = Date.now();
    const timeLeft = Math.max(0, pixelCooldown - (now - lastPixelTime));
    if (timeLeft > 0) {
      countdownDiv.textContent = `You can place a pixel in ${Math.ceil(
        timeLeft / 1000
      )} seconds`;
    } else {
      countdownDiv.textContent = "You can place a pixel now";
    }
  }, 1000);
};
