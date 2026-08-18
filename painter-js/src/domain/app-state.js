export const createAppState = (initial) => {
  const state = { ...initial };
  const listeners = {};

  return {
    get: (key) => state[key],
    set: (key, value) => {
      const old = state[key];
      state[key] = value;
      (listeners[key] || []).forEach((fn) => fn(value, old));
    },
    subscribe: (key, fn) => {
      if (!listeners[key]) listeners[key] = [];
      listeners[key].push(fn);
      return () => {
        listeners[key] = listeners[key].filter((f) => f !== fn);
      };
    },
  };
};
