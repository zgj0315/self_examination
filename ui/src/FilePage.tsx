import React, { useEffect, useState } from "react";
import { UploadOutlined } from "@ant-design/icons";
import type { UploadProps } from "antd";
import { Button, message, Upload, Form, Input, Table } from "antd";
import axios from "axios";
import dayjs from "dayjs";

type Page = {
  size: number;
  total_elements: number;
  total_pages: number;
};

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

const App: React.FC = () => {
  const [files, setFiles] = useState<File[]>([]);
  const [current, setCurrent] = useState(1);
  const [page_size, setPageSize] = useState(5);
  const [page, setPage] = useState<Page>();
  const [loading, setLoading] = useState(false);

  const handleQuery = async (
    page = current,
    size = page_size,
    filters?: { content?: string }
  ) => {
    console.log("handleQuery page: ", page);
    console.log("handleQuery size: ", size);
    const params = new URLSearchParams();
    params.append("size", size.toString());
    params.append("page", (page - 1).toString());
    if (filters?.content) params.append("content", filters.content);
    setLoading(true);
    try {
      const response = await axios.get(`/api/files?${params.toString()}`);
      setFiles(response.data._embedded?.file);
      setPage(response.data.page);
      setCurrent(page);
      setPageSize(size);
      message.success("查询成功");
    } catch (e) {
      console.error("查询失败: ", e);
      message.error("查询失败");
    } finally {
      setLoading(false);
    }
  };

  const columns = [
    {
      title: "ID",
      dataIndex: "id",
      key: "id",
    },
    {
      title: "文件名",
      dataIndex: "name",
      key: "name",
    },
    {
      title: "上传时间",
      dataIndex: "created_at",
      key: "created_at",
      render: (timestamp: number) =>
        timestamp ? dayjs(timestamp).format("YYYY-MM-DD HH:mm:ss") : "--",
    },
  ];

  useEffect(() => {
    handleQuery();
  }, []);

  return (
    <>
      <Upload {...props}>
        <Button icon={<UploadOutlined />}>Click to Upload</Button>
      </Upload>
      <Form
        layout="inline"
        onFinish={(values) => handleQuery(1, page_size, values)}
        style={{ marginTop: 16 }}
      >
        <Form.Item name="name" label="文件名">
          <Input placeholder="请输入文件名关键字" />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            查询
          </Button>
        </Form.Item>
      </Form>
      <Table
        dataSource={files}
        columns={columns}
        rowKey="id"
        loading={loading}
        pagination={{
          current: current,
          pageSize: page_size,
          total: page?.total_elements,
          onChange: (page, size) => handleQuery(page, size),
        }}
        style={{ marginTop: 24 }}
      />
    </>
  );
};

export default App;
