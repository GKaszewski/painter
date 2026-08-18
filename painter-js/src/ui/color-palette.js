import { rgbToHex } from "../domain/color.js";

const PALETTE_IDS = [
  "red",
  "green",
  "blue",
  "yellow",
  "purple",
  "pink",
  "cyan",
  "white",
  "black",
  "orange",
  "brown",
];

export const createColorPalette = ({ colorPicker, colorSpan, appState }) => {
  let activeButton = null;

  const color = appState.get("selectedColor");
  colorPicker.value = color;
  colorSpan.style.backgroundColor = color;

  const setColor = (hex, button) => {
    appState.set("selectedColor", hex);
    colorSpan.textContent = hex;
    colorSpan.style.backgroundColor = hex;

    if (activeButton)
      activeButton.classList.remove("ring-2", "ring-offset-2", "ring-cyan-400");
    if (button) {
      button.classList.add("ring-2", "ring-offset-2", "ring-cyan-400");
      activeButton = button;
    }
  };

  colorPicker.addEventListener("input", (e) => setColor(e.target.value, null));

  for (const id of PALETTE_IDS) {
    const button = document.getElementById(id);
    button.addEventListener("click", () => {
      setColor(
        rgbToHex(window.getComputedStyle(button).backgroundColor),
        button,
      );
    });
  }
};
