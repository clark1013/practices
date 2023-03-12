package main

import (
	"bytes"
	"fmt"
	"math"

	"github.com/pingcap/tidb/domain"
	"github.com/pingcap/tidb/kv"
	"github.com/pingcap/tidb/parser/model"
	"github.com/pingcap/tidb/session"
	kvstore "github.com/pingcap/tidb/store"
	"github.com/pingcap/tidb/store/driver"
	"github.com/pingcap/tidb/table"
	"github.com/pingcap/tidb/table/tables"
	"github.com/pingcap/tidb/tablecodec"
	"github.com/tikv/client-go/v2/txnkv"
)

// create table t (id int, c varchar(64));
// insert into t (id, c) values (1, "a"), (2, "abc"), (3, "abcdef");
func main() {
	// stmts, err := parser.Parse("select * from t")
	// if err != nil {
	// 	panic(err)
	// }
	// fmt.Printf("%v", stmts[0])

	err := kvstore.Register("tikv", driver.TiKVDriver{})
	if err != nil {
		panic(err)
	}
	storage, err := kvstore.New("tikv://127.0.0.1:2379")
	if err != nil {
		panic(err)
	}
	sess, err := session.CreateSession(storage)
	if err != nil {
		panic(err)
	}
	dom := domain.GetDomain(sess)
	tbl, err := dom.InfoSchema().TableByName(model.NewCIStr("test"), model.NewCIStr("t"))
	if err != nil {
		panic(err)
	}
	// column type convert
	cols := make([]*table.Column, len(tbl.Meta().Columns))
	for _, c := range tbl.Meta().Columns {
		cols = append(cols, table.ToColumn(c))
	}
	fmt.Println(cols)

	client, err := txnkv.NewClient([]string{"localhost:2379"})
	if err != nil {
		panic(err)
	}
	txn, err := client.Begin()
	if err != nil {
		panic(err)
	}
	// Refer to `tables.IterRecords`
	prefix := tablecodec.GenTableRecordPrefix(tbl.Meta().ID)
	startKey := tablecodec.EncodeRecordKey(prefix, kv.IntHandle(math.MinInt64))
	it, err := txn.Iter(startKey, prefix.PrefixNext())
	if err != nil {
		panic(err)
	}
	defer it.Close()

	for it.Valid() && bytes.HasPrefix(it.Key(), prefix) {
		// fmt.Printf("%s(% x) \n\t=> %s(% x)\n", it.Key(), it.Key(), it.Value(), it.Value())
		fmt.Printf("%q => %q\n", it.Key(), it.Value())
		rows, _, err := tables.DecodeRawRowData(
			sess,
			tbl.Meta(),
			kv.IntHandle(tbl.Meta().ID),
			cols,
			it.Value(),
		)
		if err != nil {
			panic(err)
		}
		for _, row := range rows {
			fmt.Printf("%v\n", row)
		}

		it.Next()
	}
}
