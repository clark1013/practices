package server

import (
	"context"

	"github.com/pingcap-incubator/tinykv/kv/storage"
	"github.com/pingcap-incubator/tinykv/proto/pkg/kvrpcpb"
)

// The functions below are Server's Raw API. (implements TinyKvServer).
// Some helper methods can be found in sever.go in the current directory

// RawGet return the corresponding Get response based on RawGetRequest's CF and Key fields
func (server *Server) RawGet(_ context.Context, req *kvrpcpb.RawGetRequest) (*kvrpcpb.RawGetResponse, error) {
	reader, err := server.storage.Reader(nil)
	if err != nil {
		return nil, err
	}
	value, err := reader.GetCF(req.Cf, req.Key)
	if err != nil {
		return nil, err
	}
	var notfound bool
	if value == nil {
		notfound = true
	}
	return &kvrpcpb.RawGetResponse{
		Value:    value,
		NotFound: notfound,
	}, nil
}

// RawPut puts the target data into storage and returns the corresponding response
func (server *Server) RawPut(_ context.Context, req *kvrpcpb.RawPutRequest) (*kvrpcpb.RawPutResponse, error) {
	modify := storage.Modify{
		Data: storage.Put{
			Key:   req.Key,
			Value: req.Value,
			Cf:    req.Cf,
		},
	}
	err := server.storage.Write(nil, []storage.Modify{modify})
	if err != nil {
		return nil, err
	}
	return &kvrpcpb.RawPutResponse{}, nil
}

// RawDelete delete the target data from storage and returns the corresponding response
func (server *Server) RawDelete(_ context.Context, req *kvrpcpb.RawDeleteRequest) (*kvrpcpb.RawDeleteResponse, error) {
	modify := storage.Modify{
		Data: storage.Delete{
			Key: req.Key,
			Cf:  req.Cf,
		},
	}
	err := server.storage.Write(nil, []storage.Modify{modify})
	if err != nil {
		return nil, err
	}
	return &kvrpcpb.RawDeleteResponse{}, nil
}

// RawScan scan the data starting from the start key up to limit. and return the corresponding result
func (server *Server) RawScan(_ context.Context, req *kvrpcpb.RawScanRequest) (*kvrpcpb.RawScanResponse, error) {
	reader, err := server.storage.Reader(nil)
	if err != nil {
		return nil, err
	}
	iter := reader.IterCF(req.Cf)
	iter.Seek(req.StartKey)

	var size uint32
	kvs := make([]*kvrpcpb.KvPair, 0, req.Limit)
	for iter.Valid() && size < req.Limit {
		item := iter.Item()
		value, err := item.Value()
		if err != nil {
			return nil, err
		}
		kvs = append(kvs, &kvrpcpb.KvPair{Key: item.Key(), Value: value})
		iter.Next()
		size++
	}
	return &kvrpcpb.RawScanResponse{Kvs: kvs}, nil
}
