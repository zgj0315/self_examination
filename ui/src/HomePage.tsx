import React, { useState, useEffect } from "react";
import { Line } from "@ant-design/plots";
import type { StatisticProps } from "antd";
import { Col, Row, Statistic, message } from "antd";
import CountUp from "react-countup";
import restful_api from "./RESTfulApi.tsx";

const DailyAccessStat = ({
  data,
}: {
  data: { day: string; count: number }[];
}) => {
  const config = {
    data,
    height: 400,
    xField: "day",
    yField: "count",
    style: {
      lineWidth: 2,
    },
    point: {
      size: 5,
      shape: "circle",
    },
    tooltip: {
      showMarkers: true,
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
  const [dailyAccessStats, setDailyAccessStats] = useState<
    { day: string; count: number }[]
  >([]);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await restful_api.get("/api/home/pdf_article_stat");
        setPdfArticleCountStat(response.data.pdf_article_count);
        setPdfArticleAccessLogCountStat(
          response.data.pdf_article_access_log_count
        );
        setDailyAccessStats(response.data.daily_access_stats || []);
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
      <DailyAccessStat data={dailyAccessStats} />
    </>
  );
};

export default App;
