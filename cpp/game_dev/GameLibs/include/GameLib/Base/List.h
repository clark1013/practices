#ifndef INCLUDED_GAMELIB_LIST_H
#define INCLUDED_GAMELIB_LIST_H

#include "GameLib/Base/Stack.h"
#include "GameLib/Base/Array.h"

namespace GameLib{

//考虑到new的开销而使用双向链表
template< class T > class List{
public:
	List();
	explicit List( int capacity );
	~List();
	///之后指定大小。如果内部有数据则中止。必须先执行clear()
	void setCapacity( int capacity );
	int capacity() const;
	//追加 返回所有位置
	int addBefore( int position, const T& value ); //在指定位置之前添加
	int addAfter( int position, const T& value ); //在指定位置后添加
	int addHead( const T& value ); //添加头
	int addTail( const T& value ); //添加末尾
	//删掉
	void remove( int position ); //删除指定位置
	void removeHead(); //删除开头
	void removeTail(); //去除尾巴
	void clear(); //全部删除
	///获取值（const）
	const T* value( int position ) const;
	///获取值（非const）
	T* value( int position );
	///通过在isEnd()确认是否是终端
	int next( int position ) const;
	///通过isEnd（）检查是否在结尾
	int previous( int position ) const;
	int first() const;
	int last() const;
	//判断position是否为结尾。
	bool isEnd( int position ) const;
	///转换成简单的数组。根据size()的尺寸申请的空间
	void copyTo( T* ) const;
	//复制到Arry。允许传递空
	void copyTo( Array< T >* ) const;
private:
	struct Node{
		int mNext;
		int mPrev;
	};
	List( const List& ); //禁止复制构造函数
	void operator=( const List& ); //禁止赋值

	T* mValues;
	Node* mNodes;
	Stack< int > mEmptyStack;
	int mCapacity;
	int mSize;
};

} //namespace GameLib
#include "GameLib/Base/Impl/ListImpl.h" //里面是这个

#endif

