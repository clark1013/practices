import { PageContainer, ProCard } from "@ant-design/pro-components";

export default () => {
    return <PageContainer>
      <ProCard ghost gutter={8}>
      <ProCard style={{height:"88vh"}} colSpan={12} bordered>New page1</ProCard>
      <ProCard style={{height:"88vh"}} colSpan={12} bordered>New page2</ProCard>
      </ProCard>
      </PageContainer>
  };