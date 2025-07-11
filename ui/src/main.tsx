import { createRoot } from "react-dom/client";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import ArticlePage from "./ArticlePage.tsx";
import LogPage from "./LogPage.tsx";
import AppLayout from "./AppLayout.tsx";
import FilePage from "./FilePage.tsx";

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <Routes>
      <Route path="/" element={<AppLayout />}>
        <Route index element={<Navigate to="/articles" replace />} />
        <Route path="articles" element={<ArticlePage />} />
        <Route path="logs" element={<LogPage />} />
        <Route path="files" element={<FilePage />} />
      </Route>
    </Routes>
  </BrowserRouter>
);
