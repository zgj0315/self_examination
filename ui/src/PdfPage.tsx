import { useState } from "react";
import { pdfjs, Document, Page } from "react-pdf";
import "react-pdf/dist/Page/AnnotationLayer.css";
import "react-pdf/dist/Page/TextLayer.css";
import type { PDFDocumentProxy } from "pdfjs-dist";

pdfjs.GlobalWorkerOptions.workerSrc = new URL(
  "pdfjs-dist/build/pdf.worker.min.mjs",
  import.meta.url
).toString();

const App: React.FC = () => {
  const [numPages, setNumPages] = useState<number>();
  const [pageNumber, setPageNumber] = useState<number>(1);

  const token = localStorage.getItem("token");

  function onDocumentLoadSuccess({
    numPages: nextNumPages,
  }: PDFDocumentProxy): void {
    setNumPages(nextNumPages);
  }

  const goToPrevPage = () => {
    setPageNumber((prev) => Math.max(prev - 1, 1));
  };

  const goToNextPage = () => {
    if (numPages) {
      setPageNumber((prev) => Math.min(prev + 1, numPages));
    }
  };
  return (
    <div>
      <Document
        file={{
          url: "/api/files/4",
          httpHeaders: {
            Authorization: `Bearer ${token}`,
          },
        }}
        onLoadSuccess={onDocumentLoadSuccess}
      >
        <Page pageNumber={pageNumber} />
      </Document>
      <p>
        <button onClick={goToPrevPage} disabled={pageNumber <= 1}>
          上一页
        </button>
        Page {pageNumber} of {numPages}
        <button
          onClick={goToNextPage}
          disabled={!numPages || pageNumber >= numPages}
        >
          下一页
        </button>
      </p>
    </div>
  );
};

export default App;
