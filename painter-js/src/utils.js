export const u32ToHex = (color) => {
  return `#${color.toString(16).padStart(6, "0")}`;
};

export const hexToU32 = (color) => {
  return parseInt(color.slice(1), 16);
};

export const getColorFromElementCSS = (element) => {
  return window.getComputedStyle(element).backgroundColor;
};

export const rgbToHex = (rgbProperty) => {
  const rgb = rgbProperty.match(/\d+/g);
  return `#${rgb
    .map((x) => parseInt(x).toString(16).padStart(2, "0"))
    .join("")}`;
};
