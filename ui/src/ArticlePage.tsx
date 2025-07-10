import React, { useEffect, useState } from "react";
import { Button, Form, Input, message, Table, Popconfirm, Modal } from "antd";
import axios from "axios";

import dayjs from "dayjs";

type FieldType = {
  title?: string;
  content?: string;
};

type UpdateField = {
  id?: number;
  title?: string;
  content?: string;
};

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
  const [form] = Form.useForm();
  const [create_open, setCreateOpen] = useState(false);
  const [update_open, setUpdateOpen] = useState(false);

  const onCreate = async (values: FieldType) => {
    console.log("Received values of form: ", values);
    try {
      const response = await axios.post("/api/articles", values);
      console.log("create success, response: ", response);
      message.success("create success");
    } catch (e) {
      console.error("create error: ", e);
      message.error("create error");
    }
    setCreateOpen(false);
    handleQuery();
  };

  const onUpdate = async (values: UpdateField) => {
    console.log("Received values of form: ", values);
    try {
      const response = await axios.patch(`/api/articles/${values.id}`, values);
      console.log("update success, response: ", response);
      message.success("update success");
    } catch (e) {
      console.error("update error: ", e);
      message.error("update error");
    }
    setUpdateOpen(false);
    handleQuery();
  };

  const [articles, setArticles] = useState<Article[]>([]);
  const [current, setCurrent] = useState(1);
  const [page_size, setPageSize] = useState(5);
  const [page, setPage] = useState<Page>();
  const [loading, setLoading] = useState(false);

  const handleQuery = async (
    page = current,
    size = page_size,
    filters?: { title?: string; content?: string }
  ) => {
    console.log("handleQuery page: ", page);
    console.log("handleQuery size: ", size);
    const params = new URLSearchParams();
    params.append("size", size.toString());
    params.append("page", (page - 1).toString());
    if (filters?.title) params.append("title", filters.title);
    if (filters?.content) params.append("content", filters.content);
    setLoading(true);
    try {
      const response = await axios.get(`/api/articles?${params.toString()}`);
      setArticles(response.data._embedded?.article);
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
  const handleUpdate = (record: Article) => {
    form.setFieldsValue(record);
    setUpdateOpen(true);
  };
  const handleDelete = async (id: number) => {
    try {
      await axios.delete(`/api/articles/${id}`);
      message.success("删除成功");
      handleQuery();
    } catch (error) {
      console.error("删除失败:", error);
      message.error("删除失败");
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
      dataIndex: "title",
      key: "title",
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
    {
      title: "修改时间",
      dataIndex: "updated_at",
      key: "updated_at",
      render: (timestamp: number) =>
        timestamp ? dayjs(timestamp).format("YYYY-MM-DD HH:mm:ss") : "--",
    },
    {
      title: "操作",
      key: "action",
      render: (_: unknown, record: Article) => (
        <>
          <Button type="link" onClick={() => handleUpdate(record)}>
            编辑
          </Button>
          <Popconfirm
            title="确定要删除这条记录吗？"
            onConfirm={() => handleDelete(record.id)}
            okText="确定"
            cancelText="取消"
          >
            <Button danger type="link">
              删除
            </Button>
          </Popconfirm>
        </>
      ),
    },
  ];

  useEffect(() => {
    handleQuery();
  }, []);

  return (
    <>
      <Button type="primary" onClick={() => setCreateOpen(true)}>
        New Article
      </Button>
      <Modal
        open={create_open}
        title="Create a new article"
        okText="Create"
        cancelText="Cancel"
        okButtonProps={{ autoFocus: true, htmlType: "submit" }}
        onCancel={() => setCreateOpen(false)}
        destroyOnHidden
        modalRender={(dom) => (
          <Form
            layout="vertical"
            form={form}
            name="form_in_modal"
            clearOnDestroy
            onFinish={(values) => onCreate(values)}
          >
            {dom}
          </Form>
        )}
      >
        <Form.Item
          name="title"
          label="Title"
          rules={[
            {
              required: true,
              message: "Please input the title of article!",
            },
          ]}
        >
          <Input />
        </Form.Item>
        <Form.Item
          name="content"
          label="Content"
          rules={[
            {
              required: true,
              message: "Please input the Content of article!",
            },
          ]}
        >
          <Input.TextArea rows={4} />
        </Form.Item>
      </Modal>
      <Modal
        open={update_open}
        title="Update a new article"
        okText="Update"
        cancelText="Cancel"
        okButtonProps={{ autoFocus: true, htmlType: "submit" }}
        onCancel={() => setUpdateOpen(false)}
        destroyOnHidden
        modalRender={(dom) => (
          <Form
            layout="vertical"
            form={form}
            name="form_in_modal"
            clearOnDestroy
            onFinish={(values) => onUpdate(values)}
          >
            {dom}
          </Form>
        )}
      >
        <Form.Item name="id" label="ID" hidden>
          <Input />
        </Form.Item>
        <Form.Item
          name="title"
          label="Title"
          rules={[
            {
              required: true,
              message: "Please input the title of article!",
            },
          ]}
        >
          <Input />
        </Form.Item>
        <Form.Item
          name="content"
          label="Content"
          rules={[
            {
              required: true,
              message: "Please input the Content of article!",
            },
          ]}
        >
          <Input.TextArea rows={4} />
        </Form.Item>
      </Modal>
      <Form
        layout="inline"
        onFinish={(values) => handleQuery(1, page_size, values)}
        style={{ marginTop: 16 }}
      >
        <Form.Item name="title" label="标题">
          <Input placeholder="请输入标题关键字" />
        </Form.Item>
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
