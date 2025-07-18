import React from "react";
import {
  // UploadOutlined,
  UserOutlined,
  VideoCameraOutlined,
} from "@ant-design/icons";
import { Layout, Menu, theme, Button } from "antd";
import { Outlet, useNavigate, useLocation } from "react-router-dom";
import restful_api from "./RESTfulApi.tsx";

const { Header, Content, Footer, Sider } = Layout;

const App: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();

  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  const menuItems = [
    {
      key: "/pdf_articles",
      icon: <UserOutlined />,
      label: "文章列表",
    },
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
  ];
  const isLoggedIn = !!localStorage.getItem("token");
  if (isLoggedIn) {
    menuItems.push({
      key: "/pdf_article_access_logs",
      icon: <VideoCameraOutlined />,
      label: "浏览记录",
    });
  }
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
          items={menuItems}
        />
      </Sider>
      <Layout>
        <Header style={{ padding: 0, background: colorBgContainer }}>
          {localStorage.getItem("token") && (
            <Button
              type="link"
              onClick={async () => {
                const token = localStorage.getItem("token");
                if (token) {
                  try {
                    await restful_api.post(`/api/logout/${token}`, null, {
                      headers: {
                        Authorization: `Bearer ${token}`,
                      },
                    });
                  } catch (error) {
                    console.error("Logout failed", error);
                  }
                  localStorage.removeItem("token");
                  window.location.reload();
                }
              }}
            >
              Logout
            </Button>
          )}
        </Header>
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
