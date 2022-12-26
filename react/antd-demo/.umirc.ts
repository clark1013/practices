import { defineConfig } from 'umi';

export default defineConfig({
  nodeModulesTransform: {
    type: 'none',
  },
  routes: [
    { path: '/', redirect: '/services' },
    {
      path: '/services',
      name: 'Services',
      component: '@/pages/services',
      icon: 'apartment',
    },
    {
      path: '/topology',
      name: 'Topology',
      component: '@/pages/topology',
      icon: 'branches',
    },
  ],
  fastRefresh: {},
  layout: {
    title: 'Service Catalog',
    logo: 'https://static.pingcap.com/files/2021/10/TiDB-Cloud-logo-black.png',
    locale: false,
    siderWidth: 230,
    navTheme: 'light',
    primaryColor: '#1890ff',
  },
});
