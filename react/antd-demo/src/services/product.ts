export function queryProductList() {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve({
        data: [
          { id: 1, name: 'dva' },
          { id: 2, name: 'antd' },
        ],
      });
    }, 2000);
  });
}
