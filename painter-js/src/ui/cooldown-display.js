import { timeRemaining } from "../domain/cooldown.js";

export const startCooldownDisplay = () => {
  const countdownDiv = document.getElementById("countdown");

  setInterval(() => {
    const lastPlacementTime = parseInt(
      localStorage.getItem("lastPixelTime") || "0",
    );
    const remaining = timeRemaining(lastPlacementTime);
    countdownDiv.textContent =
      remaining > 0
        ? `You can place a pixel in ${Math.ceil(remaining / 1000)} seconds`
        : "You can place a pixel now";
  }, 1000);
};
