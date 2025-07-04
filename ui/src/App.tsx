import React, { useState } from "react";
import type { FormProps } from "antd";
import { Button, Form, Input, message, Table } from "antd";
import axios from "axios";

type FieldType = {
  title?: string;
  content?: string;
};

type Article = {
  id: number;
  title: string;
  content: string;
};

const onFinish: FormProps<FieldType>["onFinish"] = (values) => {
  console.log("Success:", values);
  try {
    const response = axios.post("/api/articles", values);
    console.log("create success, response: ", response);
    message.success("create success");
  } catch (e) {
    console.error("create error: ", e);
    message.error("create error");
  }
};

const onFinishFailed: FormProps<FieldType>["onFinishFailed"] = (errorInfo) => {
  console.log("Failed:", errorInfo);
};

const App: React.FC = () => {
  const [data, setData] = useState<Article[]>([]);
  const [loading, setLoading] = useState(false);

  const handleQuery = async () => {
    setLoading(true);
    try {
      const response = await axios.get("/api/articles?size=20&page=0");
      setData(response.data._embedded?.article); // 假设返回数组，如 [{ id: 1, title: 't', content: 'c' }]
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
      title: "标题",
      dataIndex: "title",
      key: "title",
    },
    {
      title: "内容",
      dataIndex: "content",
      key: "content",
    },
  ];
  return (
    <>
      <Form
        name="basic"
        labelCol={{ span: 8 }}
        wrapperCol={{ span: 16 }}
        style={{ maxWidth: 600 }}
        initialValues={{ remember: true }}
        onFinish={onFinish}
        onFinishFailed={onFinishFailed}
        autoComplete="off"
      >
        <Form.Item<FieldType>
          label="Title"
          name="title"
          rules={[{ required: true, message: "Please input title!" }]}
        >
          <Input />
        </Form.Item>

        <Form.Item<FieldType>
          label="Content"
          name="content"
          rules={[{ required: true, message: "Please input content!" }]}
        >
          <Input.TextArea />
        </Form.Item>

        <Form.Item label={null}>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
          <Button style={{ marginLeft: 8 }} onClick={handleQuery}>
            查询
          </Button>
        </Form.Item>
      </Form>
      <Table
        dataSource={data}
        columns={columns}
        rowKey="id"
        loading={loading}
        pagination={{ pageSize: 5 }}
        style={{ marginTop: 24 }}
      />
    </>
  );
};

export default App;
