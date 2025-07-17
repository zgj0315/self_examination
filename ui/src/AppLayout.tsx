import React from "react";
import {
  // UploadOutlined,
  UserOutlined,
  // VideoCameraOutlined,
} from "@ant-design/icons";
import { Layout, Menu, theme } from "antd";
import { Outlet, useNavigate, useLocation } from "react-router-dom";

const { Header, Content, Footer, Sider } = Layout;

const App: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();

  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

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
          selectedKeys={[location.pathname]}
          onClick={({ key }) => navigate(key)}
          items={[
            // {
            //   key: "/articles",
            //   icon: <UserOutlined />,
            //   label: "文章管理",
            // },
            // {
            //   key: "/logs",
            //   icon: <VideoCameraOutlined />,
            //   label: "操作日志",
            // },
            // {
            //   key: "/files",
            //   icon: <UploadOutlined />,
            //   label: "文件管理",
            // },
            {
              key: "/pdf_articles",
              icon: <UserOutlined />,
              label: "文章列表",
            },
            {
              key: "/pdf_article_access_logs",
              icon: <UserOutlined />,
              label: "浏览记录",
            },
          ]}
        />
      </Sider>
      <Layout>
        <Header style={{ padding: 0, background: colorBgContainer }} />
        <Content style={{ margin: "24px 16px 0" }}>
          <div
            style={{
              padding: 24,
              minHeight: 360,
              background: colorBgContainer,
              borderRadius: borderRadiusLG,
            }}
          >
            <Outlet />
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
