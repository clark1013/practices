package server

import (
	"bytes"
	"context"

	"github.com/pingcap-incubator/tinykv/kv/coprocessor"
	"github.com/pingcap-incubator/tinykv/kv/storage"
	"github.com/pingcap-incubator/tinykv/kv/storage/raft_storage"
	"github.com/pingcap-incubator/tinykv/kv/transaction/latches"
	"github.com/pingcap-incubator/tinykv/kv/transaction/mvcc"
	coppb "github.com/pingcap-incubator/tinykv/proto/pkg/coprocessor"
	"github.com/pingcap-incubator/tinykv/proto/pkg/kvrpcpb"
	"github.com/pingcap-incubator/tinykv/proto/pkg/tinykvpb"
	"github.com/pingcap/tidb/kv"
)

var _ tinykvpb.TinyKvServer = new(Server)

// Server is a TinyKV server, it 'faces outwards', sending and receiving messages from clients such as TinySQL.
type Server struct {
	storage storage.Storage

	// (Used in 4B)
	Latches *latches.Latches

	// coprocessor API handler, out of course scope
	copHandler *coprocessor.CopHandler
}

func NewServer(storage storage.Storage) *Server {
	return &Server{
		storage: storage,
		Latches: latches.NewLatches(),
	}
}

// The below functions are Server's gRPC API (implements TinyKvServer).

// Raft commands (tinykv <-> tinykv)
// Only used for RaftStorage, so trivially forward it.
func (server *Server) Raft(stream tinykvpb.TinyKv_RaftServer) error {
	return server.storage.(*raft_storage.RaftStorage).Raft(stream)
}

// Snapshot stream (tinykv <-> tinykv)
// Only used for RaftStorage, so trivially forward it.
func (server *Server) Snapshot(stream tinykvpb.TinyKv_SnapshotServer) error {
	return server.storage.(*raft_storage.RaftStorage).Snapshot(stream)
}

// Transactional API.
func (server *Server) KvGet(_ context.Context, req *kvrpcpb.GetRequest) (*kvrpcpb.GetResponse, error) {
	// Your Code Here (4B).
	reader, err := server.storage.Reader(nil)
	if err != nil {
		return nil, err
	}

	txn := mvcc.NewMvccTxn(reader, req.Version)

	lock, err := txn.GetLock(req.Key)
	if err != nil {
		return nil, err
	}
	if lock != nil && lock.Ts < req.Version {
		return &kvrpcpb.GetResponse{
			Error: &kvrpcpb.KeyError{
				Locked: lock.Info(req.Key),
			},
		}, nil
	}

	value, err := txn.GetValue(req.Key)
	if err != nil {
		return nil, err
	}

	notfound := bytes.Equal(value, []byte{})
	return &kvrpcpb.GetResponse{
		Value:    value,
		NotFound: notfound,
	}, nil
}

func (server *Server) KvPrewrite(_ context.Context, req *kvrpcpb.PrewriteRequest) (*kvrpcpb.PrewriteResponse, error) {
	// Your Code Here (4B).
	reader, err := server.storage.Reader(nil)
	if err != nil {
		return nil, err
	}

	txn := mvcc.NewMvccTxn(reader, req.StartVersion)

	keyErrs := make([]*kvrpcpb.KeyError, 0)
	for _, mutation := range req.Mutations {
		_, version, err := txn.MostRecentWrite(mutation.Key)
		if err != nil {
			return nil, err
		}
		if version > req.StartVersion {
			keyErrs = append(keyErrs, &kvrpcpb.KeyError{
				Conflict: &kvrpcpb.WriteConflict{
					StartTs:    req.StartVersion,
					ConflictTs: version,
					Key:        mutation.Key,
					Primary:    req.PrimaryLock,
				},
			})
			continue
		}
		lock, err := txn.GetLock(mutation.Key)
		if err != nil {
			return nil, err
		}
		if lock != nil {
			keyErrs = append(keyErrs, &kvrpcpb.KeyError{
				Locked: lock.Info(mutation.Key),
			})
			continue
		}

		switch mutation.Op {
		case kvrpcpb.Op_Put:
			txn.PutValue(mutation.Key, mutation.Value)
			txn.PutLock(mutation.Key, &mvcc.Lock{
				Primary: req.PrimaryLock,
				Ts:      req.StartVersion,
				Ttl:     req.LockTtl,
				Kind:    mvcc.WriteKindPut,
			})
		case kvrpcpb.Op_Del:
			txn.DeleteValue(mutation.Key)
			txn.PutLock(mutation.Key, &mvcc.Lock{
				Primary: req.PrimaryLock,
				Ts:      req.StartVersion,
				Ttl:     req.LockTtl,
				Kind:    mvcc.WriteKindDelete,
			})
		}
	}
	if len(keyErrs) > 0 {
		return &kvrpcpb.PrewriteResponse{
			Errors: keyErrs,
		}, nil
	}

	if err := server.storage.Write(req.Context, txn.Writes()); err != nil {
		return nil, err
	}

	return &kvrpcpb.PrewriteResponse{}, nil
}

