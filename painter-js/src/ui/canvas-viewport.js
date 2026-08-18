const MIN_ZOOM = 1;
const MAX_ZOOM = 40;

export const createCanvasViewport = ({
  canvas,
  viewport,
  zoomIn,
  zoomOut,
  zoomLabel,
}) => {
  let zoom = 1;
  let baseSize = viewport.clientWidth;

  const applyZoom = () => {
    const size = baseSize * zoom;
    canvas.style.width = `${size}px`;
    canvas.style.height = `${size}px`;
    if (zoomLabel) zoomLabel.textContent = `${zoom.toFixed(1)}x`;
  };

  const setZoom = (newZoom, centerX, centerY) => {
    const oldZoom = zoom;
    zoom = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, newZoom));
    if (zoom === oldZoom) return;

    if (centerX !== undefined && centerY !== undefined) {
      const ratio = zoom / oldZoom;
      viewport.scrollLeft = (viewport.scrollLeft + centerX) * ratio - centerX;
      viewport.scrollTop = (viewport.scrollTop + centerY) * ratio - centerY;
    }

    applyZoom();
  };

  viewport.addEventListener("wheel", (event) => {
    event.preventDefault();
    const factor = event.deltaY > 0 ? 0.8 : 1.25;
    const rect = viewport.getBoundingClientRect();
    setZoom(zoom * factor, event.clientX - rect.left, event.clientY - rect.top);
  });

  let lastPinchDist = 0;
  viewport.addEventListener("touchmove", (event) => {
    if (event.touches.length !== 2) return;
    event.preventDefault();
    const dist = Math.hypot(
      event.touches[0].clientX - event.touches[1].clientX,
      event.touches[0].clientY - event.touches[1].clientY,
    );
    if (lastPinchDist > 0) {
      const midX =
        (event.touches[0].clientX + event.touches[1].clientX) / 2 -
        viewport.getBoundingClientRect().left;
      const midY =
        (event.touches[0].clientY + event.touches[1].clientY) / 2 -
        viewport.getBoundingClientRect().top;
      setZoom(zoom * (dist / lastPinchDist), midX, midY);
    }
    lastPinchDist = dist;
  });

  viewport.addEventListener("touchend", () => {
    lastPinchDist = 0;
  });

  zoomIn?.addEventListener("click", () => {
    setZoom(zoom * 1.5);
    applyZoom();
  });

  zoomOut?.addEventListener("click", () => {
    setZoom(zoom / 1.5);
    applyZoom();
  });

  window.addEventListener("resize", () => {
    baseSize = viewport.clientWidth;
    applyZoom();
  });

  applyZoom();
};
