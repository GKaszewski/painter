import io from "socket.io-client";

const isDebug = import.meta.env.VITE_IS_DEBUG === "true";
const wsProtocol = window.location.protocol === "https:" ? "wss:" : "ws:";
const wsHost = window.location.host;

let socket;

export const connectToWS = () => {
  if (isDebug) {
    socket = io("ws://localhost:3000");
  } else {
    socket = io(`${wsProtocol}//${wsHost}`, {
      transports: ["websocket"],
    });
  }

  return socket;
};

export default socket;
