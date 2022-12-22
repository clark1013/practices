import styles from './index.less';
import { Button } from 'antd';
import ProLayout, { PageContainer } from '@ant-design/pro-layout';

export default function IndexPage() {
  return (
    <ProLayout>
      <PageContainer
        extra={[
          <Button key="3">Operating</Button>,
          <Button key="2">Operating</Button>,
          <Button key="1" type="primary">
            Main Operating
          </Button>,
        ]}
        footer={[
          <Button>reset</Button>,
          <Button type="primary">submit</Button>,
        ]}
      >
        {/* {children} */}
      </PageContainer>
    </ProLayout>
  );
}
