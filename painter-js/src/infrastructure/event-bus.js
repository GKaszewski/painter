export const createEventBus = () => {
  const handlers = {};

  return {
    on: (event, fn) => {
      if (!handlers[event]) handlers[event] = [];
      handlers[event].push(fn);
    },
    off: (event, fn) => {
      if (!handlers[event]) return;
      handlers[event] = handlers[event].filter((f) => f !== fn);
    },
    emit: (event, ...args) => {
      (handlers[event] || []).forEach((fn) => fn(...args));
    },
  };
};
