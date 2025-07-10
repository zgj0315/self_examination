import { createRoot } from "react-dom/client";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import Article from "./Article.tsx";
import Log from "./Log.tsx";
import AppLayout from "./AppLayout.tsx";

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <Routes>
      <Route path="/" element={<AppLayout />}>
        <Route index element={<Navigate to="/articles" replace />} />
        <Route path="articles" element={<Article />} />
        <Route path="logs" element={<Log />} />
      </Route>
    </Routes>
  </BrowserRouter>
);
