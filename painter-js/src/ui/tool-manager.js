export const createToolManager = ({ canvas, viewport, onActiveChange }) => {
  const tools = {};
  let activeName = null;
  let heldName = null;
  let previousName = null;

  const setActive = (name) => {
    if (activeName === name) return;
    const prev = tools[activeName];
    if (prev?.onDeactivate) prev.onDeactivate();
    activeName = name;
    const tool = tools[activeName];
    viewport.style.cursor = tool.cursor || "default";
    if (tool.onActivate) tool.onActivate();
    if (onActiveChange) onActiveChange(name);
  };

  canvas.addEventListener("click", (event) => {
    const tool = tools[activeName];
    if (tool?.onClick) tool.onClick(event);
  });

  viewport.addEventListener("mousedown", (event) => {
    const tool = tools[activeName];
    if (tool?.onMouseDown) tool.onMouseDown(event);
  });

  window.addEventListener("mousemove", (event) => {
    const tool = tools[activeName];
    if (tool?.onMouseMove) tool.onMouseMove(event);
  });

  window.addEventListener("mouseup", (event) => {
    const tool = tools[activeName];
    if (tool?.onMouseUp) tool.onMouseUp(event);
  });

  return {
    register: (tool) => {
      tools[tool.name] = tool;
    },
    setActive,
    getActive: () => activeName,
    holdTool: (name) => {
      if (heldName) return;
      previousName = activeName;
      heldName = name;
      setActive(name);
    },
    releaseTool: () => {
      if (!heldName) return;
      heldName = null;
      setActive(previousName);
      previousName = null;
    },
    confirm: () => {
      const tool = tools[activeName];
      if (tool?.confirm) tool.confirm();
    },
    cancel: () => {
      const tool = tools[activeName];
      if (tool?.cancel) tool.cancel();
    },
  };
};
