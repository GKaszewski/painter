import {
  hexToU32,
  u32ToHex,
  getColorFromElementCSS,
  rgbToHex,
} from "./utils.js";
import {
  pixelSize,
  pixelCooldown,
  canvasEndpoint,
  WIDTH,
  HEIGHT,
} from "./constants.js";

let socket = null;

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");
const countdownDiv = document.getElementById("countdown");
let lastPixelTime = parseInt(localStorage.getItem("lastPixelTime") || "0");

const colorPicker = document.getElementById("color-picker");
const redButton = document.getElementById("red");
const greenButton = document.getElementById("green");
const blueButton = document.getElementById("blue");
const yellowButton = document.getElementById("yellow");
const purpleButton = document.getElementById("purple");
const pinkButton = document.getElementById("pink");
const cyanButton = document.getElementById("cyan");
const whiteButton = document.getElementById("white");
const blackButton = document.getElementById("black");
const orangeButton = document.getElementById("orange");
const brownButton = document.getElementById("brown");

const currentColorSpan = document.getElementById("current-color-span");
const toggleGridToggle = document.getElementById("toggle-grid");
const placePixelButton = document.getElementById("place-pixel");
const saveCanvasButton = document.getElementById("save-canvas");

colorPicker.value = localStorage.getItem("currentColor") || "#000000";

let currentColor = colorPicker.value;
let showGrid = toggleGridToggle.checked;
currentColorSpan.style.backgroundColor = currentColor;

let canvasState = [];
let confirmPlacePixel = false;
let previewPixel = null;

const setCurrentColor = (color, isColorPicker = false) => {
  const hexColor = isColorPicker ? color : rgbToHex(color);
  currentColor = hexColor;
  currentColorSpan.style.backgroundColor = hexColor;
  localStorage.setItem("currentColor", hexColor);
};

const handleColorPicker = () => {
  colorPicker.addEventListener("input", (event) => {
    setCurrentColor(event.target.value, true);
  });

  redButton.addEventListener("click", () => {
    setCurrentColor(getColorFromElementCSS(redButton));
  });

  greenButton.addEventListener("click", () => {
    setCurrentColor(getColorFromElementCSS(greenButton));
  });

  blueButton.addEventListener("click", () => {
    setCurrentColor(getColorFromElementCSS(blueButton));
  });

  yellowButton.addEventListener("click", () => {
    setCurrentColor(getColorFromElementCSS(yellowButton));
  });

  purpleButton.addEventListener("click", () => {
    setCurrentColor(getColorFromElementCSS(purpleButton));
  });

  pinkButton.addEventListener("click", () => {
    setCurrentColor(getColorFromElementCSS(pinkButton));
  });

  cyanButton.addEventListener("click", () => {
    setCurrentColor(getColorFromElementCSS(cyanButton));
  });

  whiteButton.addEventListener("click", () => {
    setCurrentColor(getColorFromElementCSS(whiteButton));
  });

  blackButton.addEventListener("click", () => {
    setCurrentColor(getColorFromElementCSS(blackButton));
  });

  orangeButton.addEventListener("click", () => {
    setCurrentColor(getColorFromElementCSS(orangeButton));
  });

  brownButton.addEventListener("click", () => {
    setCurrentColor(getColorFromElementCSS(brownButton));
  });
};

const fetchCanvasState = async () => {
  fetch(canvasEndpoint)
    .then((response) => response.json())
    .then((data) => {
      canvasState = data;
      drawCanvasState(data);
    })
    .catch((error) => {
      alert("Error fetching canvas state from server. Please try again later.");
    });
};

const drawCanvasState = (canvasState) => {
  for (let y = 0; y < HEIGHT; y++) {
    for (let x = 0; x < WIDTH; x++) {
      const index = y * WIDTH + x;
      const color = u32ToHex(canvasState[index]);
      ctx.fillStyle = color;
      ctx.fillRect(x * pixelSize, y * pixelSize, pixelSize, pixelSize);
    }
  }
};

const checkIfCanPlacePixel = () => {
  const now = Date.now();
  return now - lastPixelTime >= pixelCooldown;
};

const setLastPixelTime = () => {
  lastPixelTime = Date.now();
  localStorage.setItem("lastPixelTime", lastPixelTime.toString());
};

const handlePlacePixel = (pixelData) => {
  if (!checkIfCanPlacePixel()) {
    alert("You can't place a pixel yet");
    return;
  }

  socket.emit("place-pixel", pixelData);
  const index = pixelData.y * WIDTH + pixelData.x;
  canvasState[index] = pixelData.color;
  setLastPixelTime();
  pixelData = null;
};

canvas.addEventListener("click", (event) => {
  const rect = canvas.getBoundingClientRect();
  const x = Math.floor((event.clientX - rect.left) / pixelSize);
  const y = Math.floor((event.clientY - rect.top) / pixelSize);

  const color = hexToU32(currentColor);
  const oldPreviewPixel = previewPixel;

  const update = { x, y, color };

  if (confirmPlacePixel) {
    handlePlacePixel(update);
  } else {
    previewPixel = update;
  }

  if (previewPixel) {
    if (oldPreviewPixel) {
      ctx.clearRect(
        oldPreviewPixel.x * pixelSize,
        oldPreviewPixel.y * pixelSize,
        pixelSize,
        pixelSize
      );
    }

    ctx.fillStyle = `rgba(${(color >> 16) & 0xff}, ${(color >> 8) & 0xff}, ${
      color & 0xff
    }, 0.5)`;
    ctx.fillRect(x * pixelSize, y * pixelSize, pixelSize, pixelSize);
  }
});

const removePreviewPixel = () => {
  previewPixel = null;
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  drawCanvasState(canvasState);
};

const drawGrid = () => {
  ctx.strokeStyle = "#000";
  for (let x = 0; x < canvas.width; x += pixelSize) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvas.height);
    ctx.stroke();
  }
  for (let y = 0; y < canvas.height; y += pixelSize) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(canvas.width, y);
    ctx.stroke();
  }
};

const handleToggleGrid = () => {
  toggleGridToggle.addEventListener("change", (event) => {
    showGrid = event.target.checked;
    localStorage.setItem("showGrid", showGrid);
    if (showGrid) {
      drawGrid();
    } else {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      drawCanvasState(canvasState);
    }
  });
};

window.onkeydown = (event) => {
  // on enter (keycode 13 is enter)
  if (event.keyCode === 13) {
    if (previewPixel) {
      handlePlacePixel(previewPixel);
      removePreviewPixel();
    }
  }
};

placePixelButton.addEventListener("click", () => {
  if (previewPixel) {
    handlePlacePixel(previewPixel);
    removePreviewPixel();
  }
});

saveCanvasButton.addEventListener("click", () => {
  const a = document.createElement("a");
  a.href = canvas.toDataURL();
  a.download = "canvas.png";
  a.click();
});

handleColorPicker();
handleToggleGrid();

export const handleSocketEvents = (_socket) => {
  socket = _socket;
  socket.on("connect", () => {
    fetchCanvasState();
  });

  socket.on("pixel-updated", (update) => {
    const color = u32ToHex(update.color);
    ctx.fillStyle = color;
    ctx.fillRect(
      update.x * pixelSize,
      update.y * pixelSize,
      pixelSize,
      pixelSize
    );

    const index = update.y * WIDTH + update.x;
    canvasState[index] = update.color;
  });
};
