import React from "react";
import { UploadOutlined } from "@ant-design/icons";
import type { UploadProps } from "antd";
import { Button, message, Upload } from "antd";
import axios from "axios";

const props: UploadProps = {
  name: "file",
  customRequest: async (options) => {
    const { file, onSuccess, onProgress } = options;

    const formData = new FormData();
    formData.append("file", file as Blob);

    try {
      const response = await axios.post("/api/files", formData, {
        onUploadProgress: (event) => {
          if (event.total) {
            const percent = Math.round((event.loaded * 100) / event.total);
            onProgress?.({ percent });
          }
        },
      });
      onSuccess?.(response.data);
      message.success(`${(file as File).name} uploaded successfully`);
    } catch (error) {
      console.error("Upload error:", error);
      //   onError?.(error);
      message.error(`${(file as File).name} upload failed.`);
    }
  },
};

const App: React.FC = () => (
  <Upload {...props}>
    <Button icon={<UploadOutlined />}>Click to Upload</Button>
  </Upload>
);

export default App;
