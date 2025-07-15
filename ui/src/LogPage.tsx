import React, { useEffect, useState } from "react";
import { Button, Form, Input, message, Table } from "antd";
import restful_api from "./RESTfulApi.tsx";
import dayjs from "dayjs";

type Log = {
  id: number;
  content: string;
  created_at: number;
};

type Page = {
  size: number;
  total_elements: number;
  total_pages: number;
};

const App: React.FC = () => {
  const [logs, setLogs] = useState<Log[]>([]);
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
      const response = await restful_api.get(`/api/logs?${params.toString()}`);
      setLogs(response.data._embedded?.log);
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
      title: "内容",
      dataIndex: "content",
      key: "content",
    },
    {
      title: "创建时间",
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
      <Form
        layout="inline"
        onFinish={(values) => handleQuery(1, page_size, values)}
        style={{ marginTop: 16 }}
      >
        <Form.Item name="content" label="内容">
          <Input placeholder="请输入内容关键字" />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            查询
          </Button>
        </Form.Item>
      </Form>
      <Table
        dataSource={logs}
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
