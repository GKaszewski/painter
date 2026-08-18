export const createHandTool = ({ viewport }) => {
  let isDragging = false;
  let dragStartX = 0;
  let dragStartY = 0;
  let scrollStartLeft = 0;
  let scrollStartTop = 0;

  return {
    name: "hand",
    cursor: "grab",
    onMouseDown: (event) => {
      isDragging = true;
      dragStartX = event.clientX;
      dragStartY = event.clientY;
      scrollStartLeft = viewport.scrollLeft;
      scrollStartTop = viewport.scrollTop;
      viewport.style.cursor = "grabbing";
      event.preventDefault();
    },
    onMouseMove: (event) => {
      if (!isDragging) return;
      viewport.scrollLeft = scrollStartLeft - (event.clientX - dragStartX);
      viewport.scrollTop = scrollStartTop - (event.clientY - dragStartY);
    },
    onMouseUp: () => {
      if (!isDragging) return;
      isDragging = false;
      viewport.style.cursor = "grab";
    },
    onDeactivate: () => {
      isDragging = false;
    },
  };
};
