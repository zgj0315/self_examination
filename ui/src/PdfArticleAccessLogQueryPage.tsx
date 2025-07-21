import React, { useEffect, useState } from "react";
import { Button, Form, Input, message, Table } from "antd";
import restful_api from "./RESTfulApi.tsx";
import dayjs from "dayjs";

type Article = {
  id: number;
  title: string;
  content: string;
};

type Page = {
  size: number;
  total_elements: number;
  total_pages: number;
};

const App: React.FC = () => {
  const [articles, setArticles] = useState<Article[]>([]);
  const [current, setCurrent] = useState(1);
  const [page_size, setPageSize] = useState(5);
  const [page, setPage] = useState<Page>();
  const [loading, setLoading] = useState(false);

  const handleQuery = async (
    page = current,
    size = page_size,
    filters?: { src_ip?: string; user_agent?: string }
  ) => {
    console.log("handleQuery page: ", page);
    console.log("handleQuery size: ", size);
    const params = new URLSearchParams();
    params.append("size", size.toString());
    params.append("page", (page - 1).toString());
    if (filters?.src_ip) params.append("src_ip", filters.src_ip);
    if (filters?.user_agent) params.append("user_agent", filters.user_agent);
    setLoading(true);
    try {
      const response = await restful_api.get(
        `/api/pdf_article_access_logs?${params.toString()}`
      );
      setArticles(response.data._embedded?.pdf_article_access_log);
      setPage(response.data.page);
      setCurrent(page);
      setPageSize(size);
      message.success("查询成功");
    } catch (e) {
      console.error("查询失败: ", e);
      message.error("查询失败");
      window.location.href = "/login";
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
      title: "标题",
      dataIndex: "article_title",
      key: "article_title",
    },
    {
      title: "源IP",
      dataIndex: "src_ip",
      key: "src_ip",
    },
    {
      title: "User-Agent",
      dataIndex: "user_agent",
      key: "user_agent",
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
        <Form.Item name="src_ip" label="源IP">
          <Input placeholder="请输入源IP关键字" />
        </Form.Item>
        <Form.Item name="user_agent" label="User-Agent">
          <Input placeholder="请输入User-Agent关键字" />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            查询
          </Button>
        </Form.Item>
      </Form>

      <Table
        dataSource={articles}
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
