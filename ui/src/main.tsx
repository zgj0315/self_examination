import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import Log from "./Log.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
    <Log />
  </StrictMode>
);
