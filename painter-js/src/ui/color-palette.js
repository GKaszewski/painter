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

export const createColorPalette = () => {
  const colorPicker = document.getElementById("color-picker");
  const currentColorSpan = document.getElementById("current-color-span");
  let activeButton = null;

  let currentColor = localStorage.getItem("currentColor") || "#000000";
  colorPicker.value = currentColor;
  currentColorSpan.style.backgroundColor = currentColor;

  const setColor = (hex, button) => {
    currentColor = hex;
    currentColorSpan.textContent = hex;
    currentColorSpan.style.backgroundColor = hex;
    localStorage.setItem("currentColor", hex);

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

  return { getColor: () => currentColor };
};
