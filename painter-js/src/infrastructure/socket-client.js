import io from "socket.io-client";

const isDebug = import.meta.env.VITE_IS_DEBUG === "true";
const transport = import.meta.env.VITE_TRANSPORT || "socketio";

const createSocketIoTransport = () => {
  const url = isDebug ? "ws://localhost:3000" : undefined;
  const socket = url ? io(url) : io({ transports: ["websocket"] });

  return {
    on: (event, handler) => socket.on(event, handler),
    emit: (event, data) => socket.emit(event, data),
  };
};

const decompressGzip = async (buffer) => {
  const stream = new Blob([buffer])
    .stream()
    .pipeThrough(new DecompressionStream("gzip"));
  return new Response(stream).arrayBuffer();
};

const createWebSocketTransport = () => {
  const handlers = {};
  const wsUrl = isDebug
    ? "ws://localhost:3000/ws"
    : `${window.location.protocol === "https:" ? "wss:" : "ws:"}//${window.location.host}/ws`;

  const ws = new WebSocket(wsUrl);
  ws.binaryType = "arraybuffer";

  const on = (event, handler) => {
    if (!handlers[event]) handlers[event] = [];
    handlers[event].push(handler);
  };

  const emit = (event, data) => {
    if (ws.readyState !== WebSocket.OPEN) return;
    ws.send(JSON.stringify({ type: event, ...data }));
  };

  const dispatch = (event, ...args) => {
    (handlers[event] || []).forEach((handler) => handler(...args));
  };

  ws.addEventListener("open", () => dispatch("connect"));

  ws.addEventListener("message", async (event) => {
    if (event.data instanceof ArrayBuffer) {
      const decompressed = await decompressGzip(event.data);
      const pixels = new Uint32Array(decompressed);
      dispatch("canvas_state", Array.from(pixels));
      return;
    }

    const message = JSON.parse(event.data);
    switch (message.type) {
      case "pixel-updated":
        dispatch("pixel-updated", message);
        break;
      case "current_soldiers":
        dispatch("current_soldiers", message.count);
        break;
      case "error":
        dispatch("error", message.message);
        break;
    }
  });

  ws.addEventListener("close", () => dispatch("disconnect"));

  return { on, emit };
};

export const createSocketConnection = () => {
  if (transport === "websocket") {
    return createWebSocketTransport();
  }
  return createSocketIoTransport();
};
