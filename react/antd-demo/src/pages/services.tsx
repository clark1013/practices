import {
  CreditCardOutlined,
  DownOutlined,
  AppstoreOutlined,
  CloudOutlined,
  ProfileOutlined,
  FolderOpenOutlined,
} from '@ant-design/icons';
import { PageContainer, ProCard } from '@ant-design/pro-components';
import { Tree, Tabs, Descriptions, Table } from 'antd';
import type { DataNode, DirectoryTreeProps } from 'antd/es/tree';
import { useEffect, useState } from 'react';
import { useModel } from 'umi';
import './services.less';

export default () => {
  const [treeData, setTreeData] = useState([]);
  const [tabItems, setTabItems] = useState([]);
  const [expandedKeys, setExpandedKeys] = useState([]);
  const [selectedKeys, setSelectedKeys] = useState([]);

  const renderDetails = (keys) => {
    fetch('http://localhost:8080/api/services/' + keys[0])
      .then((res) => res.json())
      .then(
        (result) => {
          let items = [
            {
              label: 'Meta',
              key: 'meta',
              children: (
                <Descriptions column={1} bordered>
                  <Descriptions.Item label="Service">
                    {result.service_name}
                  </Descriptions.Item>
                  <Descriptions.Item label="Business">
                    {result.business}
                  </Descriptions.Item>
                  <Descriptions.Item label="Application">
                    {result.application}
                  </Descriptions.Item>
                  <Descriptions.Item label="Github Repository">
                    {result.repo}
                  </Descriptions.Item>
                  <Descriptions.Item label="Pulumi">
                    {result.pulumi_path}
                  </Descriptions.Item>
                </Descriptions>
              ),
            },
          ];
          if (result.repo_rpc != null && result.repo_rpc.length > 0) {
            const columns = [
              {
                title: 'gRPC Method',
                dataIndex: 'method',
              },
              {
                title: 'gRPC Service',
                dataIndex: 'service',
              },
              {
                title: 'Request/day',
                dataIndex: 'rpd',
              },
            ];
            let dataSource = [];
            for (let repo of result.repo_rpc) {
              for (let rpc of repo.rpcs) {
                for (let method of rpc.methods) {
                  dataSource.push({
                    method: method,
                    service: rpc.service,
                    key: rpc.service + '.' + method,
                  });
                }
              }
            }
            items.push({
              label: 'gRPC',
              key: 'grpc',
              children: <Table columns={columns} dataSource={dataSource} />,
            });
          }
          setTabItems(items);
          setSelectedKeys(keys);
        },
        (error) => {
          console.log(error);
        },
      );
  };

  const onSelect: DirectoryTreeProps['onSelect'] = (keys, info) => {
    if (info.node.kind === 'service') {
      renderDetails(keys);
    }
  };

  const onExpand: DirectoryTreeProps['onExpand'] = (keys, info) => {
    setExpandedKeys(keys);
  };

  const traverse = (data, treeData, expandedKeys) => {
    treeData.title = data.name;
    treeData.key = data.name;
    treeData.kind = data.kind;
    if (treeData.kind === 'application') {
      treeData.icon = <AppstoreOutlined />;
      treeData.selectable = false;
    } else if (treeData.kind === 'service') {
      treeData.icon = <CloudOutlined />;
    } else if (treeData.kind === 'business') {
      treeData.icon = <ProfileOutlined />;
      treeData.selectable = false;
    } else {
      treeData.icon = <FolderOpenOutlined />;
      treeData.selectable = false;
    }

    treeData.children = [];
    if (data.children === null) {
      return treeData;
    }
    expandedKeys.push(treeData.key);
    data.children.sort((a, b) => a.name.localeCompare(b.name));
    for (let child of data.children) {
      let node = {};
      treeData.children.push(traverse(child, node, expandedKeys));
    }
    return treeData;
  };

  useEffect(() => {
    fetch('http://localhost:8080/api/services')
      .then((res) => res.json())
      .then(
        (result) => {
          let treeData = {};
          let expandedKeys = [];
          treeData = traverse(result, treeData, expandedKeys);
          setTreeData([treeData]);
          setExpandedKeys(expandedKeys);
          setSelectedKeys(['metrics-api']);
          renderDetails(['metrics-api']);
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
          style={{ minHeight: '90vh', height: '100px', overflowY: 'scroll' }}
          colSpan={8}
          bordered
        >
          <Tree
            showIcon
            onSelect={onSelect}
            onExpand={onExpand}
            treeData={treeData}
            expandedKeys={expandedKeys}
            selectedKeys={selectedKeys}
          />
        </ProCard>
        <ProCard
          style={{ minHeight: '90vh', height: '100px', overflowY: 'scroll' }}
          colSpan={16}
          bordered
        >
          <Tabs defaultActiveKey="meta" items={tabItems} />
        </ProCard>
      </ProCard>
    </PageContainer>
  );
};
