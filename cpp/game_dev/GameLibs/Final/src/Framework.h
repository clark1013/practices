#ifndef INCLUDED_GAMELIB_FRAMEWORK_H
#define INCLUDED_GAMELIB_FRAMEWORK_H

#include <string>
using namespace std;

namespace GameLib{

namespace Graphics{
	class Texture;
}

namespace Math{
	class Vector3;
	class Vector2;
	class Matrix44;
}

class Framework{
public:
	enum LoadMode{
		LOAD_ARCHIVE_FIRST,
		LOAD_DIRECT_FIRST,
		LOAD_ARCHIVE_ONLY,
	};
	class Configuration{
	public:
		///宽度设置
		void setWidth( int );
		///高度设置
		void setHeight( int );
		///设置窗口标题
		void setTitle( const char* title );
		///垂直同步有效化设置
		void enableVSync( bool = true );
		///全屏幕抗锯齿设置
		void enableAntiAlias( bool = true );
		///启动时设置全屏标志
		void enableFullScreen( bool = true );
		///禁止全屏化
		void forbidFullScreen( bool = true );
		///启用拖放
		void enableDragAndDrop( bool = true );
		///确定存档数
		void setArchiveNumber( int );
		///存档名称设置
		void setArchiveName( int index, const char* );
		///加载模式设置
		void setLoadMode( LoadMode );
		///载入什么文件
		void loadFile( string* out, const char* filename ) const;
		///获取命令行选项
		const char* commandLineString() const;
		//HALT、ASSERT时不会立刻结束而是抛出异常。这是为了避免在商业作品中出现程序崩溃而设置
		void enableThrowWhenHalt( bool = true );
		///设置追加的线程数
		void setExtraThreadNumber( int );
	};
	Framework();
	///获取实例
	static Framework instance();

	///用户定义函数。如果没有创建则会产生链接错误
	void configure( Configuration* );
	///用户定义函数。
	void update(); 

	///设置全屏标志。
	void enableFullScreen( bool );
	///获取画面宽度
	int width() const;
	///获取画面高度
	int height() const;
	///获取应用程序名称
	const char* title() const;
	///全屏吗？
	bool isFullScreen() const;
	///禁止全屏化吗？
	bool isFullScreenForbidden() const;
	///拖放有效吗？
	bool isDragAndDropEnabled() const;
	//向Framework发出结束命令
	void requestEnd();
	//检查Framework是否发出结束命令
	bool isEndRequested() const;
	///获取时间（以毫秒计）
	unsigned time() const;
	///获得帧速率
	int frameRate() const;
	///帧速率设置（内部可循环等待）
	void setFrameRate( int );
	///获取上一帧的时间（以毫秒为单位）
	int previousFrameInterval() const;

	//以下库用户不需要知道
	void start( void* windowHandle );
	void preUpdate();
	void postUpdate();
	static void create();
	static void destroy();
};

} //namespace GameLib

//开启基本服务
#include "GameLib/GameLib.h"

#endif
