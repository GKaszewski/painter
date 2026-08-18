import { timeRemaining } from "../domain/cooldown.js";

let audioCtx = null;

const playBeep = () => {
  try {
    if (!audioCtx) audioCtx = new AudioContext();
    const osc = audioCtx.createOscillator();
    const gain = audioCtx.createGain();
    osc.connect(gain);
    gain.connect(audioCtx.destination);
    osc.type = "sine";
    osc.frequency.setValueAtTime(523, audioCtx.currentTime);
    osc.frequency.setValueAtTime(659, audioCtx.currentTime + 0.1);
    gain.gain.setValueAtTime(0.2, audioCtx.currentTime);
    gain.gain.exponentialRampToValueAtTime(0.001, audioCtx.currentTime + 0.25);
    osc.start();
    osc.stop(audioCtx.currentTime + 0.25);
  } catch (e) {}
};

export const startCooldownDisplay = ({ countdown, placeBtn, appState }) => {
  let wasCoolingDown = false;

  const update = () => {
    const lastPlacementTime = appState.get("lastPlacementTime");
    const remaining = timeRemaining(lastPlacementTime);
    const isCoolingDown = remaining > 0;

    countdown.textContent = isCoolingDown
      ? `You can place a pixel in ${Math.ceil(remaining / 1000)} seconds`
      : "You can place a pixel now";

    placeBtn.disabled = isCoolingDown;
    if (isCoolingDown) {
      placeBtn.classList.add("opacity-50", "cursor-not-allowed");
    } else {
      placeBtn.classList.remove("opacity-50", "cursor-not-allowed");
    }

    if (wasCoolingDown && !isCoolingDown && lastPlacementTime > 0) {
      playBeep();
    }

    wasCoolingDown = isCoolingDown;
  };

  update();
  setInterval(update, 1000);
  appState.subscribe("lastPlacementTime", update);
};
