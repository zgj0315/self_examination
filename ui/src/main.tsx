import { createRoot } from "react-dom/client";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import ArticlePage from "./ArticlePage.tsx";
import LogPage from "./LogPage.tsx";
import AppLayout from "./AppLayout.tsx";
import FilePage from "./FilePage.tsx";
import PdfPage from "./PdfPage.tsx";
import LoginPage from "./LoginPage.tsx";
import PdfArticleQueryPage from "./PdfArticleQueryPage.tsx";
import PdfArticleAccessLogQueryPage from "./PdfArticleAccessLogQueryPage.tsx";
import PdfArticleDetailPage from "./PdfArticleDetailPage.tsx";

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <Routes>
      <Route path="login" element={<LoginPage />} />
      <Route path="/" element={<AppLayout />}>
        <Route index element={<Navigate to="/pdf_articles" replace />} />
        <Route path="articles" element={<ArticlePage />} />
        <Route path="logs" element={<LogPage />} />
        <Route path="files" element={<FilePage />} />
        <Route path="pdfs/:id" element={<PdfPage />} />
        <Route path="pdf_articles" element={<PdfArticleQueryPage />} />
        <Route path="pdf_articles/:id" element={<PdfArticleDetailPage />} />
        <Route
          path="pdf_article_access_logs"
          element={<PdfArticleAccessLogQueryPage />}
        />
      </Route>
    </Routes>
  </BrowserRouter>
);
