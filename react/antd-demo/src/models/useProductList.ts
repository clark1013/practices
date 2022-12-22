import { queryProductList } from '@/services/product';
import { useRequest } from 'umi';

export default function useProductList() {
  const msg = useRequest(() => queryProductList());

  const deleteProducts = async (id: string) => {
    try {
      msg.run();
    } catch (error) {
      console.log('error');
    }
  };

  return {
    dataSource: msg.data,
    reload: msg.run,
    loading: msg.loading,
    deleteProducts,
  };
}
