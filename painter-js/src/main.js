import io from 'socket.io-client';

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const colorPicker = document.getElementById('color-picker');
const countdownDiv = document.getElementById('countdown');
const pixelSize = 10;
const pixelCooldown = 60 * 1000; // 1 minute

let lastPixelTime = parseInt(localStorage.getItem('lastPixelTime') || '0');

const socket = io('ws://localhost:3000');

socket.on('connect', () => {
  console.log('connect');
});

socket.on('init-canvas', (data) => {
  const canvasData = JSON.parse(data);
  for (let y = 0; y < canvasData.length; y++) {
    for (let x = 0; x < canvasData[y].length; x++) {
      const color = u32ToHex(canvasData[y][x]);
      ctx.fillStyle = color;
      ctx.fillRect(x * pixelSize, y * pixelSize, pixelSize, pixelSize);
    }
  }
});

socket.on('pixel-updated', (update) => {
  const color = u32ToHex(update.color);
  ctx.fillStyle = color;
  ctx.fillRect(
    update.x * pixelSize,
    update.y * pixelSize,
    pixelSize,
    pixelSize
  );
});

socket.on('error', (message) => {
  alert(message);
});

canvas.addEventListener('click', (event) => {
  const now = Date.now();

  if (now - lastPixelTime < pixelCooldown) {
    alert(
      `Please wait ${Math.round(
        (pixelCooldown - (now - lastPixelTime)) / 1000
      )} more seconds before placing a new pixel.`
    );
    return;
  }

  const rect = canvas.getBoundingClientRect();
  const x = Math.floor((event.clientX - rect.left) / pixelSize);
  const y = Math.floor((event.clientY - rect.top) / pixelSize);
  // random color
  const color = hexToU32(colorPicker.value);

  const update = { x, y, color };
  socket.emit('place-pixel', update);

  lastPixelTime = now;
  localStorage.setItem('lastPixelTime', now.toString());
});

const u32ToHex = (color) => {
  return `#${color.toString(16).padStart(6, '0')}`;
};

const hexToU32 = (color) => {
  return parseInt(color.slice(1), 16);
};

setInterval(() => {
  const now = Date.now();
  const timeLeft = Math.max(0, pixelCooldown - (now - lastPixelTime));
  if (timeLeft > 0) {
    countdownDiv.textContent = `You can place a pixel in ${Math.ceil(
      timeLeft / 1000
    )} seconds`;
  } else {
    countdownDiv.textContent = 'You can place a pixel now';
  }
}, 1000);
