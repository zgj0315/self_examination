import React from "react";
import type { FormProps } from "antd";
import { Button, Form, Input, message } from "antd";
import axios from "axios";

type FieldType = {
  title?: string;
  content?: string;
};

const onFinish: FormProps<FieldType>["onFinish"] = (values) => {
  console.log("Success:", values);
  try {
    const response = axios.post("/api/article/create", values);
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

const App: React.FC = () => (
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
    </Form.Item>
  </Form>
);

export default App;