func (server *Server) KvCommit(_ context.Context, req *kvrpcpb.CommitRequest) (*kvrpcpb.CommitResponse, error) {
	// Your Code Here (4B).
	reader, err := server.storage.Reader(nil)
	if err != nil {
		return nil, err
	}

	txn := mvcc.NewMvccTxn(reader, req.StartVersion)

	for _, key := range req.Keys {
		lock, err := txn.GetLock(key)
		if err != nil {
			return nil, err
		}
		if lock == nil {
			write, _, err := txn.CurrentWrite(key)
			if err != nil {
				return nil, err
			}
			if write != nil && write.StartTS == req.StartVersion && write.Kind == mvcc.WriteKindRollback {
				return &kvrpcpb.CommitResponse{
					Error: &kvrpcpb.KeyError{Retryable: "true"},
				}, nil
			}
			continue
		} else if lock.Ts != req.StartVersion {
			return &kvrpcpb.CommitResponse{
				Error: &kvrpcpb.KeyError{Retryable: "true"},
			}, nil
		}
		txn.PutWrite(key, req.CommitVersion, &mvcc.Write{
			StartTS: lock.Ts,
			Kind:    lock.Kind,
		})
	}

	if err := server.storage.Write(req.Context, txn.Writes()); err != nil {
		return nil, err
	}

	removeLockTxn := mvcc.NewMvccTxn(reader, req.StartVersion)
	for _, key := range req.Keys {
		removeLockTxn.DeleteLock(key)
	}

	if err := server.storage.Write(req.Context, removeLockTxn.Writes()); err != nil {
		return nil, err
	}

	return &kvrpcpb.CommitResponse{}, nil
}

func (server *Server) KvScan(_ context.Context, req *kvrpcpb.ScanRequest) (*kvrpcpb.ScanResponse, error) {
	// Your Code Here (4C).
	return nil, nil
}

func (server *Server) KvCheckTxnStatus(_ context.Context, req *kvrpcpb.CheckTxnStatusRequest) (*kvrpcpb.CheckTxnStatusResponse, error) {
	// Your Code Here (4C).
	return nil, nil
}

func (server *Server) KvBatchRollback(_ context.Context, req *kvrpcpb.BatchRollbackRequest) (*kvrpcpb.BatchRollbackResponse, error) {
	// Your Code Here (4C).
	return nil, nil
}

func (server *Server) KvResolveLock(_ context.Context, req *kvrpcpb.ResolveLockRequest) (*kvrpcpb.ResolveLockResponse, error) {
	// Your Code Here (4C).
	return nil, nil
}

// SQL push down commands.
func (server *Server) Coprocessor(_ context.Context, req *coppb.Request) (*coppb.Response, error) {
	resp := new(coppb.Response)
	reader, err := server.storage.Reader(req.Context)
	if err != nil {
		if regionErr, ok := err.(*raft_storage.RegionError); ok {
			resp.RegionError = regionErr.RequestErr
			return resp, nil
		}
		return nil, err
	}
	switch req.Tp {
	case kv.ReqTypeDAG:
		return server.copHandler.HandleCopDAGRequest(reader, req), nil
	case kv.ReqTypeAnalyze:
		return server.copHandler.HandleCopAnalyzeRequest(reader, req), nil
	}
	return nil, nil
}
