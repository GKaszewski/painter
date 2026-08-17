const isDebug = import.meta.env.VITE_IS_DEBUG === "true";

const CHECK_ENDPOINT = isDebug ? "http://localhost:3000/check/" : "/check/";

export const checkServer = () => fetch(CHECK_ENDPOINT);
