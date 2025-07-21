import React, { useState, useEffect } from "react";
import { Line } from "@ant-design/plots";
import type { StatisticProps } from "antd";
import { Col, Row, Statistic, message } from "antd";
import CountUp from "react-countup";
import restful_api from "./RESTfulApi.tsx";

const Demo = () => {
  const data = [
    { year: "1991", value: 3 },
    { year: "1992", value: 4 },
    { year: "1993", value: 3.5 },
    { year: "1994", value: 5 },
    { year: "1995", value: 4.9 },
    { year: "1996", value: 6 },
    { year: "1997", value: 7 },
    { year: "1998", value: 9 },
    { year: "1999", value: 13 },
  ];

  const config = {
    data,
    height: 400,
    xField: "year",
    yField: "value",
    style: {
      lineWidth: 2,
    },
  };
  return <Line {...config} />;
};

const formatter: StatisticProps["formatter"] = (value) => (
  <CountUp end={value as number} separator="," />
);

const App: React.FC = () => {
  const [pdfArticleCountStat, setPdfArticleCountStat] = useState<number>(0);
  const [pdfArticleAccessLogCountStat, setPdfArticleAccessLogCountStat] =
    useState<number>(0);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await restful_api.get("/api/home/pdf_article_stat");
        setPdfArticleCountStat(response.data.pdf_article_count);
        setPdfArticleAccessLogCountStat(
          response.data.pdf_article_access_log_count
        );
        message.success("查询成功");
      } catch (e) {
        console.error("查询失败: ", e);
        message.error("查询失败");
      }
    };
    fetchData();
  }, []);
  return (
    <>
      <Row gutter={16}>
        <Col span={12}>
          <Statistic
            title="PDF文章"
            value={pdfArticleCountStat}
            formatter={formatter}
          />
        </Col>
        <Col span={12}>
          <Statistic
            title="浏览次数"
            value={pdfArticleAccessLogCountStat}
            precision={2}
            formatter={formatter}
          />
        </Col>
      </Row>
      <Demo />
    </>
  );
};

export default App;
