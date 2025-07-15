import axios from "axios";

const restful_api = axios.create({
  baseURL: "/",
});

restful_api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem("token");
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

export default restful_api;
