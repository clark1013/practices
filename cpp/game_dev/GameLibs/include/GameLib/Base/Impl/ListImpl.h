#ifndef INCLUDED_GAMELIB_LISTIMPL_H
#define INCLUDED_GAMELIB_LISTIMPL_H

#include "GameLib/GameLib.h"
#include "GameLib/Base/List.h"


namespace GameLib{

//要做一些工作，以使开头和结尾不需要特别处理。
template< class T > inline List< T >::List() : 
mValues( 0 ),
mNodes( 0 ),
mCapacity( 0 ),
mSize( 0 ){
}

//要做一些工作，以使开头和结尾不需要特别处理。
template< class T > inline List< T >::List( int capacity ) : 
mValues( 0 ),
mNodes( 0 ),
mCapacity( 0 ),
mSize( 0 ){
	setCapacity( capacity );
}

template< class T > inline List< T >::~List(){
	if ( mNodes ){
		//销毁单体
		int pos = mNodes[ mCapacity ].mNext;
		while ( pos < mCapacity ){ //如果不是空的
			mValues[ pos ].~T(); //析构
			pos = mNodes[ pos ].mNext;
		}
		OPERATOR_DELETE( mValues ); //直接delete
		SAFE_DELETE_ARRAY( mNodes );
	}
}

template< class T > inline void List< T >::setCapacity( int capacity ){
	ASSERT( mSize == 0 && "NOT EMPTY! call clear()." );
	//先清除
	if ( mCapacity > 0 ){
		mEmptyStack.clear();
		clear();
		OPERATOR_DELETE( mValues ); //直接delete
		SAFE_DELETE_ARRAY( mNodes );
	}
	//重新分配
	mCapacity = capacity;
	if ( capacity <= 0 ){ //忽略0
		return;
	}
	mNodes = NEW Node[ mCapacity + 1 ];
	//分配节点数组。直接使用malloc，因为不用构造函数
	mValues = static_cast< T* >( OPERATOR_NEW( sizeof( T ) * mCapacity ) );
	//存储空的编号
	mEmptyStack.setCapacity( mCapacity );
	for ( int i = 0; i < mCapacity; ++i ){
		mEmptyStack.push( i ); //放入空编号列表
	}
	//mCapacity编号是特殊的。虚拟元素在第一个元素之前和最后一个元素之后
	mNodes[ mCapacity ].mNext = mNodes[ mCapacity ].mPrev = mCapacity;
}

template< class T > inline int List< T >::capacity() const {
	return mCapacity;
}

template< class T > inline int List< T >::addAfter( int position, const T& v ){
	ASSERT( position >= 0 && position <= mCapacity ); //可能相等
	//获得可用空间
	int newPos;
	mEmptyStack.pop( &newPos );
	//添加的地方是addPosition
	Node* e = &mNodes[ newPos ];
	//取出指定的物体
	Node* cur = &mNodes[ position ];
	//取出下一个家伙
	Node* next = &mNodes[ cur->mNext ];
	//值存储（复制构造函数调用）
	new ( &mValues[ newPos ] ) T( v );
	e->mPrev = position; //在指定地点之前
	e->mNext = cur->mNext; //
	//
	cur->mNext = newPos;
	//
	next->mPrev = newPos;
	++mSize;

	return newPos;
}

template< class T > inline int List< T >::addBefore( int position, const T& v ){
	ASSERT( position >= 0 && position <= mCapacity ); //可能相等
	//获得可用空间
	int newPos;
	mEmptyStack.pop( &newPos );
	//添加的地方是addPosition
	Node* e = &mNodes[ newPos ];
	//取出指定的物体
	Node* cur = &mNodes[ position ];
	//取出前一个
	Node* prev = &mNodes[ cur->mPrev ];
	//值存储（构造函数调用）
	new ( &mValues[ newPos ] ) T( v );
	e->mPrev = cur->mPrev; //
	e->mNext = position; //下一个指定的
	//
	cur->mPrev = newPos;
	//
	prev->mNext = newPos;
	++mSize;

	return newPos;
}

template< class T > inline int List< T >::addHead( const T& v ){
	return addAfter( mCapacity, v ); //借助于dummy元素可以写
}

template< class T > inline int List< T >::addTail( const T& v ){
	return addBefore( mCapacity, v ); //借助于dummy元素可以写
}

template< class T > inline void List< T >::remove( int position ){
	ASSERT( mSize > 0 );
	//指定
	Node* cur = &mNodes[ position ];
	//下一个
	Node* next = &mNodes[ cur->mNext ];
	//前
	Node* prev = &mNodes[ cur->mPrev ];
	//上一个，下一个。
	prev->mNext = cur->mNext;
	//
	next->mPrev = cur->mPrev;
	//明确调用析构函数
	mValues[ position ].~T();
	//尺寸缩小
	--mSize;
	//放入空栈
	mEmptyStack.push( position );
}

template< class T > inline void List< T >::removeHead(){ //等同于别名。
	int pos = mNodes[ mCapacity ].mNext;
	if ( pos < mCapacity ){ //无法删除dummy
		remove( pos  );
	}
}

template< class T > inline void List< T >::removeTail(){ //等同于别名。
	int pos = mNodes[ mCapacity ].mPrev;
	if ( pos < mCapacity ){ //无法删除dummy
		remove( pos  );
	}
}

template< class T> inline void List< T >::clear(){
	int pos = mNodes[ mCapacity ].mNext;
	while ( pos < mCapacity ){ //如果不是空的
		mValues[ pos ].~T(); //析构
		pos = mNodes[ pos ].mNext;
	}
	mNodes[ mCapacity ].mNext = mCapacity;
	mNodes[ mCapacity ].mPrev = mCapacity;
	mSize = 0;
	//
	mEmptyStack.clear();
	for ( int i = 0; i < mCapacity; ++i ){
		mEmptyStack.push( i ); //放入空编号列表
	}
}

template< class T > inline const T* List< T >::value( int position ) const {
	ASSERT( position >= 0 && position < mCapacity );
	return &mValues[ position ];
}

template< class T > inline T* List< T >::value( int position ){
	ASSERT( position >= 0 && position < mCapacity );
	return &mValues[ position ];
}

template< class T > inline int List< T >::next( int position ) const {
	ASSERT( position >= 0 && position <= mCapacity ); //
	return mNodes[ position ].mNext;
}

template< class T > inline int List< T >::previous( int position ) const {
	ASSERT( position >= 0 && position <= mCapacity );//
	return mNodes[ position ].mPrev;
}

template< class T > inline int List< T >::first() const {
	return next( mCapacity );
}

template< class T > inline int List< T >::last() const {
	return getPrev( mCapacity );
}

template< class T > inline bool List< T >::isEnd( int position ) const {
	return ( position == mCapacity );
}

template< class T > inline void List< T >::copyTo( T* a ) const {
	int p = mNodes[ mCapacity ].mNext;
	int idx = 0;
	while ( p != mCapacity ){
		a[ idx ] = mValues[ p ];;
		++idx;
		p = mNodes[ p ].mNext;
	}
}

template< class T > inline void List< T >::copyTo( Array< T >* a ) const {
	a->setSize( mSize );
	if ( mSize > 0 ){
		copyTo( &( ( *a )[ 0 ] ) );
	}
}
		




} //namespace GameLib

#endif
