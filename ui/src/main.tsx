import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import Article from "./Article.tsx";
import Log from "./Log.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Article />
    <Log />
  </StrictMode>
);
