export const u32ToHex = (color) => {
  return `#${color.toString(16).padStart(6, "0")}`;
};

export const hexToU32 = (color) => {
  return parseInt(color.slice(1), 16);
};

export const rgbToHex = (rgbProperty) => {
  const rgb = rgbProperty.match(/\d+/g);
  return `#${rgb
    .map((x) => parseInt(x).toString(16).padStart(2, "0"))
    .join("")}`;
};

export const u32ToRGBA = (color, alpha) => {
  const r = (color >> 16) & 0xff;
  const g = (color >> 8) & 0xff;
  const b = color & 0xff;
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
};
