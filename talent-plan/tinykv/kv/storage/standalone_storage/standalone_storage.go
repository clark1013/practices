package standalone_storage

import (
	"github.com/Connor1996/badger"
	"github.com/pingcap-incubator/tinykv/kv/config"
	"github.com/pingcap-incubator/tinykv/kv/storage"
	"github.com/pingcap-incubator/tinykv/kv/util/engine_util"
	"github.com/pingcap-incubator/tinykv/proto/pkg/kvrpcpb"
)

// StandAloneStorage is an implementation of `Storage` for a single-node TinyKV instance. It does not
// communicate with other nodes and all data is stored locally.
type StandAloneStorage struct {
	db   *badger.DB
	conf *config.Config
}

func NewStandAloneStorage(conf *config.Config) *StandAloneStorage {
	return &StandAloneStorage{
		conf: conf,
	}
}

func (s *StandAloneStorage) Start() error {
	opt := badger.DefaultOptions
	opt.Dir = s.conf.DBPath
	opt.ValueDir = s.conf.DBPath
	db, err := badger.Open(opt)
	if err != nil {
		return err
	}
	s.db = db
	return nil
}

func (s *StandAloneStorage) Stop() error {
	if s.db != nil {
		return s.db.Close()
	}
	return nil
}

func (s *StandAloneStorage) Reader(ctx *kvrpcpb.Context) (storage.StorageReader, error) {
	return NewStandAloneReader(s), nil
}

func (s *StandAloneStorage) Write(ctx *kvrpcpb.Context, batch []storage.Modify) (err error) {
	txn := s.db.NewTransaction(true)
	defer txn.Discard()

	for _, m := range batch {
		switch data := m.Data.(type) {
		case storage.Put:
			err = txn.Set(engine_util.KeyWithCF(data.Cf, data.Key), data.Value)
		case storage.Delete:
			err = txn.Delete(engine_util.KeyWithCF(data.Cf, data.Key))
		}
		if err != nil {
			return err
		}
	}
	return txn.Commit()
}

type standaloneReader struct {
	txn *badger.Txn
}

func NewStandAloneReader(storage *StandAloneStorage) *standaloneReader {
	return &standaloneReader{
		txn: storage.db.NewTransaction(false),
	}
}

func (sr *standaloneReader) GetCF(cf string, key []byte) ([]byte, error) {
	iter := sr.IterCF(cf)
	iter.Seek(key)
	if !iter.Valid() {
		return nil, nil
	}
	value, err := iter.Item().Value()
	if err != nil {
		return nil, err
	}
	return value, nil
}

func (sr *standaloneReader) IterCF(cf string) engine_util.DBIterator {
	return engine_util.NewCFIterator(cf, sr.txn)
}

func (sr *standaloneReader) Close() {
	sr.txn.Discard()
}
