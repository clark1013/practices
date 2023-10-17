#ifndef INCLUDED_GAMELIB_HASTSETIMPL_H
#define INCLUDED_GAMELIB_HASHSETIMPL_H

#include "GameLib/GameLib.h"
#include "GameLib/Base/HashSet.h"
#include "GameLib/Base/PrimeNumber.h"


namespace GameLib{

//要做一些工作，以使开头和结尾不需要特别处理。
template< class T, class H > inline HashSet< T, H >::HashSet() : 
mNexts( 0 ),
mValues( 0 ),
mCapacity( 0 ),
mSize( 0 ),
mTableSize( 0 ){
	ASSERT( sizeof( T ) != 0 ); //有可能吗
}

//要做一些工作，以使开头和结尾不需要特别处理。
template< class T, class H > inline HashSet< T ,H >::HashSet( int capacity, int tableSize ) : 
mNexts( 0 ),
mValues( 0 ),
mCapacity( 0 ),
mSize( 0 ),
mTableSize( 0 ){
	ASSERT( sizeof( T ) != 0 ); //有可能吗
	setCapacity( capacity, tableSize );
}

template< class T, class H > inline HashSet< T, H >::~HashSet(){
	if ( mNexts ){
		int pos = mNexts[ mCapacity ];
		while ( pos < mCapacity + mTableSize ){ //如果有节点
			if ( pos < mCapacity ){
				mValues[ pos ].~T();
			}
			pos = mNexts[ pos ];
		}
		OPERATOR_DELETE( mValues ); //直接delete
		SAFE_DELETE_ARRAY( mNexts );
	}
}

template< class T, class H > inline void HashSet< T, H >::setCapacity( int capacity, int tableSize ){
	ASSERT( mSize == 0 && "NOT EMPTY! call clear()." );
	//先清除
	if ( mCapacity > 0 ){
		mEmptyStack.clear();
		clear();
		OPERATOR_DELETE( mValues ); //直接delete
		SAFE_DELETE_ARRAY( mNexts );
	}
	//重新分配
	mCapacity = capacity;
	if ( capacity <= 0 ){ //忽略0
		return;
	}	
	if ( tableSize == 0 ){ //自动设置大于capacity的质数
		tableSize = capacity;
	}
	tableSize = PrimeNumber::next( tableSize );
	if ( tableSize < 3 ){
		tableSize = 3; //至少3
	}else if ( tableSize > 0x10000 ){ //65536既然这样就不允许。
		tableSize = 65521; //65536最大素数
	}
	mTableSize = tableSize;
	//
	mNexts = NEW int[ mCapacity + mTableSize ];
	//分配值数组。
	mValues = static_cast< T* >( OPERATOR_NEW( sizeof( T ) * mCapacity ) );
	//存储空的编号
	mEmptyStack.setCapacity( mCapacity );
	for ( int i = 0; i < mCapacity; ++i ){
		mEmptyStack.push( i ); //放入空编号列表
	}
	//从mCapacity + 0到mCapacity + mTab​​leSize-1的数字是伪元素。位于每列开头之前。
	for ( int i = 0; i < mTableSize; ++i ){
		mNexts[ mCapacity + i ] = mCapacity + i + 1;
	}
}

template< class T, class H > inline int HashSet< T, H >::capacity() const {
	return mCapacity;
}

template< class T, class H > inline int HashSet< T, H >::add( const T& v ){
	ASSERT( mSize < mCapacity );
	//哈希函数计算
	int h = H().value( v, mTableSize );
	//查找
	int p = mNexts[ mCapacity + h ];
	while ( p < mCapacity ){
		if ( H().isEqual( mValues[ p ], v ) ){
			break;
		}
		p = mNexts[ p ];
	}
	if ( p >= mCapacity ){ //因为没有所以加上
		//获得可用空间
		int newPos;
		mEmptyStack.pop( &newPos );
		//插入表格顶部
		int head = mCapacity + h;
		mNexts[ newPos ] = mNexts[ head ]; //
		new ( &mValues[ newPos ] ) T( v ); //调用复制构造函数
		mNexts[ head ] = newPos; //将先前的链接连接到新节点
		++mSize;
		return newPos;
	}else{
		return mCapacity + mTableSize;
	}
}

template< class T, class H > inline int HashSet< T, H >::find( const T& v ){
	int h = H().value( v, mTableSize );

	int p = mNexts[ mCapacity + h ];
	while ( p < mCapacity ){
		if ( H().isEqual( mValues[ p ], v ) ){
			return p;
		}
		p = mNexts[ p ];
	}
	return mCapacity + mTableSize;
}

template< class T, class H > inline bool HashSet< T, H >::isIn( const T& v ) const {
	int h = H().value( v, mTableSize );

	int p = mNexts[ mCapacity + h ];
	while ( p < mCapacity ){
		if ( H().isEqual( mValues[ p ], v ) ){
			return true;
		}
		p = mNexts[ p ];
	}
	return false;
}

template< class T, class H > inline bool HashSet< T, H >::remove( int position ){
	//
	//因此，步进迭代器，直到它出现在表中。
	int h = position;
	while ( h < mCapacity ){
		h = mNexts[ h ];
	}
	//一旦超出范围。从中减去mCapacity可获得下一个哈希值。
	h -= mCapacity;
	//-1的话这就是哈希值了。
	h -= 1;
	//现在我们知道了哈希值，我们通过从表顶部进行跟踪来找到p。
	int prev = mCapacity + h; //头
	int p = mNexts[ prev ];
	while ( p < mCapacity ){
		if ( p == position ){ //发现。销毁它
			mNexts[ prev ] = mNexts[ p ]; //
			mValues[ p ].~T(); //析构
			mEmptyStack.push( p ); //放入空栈
			--mSize;
			return true;
		}
		prev = p;
		p = mNexts[ p ];
	}
	return false;
}

template< class T, class H > inline const T* HashSet< T, H >::value( int p ) const {
	return &mValues[ p ];
}

template< class T, class H > inline void HashSet< T, H >::clear(){
	//析构
	int prev = mCapacity; //头
	int p = mNexts[ mCapacity ]; //dummy头
	while ( p < mCapacity + mTableSize ){  //mCapacity + mTab​​leSize不存在。到目前为止，它作为虚拟对象存在。
		if ( p < mCapacity ){
			mNexts[ prev ] = mNexts[ p ]; //
			mValues[ p ].~T();
			mEmptyStack.push( p ); //放入空栈
		}
		prev = p;
		p = mNexts[ p ];
	}
	mSize = 0;
}

template< class T, class H > inline int HashSet< T, H >::next( int p ) const {
	ASSERT( p >= 0 );
	while ( p < mCapacity + mTableSize ){
		p = mNexts[ p ];
		if ( p < mCapacity ){ //返回非虚拟节点
			return p;
		}
	}
	return p;
}

template< class T, class H > inline int HashSet< T, H >::first() const {
	int p = mCapacity; //第一个dummy
	while ( p < mCapacity + mTableSize ){
		p = mNexts[ p ];
		if ( p < mCapacity ){ //返回非虚拟节点
			return p;
		}
	}
	return mCapacity + mTableSize;
}

template< class T, class H > inline bool HashSet< T, H >::isEnd( int position ) const {
	return ( position == mCapacity + mTableSize );
}

template< class T, class H > inline void HashSet< T, H >::copyTo( T* a ) const {
	int p = mNexts[ mCapacity ];
	int idx = 0;
	while ( p != mCapacity + mTableSize ){
		if ( p < mCapacity ){
			a[ idx ] = mValues[ p ];
			++idx;
		}
		p = mNexts[ p ];
	}
}

template< class T, class H > inline void HashSet< T, H >::copyTo( Array< T >* a ) const {
	a->setSize( mSize );
	if ( mSize > 0 ){
		copyTo( &( ( *a )[ 0 ] ) );
	}
}


template< class T, class H > inline int HashSet< T, H >::size() const {
	return mSize;
}

} //namespace GameLib

#endif
