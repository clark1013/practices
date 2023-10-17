#ifndef INCLUDED_GAMELIB_ARRAY2D_H
#define INCLUDED_GAMELIB_ARRAY2D_H

namespace GameLib{

///二维数组
template< class T > class Array2d{
public:
	///默认。尺寸是0。
	Array2d();
	///初始大小。所有元素都用默认构造器初始化。
	Array2d( int size0, int size1 );
	~Array2d();
	///用于后续调整空间。有内容的话将触发ASSERT()。要先执行clear()
	void setSize( int size0, int size1 );
	///获取大小
	int size( int index ) const;
	///全部释放
	void clear();
	///下标索引（const）
	const T& operator()( int i, int j ) const;
	///下标索引（非const）
	T& operator()( int i, int j );
private:
	void operator=( const Array2d& ); //禁止赋值
	Array2d( const Array2d& ); //禁止复制构造函数

	T* mElements;
	int mSize[ 2 ];
};

} //namespace GameLib
#include "GameLib/Base/Impl/Array2dImpl.h" //里面是这个

#endif

