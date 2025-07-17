# 吾日三省吾身
## 我是什么
我是一个单体Web站点，用于记录每天对自己反省的内容  
没有Nginx，没有Redis，没有数据库
## 架构设计
采用前后端分离的架构设计  
前端：Vite, React, TypeScript, Ant-Design, Axios, react-router-dom  
后端：Axum, Sea-Orm

## FAQ
### 工程如何开发联调
```
# 启动后台工程
cd server/server
cargo watch -x run

# 启动前台工程
cd ui
npm run dev

```