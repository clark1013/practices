package main

import (
	"context"
	"fmt"
	"net"
	"net/http"
	"reflect"

	pb "github.com/clark1013/grpc-demo/gen/go"
	grpc_middleware "github.com/grpc-ecosystem/go-grpc-middleware"
	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"github.com/pingcap/errcode"
	grpcerr "github.com/pingcap/errcode/grpc"
	"github.com/pingcap/log"
	"github.com/tidbcloud/pkgv2/bizerr"
	grpcbizerr "github.com/tidbcloud/pkgv2/grpc/middleware/bizerr"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/status"
)

type server struct {
	pb.UnimplementedTestServiceServer
}

// MARK: 业务中返回 error
func (s *server) Echo(ctx context.Context, t *pb.StringMessage) (*pb.TestResponse, error) {
	// return t, nil
	// return &pb.TestResponse{}, errcode.NewForbiddenErr(bizerr.New(bizerr.ErrCodeGeneralBusiness, "1234567"))
	// return &pb.TestResponse{}, bizerr.New(bizerr.ErrCodeGeneralBusiness, "1234567")
	// return nil, errcode.NewForbiddenErr(bizerr.New(bizerr.ErrCodeGeneralBusiness, "1234567"))
	return nil, bizerr.New(bizerr.ErrCodeGeneralBusiness, "1234567")
	// return nil, status.Error(4000321, "fhdkfdksfe")
}

func errorConvert(err error) error {
	if err == nil {
		return nil
	}

	if coded := errcode.CodeChain(err); coded != nil {
		return grpcerr.WrapAsGRPC(coded)
	}
	return err
}

func UnaryServerInterceptor() grpc.UnaryServerInterceptor {
	return func(ctx context.Context, req interface{}, info *grpc.UnaryServerInfo, handler grpc.UnaryHandler) (interface{}, error) {
		resp, err := handler(ctx, req)
		return resp, errorConvert(err)
	}
}

func main() {
	s := grpc.NewServer(
		grpc.UnaryInterceptor(
			grpc_middleware.ChainUnaryServer(
				// MARK: 解析 bizerr
				grpcbizerr.UnaryServerInterceptor(func(resp interface{}, err bizerr.BizError) (resp_ interface{}, err_ error) {
					// typed := getBaseRespFromResp(resp)
					// if typed == nil {
					// 	return resp, err
					// }
					// typed.ErrCode = int64(err.ErrCode())
					// typed.ErrMsg = err.ErrMsg()
					// return resp, nil
					return resp, status.Error(codes.Code(err.ErrCode()), err.ErrMsg())
				}),
				UnaryServerInterceptor(),
			),
		),
	)
	pb.RegisterTestServiceServer(s, &server{})
	addr := "0.0.0.0:9200"
	go func() {
		log.Info("serving gRPC", zap.String("address", addr))
		lis, err := net.Listen("tcp", addr)
		if err != nil {
			panic(err)
		}
		err = s.Serve(lis)
		if err != nil {
			panic(err)
		}
	}()
	// MARK: 自定义错误处理
	mux := runtime.NewServeMux(
		runtime.WithErrorHandler(func(ctx context.Context, mux *runtime.ServeMux, marshaler runtime.Marshaler, writer http.ResponseWriter, request *http.Request, err error) {
			s := status.Convert(err)
			fmt.Println(s.Code())
			fmt.Println(s.Message())
			newError := runtime.HTTPStatusError{
				HTTPStatus: 400,
				Err:        err,
			}
			runtime.DefaultHTTPErrorHandler(ctx, mux, marshaler, writer, request, &newError)
		}),
	)
	opts := []grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())}
	err := pb.RegisterTestServiceHandlerFromEndpoint(context.Background(), mux, addr, opts)
	if err != nil {
		panic("111")
	}
	http.ListenAndServe(":8081", mux)
}

// TODO(chenfei): merge with idl/pbgen/util/helper.go:GetBaseRespFromResp
func getBaseRespFromResp(resp interface{}) *pb.BaseResp {
	if resp == nil {
		return nil
	}

	baseResp := reflect.ValueOf(resp).Elem().FieldByName("BaseResp")
	if !baseResp.IsValid() { // don't have BaseReq field
		return nil
	}
	if baseResp.IsNil() {
		baseResp.Set(reflect.New(baseResp.Type().Elem()))
	}

	typed, ok := baseResp.Interface().(*pb.BaseResp)
	if !ok {
		return nil
	}
	return typed
}
