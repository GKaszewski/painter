export const createKeyboard = () => {
  const actions = {};
  const handlers = {};
  const heldKeys = new Set();

  const dispatch = (actionName) => {
    (handlers[actionName] || []).forEach((fn) => fn());
  };

  window.addEventListener("keydown", (event) => {
    const action = actions[event.key];
    if (!action) return;
    event.preventDefault();
    if (action.hold) {
      if (!event.repeat && !heldKeys.has(event.key)) {
        heldKeys.add(event.key);
        dispatch(`${action.name}:down`);
      }
    } else if (!event.repeat) {
      dispatch(action.name);
    }
  });

  window.addEventListener("keyup", (event) => {
    const action = actions[event.key];
    if (!action) return;
    event.preventDefault();
    if (action.hold && heldKeys.has(event.key)) {
      heldKeys.delete(event.key);
      dispatch(`${action.name}:up`);
    }
  });

  window.addEventListener("blur", () => {
    for (const key of heldKeys) {
      const action = actions[key];
      if (action?.hold) dispatch(`${action.name}:up`);
    }
    heldKeys.clear();
  });

  return {
    register: (key, name, opts = {}) => {
      actions[key] = { name, ...opts };
    },
    onAction: (actionName, fn) => {
      if (!handlers[actionName]) handlers[actionName] = [];
      handlers[actionName].push(fn);
    },
  };
};
