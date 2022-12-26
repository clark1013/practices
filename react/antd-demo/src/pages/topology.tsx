import { RadialGraph } from '@ant-design/graphs';

import { PageContainer, ProCard } from '@ant-design/pro-components';
import { useEffect, useState, useRef } from 'react';
export default () => {
  const [data, setData] = useState({nodes: [{"id": "1", "label": ""}], edges: []})
  const chartRef = useRef();
  const asyncData = () => {
    console.log('test')
  }
  useEffect(() => {
    fetch('http://localhost:8080/api/topology')
    .then((res) => res.json())
    .then(
      (result) => {
        setData({nodes: result.nodes, edges: result.edges})
      },
      (error) => {
        console.log(error);
      },
    );
  }, []);

  return (
    <PageContainer>
      <ProCard ghost gutter={8}>
        <ProCard
          style={{ minHeight: '90vh', height: '100%' }}
          colSpan={24}
          bordered
        >
          <RadialGraph
            data={data}
            autoFit={false}
            behaviors={['drag-node', 'drag-canvas', 'scroll-canvas', 'zoom-canvas']}
            layout={{
              unitRadius: 160,
              nodeSize: 36,
              linkDistance: 200,
              nodeSpacing: 40,
              center: [0, 0]
            }}
            nodeCfg={{
              asyncData,
              size: 40,
              style: {
                fill: '#1890ff',
                stroke: '#1890ff',
              },
              labelCfg: {
                style: {
                  fontSize: 10,
                //   fill: '#ffffff',
                },
              },
            }}
            edgeCfg={{
              style: {
                lineWidth: 2,
              },
              endArrow: {
                d: 15,
                size: 8,
              },
            }}
            onReady= {(graph) => {
                chartRef.current = graph;
              }}
          />
        </ProCard>
      </ProCard>
    </PageContainer>
  );
};
