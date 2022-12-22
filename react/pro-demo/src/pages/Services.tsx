import { CreditCardOutlined, DownOutlined } from "@ant-design/icons";
import { PageContainer, ProCard } from "@ant-design/pro-components";
import {Tree} from "antd";
import type { DataNode, DirectoryTreeProps } from 'antd/es/tree';

const treeData: DataNode[] = [
  {
    title: 'parent 0',
    key: '0-0',
    icon: CreditCardOutlined,
    children: [
      { title: 'leaf 0-0', key: '0-0-0', isLeaf: true, icon: CreditCardOutlined },
      { title: 'leaf 0-1', key: '0-0-1', isLeaf: true, icon: CreditCardOutlined},
    ],
  },
  {
    title: 'parent 1',
    key: '0-1',
    icon: CreditCardOutlined,
    children: [
      { title: 'leaf 1-0', key: '0-1-0', isLeaf: true, icon: CreditCardOutlined },
      { title: 'leaf 1-1', key: '0-1-1', isLeaf: true, icon: CreditCardOutlined },
    ],
  },
];

export default () => {
  const onSelect: DirectoryTreeProps['onSelect'] = (keys, info) => {
    console.log('Trigger Select', keys, info);
  };

  const onExpand: DirectoryTreeProps['onExpand'] = (keys, info) => {
    console.log('Trigger Expand', keys, info);
  };
    return <PageContainer>
      <ProCard ghost gutter={8}>
      <ProCard style={{height:"88vh"}} colSpan={12} bordered>
        <Tree
          multiple
          // showIcon
          defaultExpandAll
          switcherIcon={<DownOutlined />}
          onSelect={onSelect}
          onExpand={onExpand}
          treeData={treeData}
        />
      </ProCard>
      <ProCard style={{height:"88vh"}} colSpan={12} bordered>New page2</ProCard>
      </ProCard>
      </PageContainer>
  };