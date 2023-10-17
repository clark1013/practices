#include "GameLib/GameLib.h"
#include "GameLib/WindowCreator/WindowCreator.h"
#include "GameLib/Input/Manager.h"
#include "GameLib/FileIO/Manager.h"
#include "GameLib/Sound/Manager.h"
#include "GameLib/Graphics/Manager.h"
#include "GameLib/Graphics/Texture.h"
#include "GameLib/Math/Vector2.h"
#include "GameLib/Scene/StringRenderer.h"
#include "GameLib/Scene/Font.h"
#include "GameLib/Threading/Functions.h"
#include "GameLib/Threading/Manager.h"

#include "Framework.h"
#include "DebugScreenImpl.h"

#include <string>
#include <fstream>
using namespace std;

namespace GameLib{

namespace { //匿名

class Impl{
public:
	Impl() :
	mWidth( 640 ),
	mHeight( 480 ),
	mFullScreen( false ),
	mFullScreenForbidden( false ),
	mDragAndDropEnabled( false ),
	mVSync( true ),
	mAntiAlias( true ),
	mTitle( "A GameLib Framework Application" ),
	mArchiveNames( 0 ),
	mArchiveNumber( 0 ),
	mLoadMode( FileIO::Manager::MODE_DIRECT_FIRST ),
	mExtraThreadNumber( 0 ),
	mPreviousFrameInterval( 0 ),
	mPreviousFrameTime( 0 ),
	mFrameRate( 0 ),
	mIdealFrameInterval( 0 ),
	mEndRequested( false ),
	mStarted( false ){
		//cout初始化
		cout.begin();
		//帧历史记录重置
		unsigned t = time();
		for ( int i = 0; i < TIME_HISTORY_SIZE; ++i ){
			mTimeHistory[ i ] = t;
		}
	}
	~Impl(){
		if ( mArchiveNames ){
			SAFE_DELETE_ARRAY( mArchiveNames );
		}
		SAFE_DELETE( gDebugScreenImpl );

		Graphics::Manager::destroy();
		Input::Manager::destroy();
		Sound::Manager::destroy();
		FileIO::Manager::destroy();
		Threading::Manager::destroy();
		//cout结束
		cout.end();
	}
	void start( void* windowHandle ){
		//线程系统初始化
		Threading::Manager::create( mExtraThreadNumber );
		//初始化文件加载器
		if ( mArchiveNumber > 0 ){
			Array< const char* > names( mArchiveNumber );
			for ( int i = 0; i < mArchiveNumber; ++i ){
				names[ i ] = mArchiveNames[ i ].c_str();
			}
			FileIO::Manager::create( &names[ 0 ], mArchiveNumber, mLoadMode );
			//丢弃，因为不需要归档名称数组
			SAFE_DELETE_ARRAY( mArchiveNames );
			mArchiveNumber = 0;
		}else{
			FileIO::Manager::create();
		}
		//声音初始化
		Sound::Manager::create( windowHandle );
		//输入初始化
		Input::Manager::create( windowHandle );
		//初始化绘制
		Graphics::Manager::create( windowHandle, mWidth, mHeight, mFullScreen, mVSync, mAntiAlias );
		//绘制文字
		gDebugScreenImpl = NEW DebugScreenImpl;

		mStarted = true;
	}
	unsigned time() const {
		return WindowCreator().time();
	}
	void preUpdate(){
		unsigned currentTime = time();
		while ( ( currentTime - mPreviousFrameTime ) < mIdealFrameInterval ){
			Threading::sleep( 1 );
			currentTime = time();
		}
		mPreviousFrameTime = currentTime;

		//帧时间更新
		mPreviousFrameInterval = currentTime - mTimeHistory[ TIME_HISTORY_SIZE - 1 ];
		unsigned frameIntervalSum = currentTime - mTimeHistory[ 0 ];
		mFrameRate = TIME_HISTORY_SIZE * 1000 / frameIntervalSum;
		//更新历史
		for ( int i = 0; i < TIME_HISTORY_SIZE - 1; ++i ){
			mTimeHistory[ i ] = mTimeHistory[ i + 1 ]; 
		}
		mTimeHistory[ TIME_HISTORY_SIZE - 1 ] = currentTime;

		Graphics::Manager().beginDraw();
		float pointerScale;
		Vector2 pointerOffset;
		Graphics::Manager().getPointerModifier( &pointerScale, &pointerOffset );
		Input::Manager().update( pointerScale, pointerOffset );
	}
	void postUpdate(){
		gDebugScreenImpl->draw(); //绘制文字

		Graphics::Manager().endDraw();
	}
	int mWidth;
	int mHeight;
	bool mFullScreen;
	bool mFullScreenForbidden;
	bool mDragAndDropEnabled;
	bool mVSync;
	bool mAntiAlias;
	string mTitle;
	string* mArchiveNames;
	int mArchiveNumber;
	FileIO::Manager::AccessMode mLoadMode;
	int mExtraThreadNumber;
	static const int TIME_HISTORY_SIZE = 60;
	unsigned mTimeHistory[ TIME_HISTORY_SIZE ];
	int mPreviousFrameInterval;
	unsigned mPreviousFrameTime;
	int mFrameRate;
	unsigned mIdealFrameInterval;
	bool mEndRequested;
	bool mStarted;
};

Impl* gImpl = 0;

} //namespace{}

Framework::Framework(){
	//不允许来自其他线程的调用
	ASSERT( WindowCreator::isMainThread() && "you must call from MAIN thread" );
}

void Framework::create(){
	ASSERT( !gImpl );
	gImpl = NEW Impl();
}

void Framework::destroy(){
	ASSERT( gImpl );
	SAFE_DELETE( gImpl );
}

Framework Framework::instance(){
	return Framework();
}

void Framework::start( void* windowHandle ){
	gImpl->start( windowHandle );
}

void Framework::preUpdate(){
	gImpl->preUpdate();
}

void Framework::postUpdate(){
	gImpl->postUpdate();
}

int Framework::width() const {
	return gImpl->mWidth;
}

int Framework::height() const {
	return gImpl->mHeight;
}

const char* Framework::title() const {
	return gImpl->mTitle.c_str();
}

bool Framework::isFullScreen() const {
	return gImpl->mFullScreen;
}

bool Framework::isFullScreenForbidden() const {
	return gImpl->mFullScreenForbidden;
}

bool Framework::isDragAndDropEnabled() const {
	return gImpl->mDragAndDropEnabled;
}

void Framework::requestEnd(){
	gImpl->mEndRequested = true;
}

bool Framework::isEndRequested() const {
	return gImpl->mEndRequested;
}

unsigned Framework::time() const {
	return gImpl->time();
}

int Framework::previousFrameInterval() const {
	return gImpl->mPreviousFrameInterval;
}

int Framework::frameRate() const {
	return gImpl->mFrameRate;
}

void Framework::setFrameRate( int fr ){
	gImpl->mIdealFrameInterval = 1000 / fr;
}

void Framework::enableFullScreen( bool f ){ 
	if ( gImpl->mFullScreenForbidden ){ //被禁止
		return;
	}
	WindowCreator wc;
	Graphics::Manager gm;
	if ( gImpl->mStarted ){
		//中途更改
		if ( gImpl->mFullScreen != f ){
			//上游
			wc.enableFullScreen( f );
			//下游
			gm.enableFullScreen( f );
		}
	}
	gImpl->mFullScreen = f;
}

//Configuration
void Framework::Configuration::setWidth( int width ){
	gImpl->mWidth = width;
}

void Framework::Configuration::setHeight( int height ){
	gImpl->mHeight = height;
}

void Framework::Configuration::setTitle( const char* title ){
	gImpl->mTitle = title;
}

void Framework::Configuration::setArchiveNumber( int n ){
	ASSERT( !gImpl->mArchiveNames && "you can't call twice." );
	gImpl->mArchiveNumber = n;
	gImpl->mArchiveNames = NEW string[ n ];
}

void Framework::Configuration::setArchiveName( int index, const char* name ){
	ASSERT( gImpl->mArchiveNames && "you must call setArchiveNumber() before this call." );
	ASSERT( index >= 0 && index < gImpl->mArchiveNumber );
	gImpl->mArchiveNames[ index ] = name;
}

void Framework::Configuration::setLoadMode( LoadMode lm ){
	using namespace FileIO;
	Manager::AccessMode am = Manager::MODE_DIRECT_FIRST;
	switch ( lm ){
		case LOAD_ARCHIVE_FIRST: am = Manager::MODE_ARCHIVE_FIRST; break;
		case LOAD_DIRECT_FIRST: am = Manager::MODE_DIRECT_FIRST; break;
		case LOAD_ARCHIVE_ONLY: am = Manager::MODE_ARCHIVE_ONLY; break;
		default: ASSERT( false ); break;
	}
	gImpl->mLoadMode = am;
}

void Framework::Configuration::enableFullScreen( bool f ){
	gImpl->mFullScreen = f;
}

void Framework::Configuration::enableVSync( bool f ){
	gImpl->mVSync = f;
}

void Framework::Configuration::enableAntiAlias( bool f ){
	gImpl->mAntiAlias = f;
}

void Framework::Configuration::forbidFullScreen( bool f ){
	gImpl->mFullScreenForbidden = f;
}

void Framework::Configuration::enableDragAndDrop( bool f ){
	gImpl->mDragAndDropEnabled = f;
}

void Framework::Configuration::loadFile( string* out, const char* filename ) const {
	ifstream in( filename, ifstream::binary );
	in.seekg( 0, ifstream::end );
	streamsize size = in.tellg();
	in.seekg( 0, ifstream::beg );
	char* s = new char[ static_cast< unsigned >( size ) + 1 ];
	in.read( s, size );
	s[ size ] = '\0';
	*out = s;
}

const char* Framework::Configuration::commandLineString() const {
	return WindowCreator().commandLineString();
}

void Framework::Configuration::enableThrowWhenHalt( bool f ){
	extern bool gThrowWhenHalt;
	gThrowWhenHalt = f;
}

void Framework::Configuration::setExtraThreadNumber( int n ){
	STRONG_ASSERT( n >= 0 && "extraThreadNumber must be >= 0" );
	gImpl->mExtraThreadNumber = n;
}

//WindowProcedure的用户封装函数
void WindowCreator::configure( Configuration* config ){
	Framework::create();
	//用户设定
	Framework f;
	Framework::Configuration fwConfig;
	f.configure( &fwConfig );
	config->setWidth( f.width() );
	config->setHeight( f.height() );
	config->setTitle( f.title() );
	config->enableFullScreen( f.isFullScreen() );
	config->forbidFullScreen( f.isFullScreenForbidden() );
	config->enableDragAndDrop( f.isDragAndDropEnabled() );
}

void WindowCreator::update(){
	WindowCreator wc;
	Graphics::Manager gm;
	Framework f;
	//根据WindowCreator的状态执行各种操作
	bool sleepFlag = false;
	//最小化后则休眠
	if ( wc.isMinimized() ){
		sleepFlag = true;
	}
	//基本上会进入睡眠状态。
	if ( !gm.canRender() ){
		gm.restore();
		sleepFlag = true;
	}
	//来自Window的模式切换信号
	bool wcFullScreen = wc.isFullScreen();
	if ( f.isFullScreen() != wcFullScreen ){
		f.enableFullScreen( wcFullScreen );
	}
	if ( !sleepFlag ){
		f.preUpdate();
	}
	//检查结束命令是否来自上游（例如x按钮）
	if ( isEndRequested() ){ //收到后则将终止命令发送到Framework
		f.requestEnd();
	}
	if ( sleepFlag ){
		Threading::sleep( 50 ); //睡50ms
	}else{
		//从用户代码捕获异常时发送退出信号
		try{
			f.update();
		}
		catch ( ... ){
			f.postUpdate();
			Framework::destroy();
			end(); //让上游知道
			throw EXCEPTION_EXIT;
		}
		f.postUpdate();
	}
	//结束判断
	if ( f.isEndRequested() ){
		Framework::destroy();
		end(); //让上游知道
	}
}

void WindowCreator::start( void* windowHandle ){
	Framework().start( windowHandle );
}

} //namespace GameLib
