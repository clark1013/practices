import ProductList from '@/components/ProductList';
import React from 'react';
import { useModel } from 'umi';
import styles from './products.css';

export default function Page() {
  const { dataSource, reload, deleteProducts } = useModel('useProductList');
  return (
    <div>
      <a onClick={() => reload()}>reload</a>
      <ProductList onDelete={deleteProducts} products={dataSource} />
    </div>
  );
}
