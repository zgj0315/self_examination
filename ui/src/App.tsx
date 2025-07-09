import React, { useState } from "react";
import {
  Button,
  Form,
  Input,
  message,
  Table,
  Popconfirm,
  Layout,
  Menu,
  theme,
  Modal,
} from "antd";
import axios from "axios";
import {
  UploadOutlined,
  UserOutlined,
  VideoCameraOutlined,
} from "@ant-design/icons";
import dayjs from "dayjs";

const { Header, Content, Footer, Sider } = Layout;

const items = [UserOutlined, VideoCameraOutlined, UploadOutlined].map(
  (icon, index) => ({
    key: String(index + 1),
    icon: React.createElement(icon),
    label: `Nav ${index + 1}`,
  })
);

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

  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  const [data, setData] = useState<Article[]>([]);
  const [loading, setLoading] = useState(false);

  const handleQuery = async () => {
    setLoading(true);
    try {
      const response = await axios.get("/api/articles?size=20&page=0");
      setData(response.data._embedded?.article);
      message.success("查询成功");
    } catch (e) {
      console.error("查询失败: ", e);
      message.error("查询失败");
    } finally {
      setLoading(false);
    }
  };
  const handleUpdate = (record: Article) => {
    // setEditingArticle(record);
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
  return (
    <Layout>
      <Sider
        breakpoint="lg"
        collapsedWidth="0"
        onBreakpoint={(broken) => {
          console.log(broken);
        }}
        onCollapse={(collapsed, type) => {
          console.log(collapsed, type);
        }}
      >
        <div className="demo-logo-vertical" />
        <Menu
          theme="dark"
          mode="inline"
          defaultSelectedKeys={["4"]}
          items={items}
        />
      </Sider>
      <Layout>
        <Header style={{ padding: 0, background: colorBgContainer }} />
        <Content>
          <div
            style={{
              padding: 24,
              minHeight: 360,
              minWidth: 600,
              background: colorBgContainer,
              borderRadius: borderRadiusLG,
            }}
          >
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
            <Button style={{ marginLeft: 8 }} onClick={handleQuery}>
              查询
            </Button>
            <Table
              dataSource={data}
              columns={columns}
              rowKey="id"
              loading={loading}
              pagination={{ pageSize: 5 }}
              style={{ marginTop: 24 }}
            />
          </div>
        </Content>
        <Footer style={{ textAlign: "center" }}>
          Ant Design ©{new Date().getFullYear()} Created by Ant UED
        </Footer>
      </Layout>
    </Layout>
  );
};

export default App;
