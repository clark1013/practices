#ifndef INCLUDED_GAMELIB_DEBUGSCREEN_H
#define INCLUDED_GAMELIB_DEBUGSCREEN_H

#include <string>

namespace GameLib{
using namespace std;

class DebugScreen{
public:
	struct Color{
		Color( unsigned c ) : mColor( c ){}
		unsigned mColor;
	};
	DebugScreen();
	static DebugScreen instance();

	int precision( int );
	DebugScreen& operator<<( char );
	DebugScreen& operator<<( unsigned char );
	DebugScreen& operator<<( int );
	DebugScreen& operator<<( unsigned );
	DebugScreen& operator<<( short );
	DebugScreen& operator<<( unsigned short );
	DebugScreen& operator<<( float );
	DebugScreen& operator<<( double );
	DebugScreen& operator<<( const string& );
	DebugScreen& operator<<( const char* );
	DebugScreen& operator<<( char* );
	DebugScreen& operator<<( const Color& );
	DebugScreen& operator<<( DebugScreen& ( *f )( DebugScreen& ) );
	DebugScreen& endl();
	DebugScreen& hex();
	DebugScreen& dec();
	//改变颜色。如果alpha为0，则会将其设置为255，因此可以像0xff4477一样传值
	DebugScreen& setColor( unsigned );
	//移动位置。row指示行开头的位置，并且在发生换行符时从行开始。
	DebugScreen& setPosition( int row, int column );
	//
	void draw( int col, int row, const char* s, unsigned color = 0xffffffff );
};

DebugScreen& endl( DebugScreen& );
DebugScreen& hex( DebugScreen& );
DebugScreen& dec( DebugScreen& );

} //namespace GameLib

#endif
