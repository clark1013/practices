package parser

import (
	"github.com/pingcap/log"
	tidbparser "github.com/pingcap/tidb/parser"
	"github.com/pingcap/tidb/parser/ast"
	_ "github.com/pingcap/tidb/types/parser_driver"
	"go.uber.org/zap"
)

func Parse(sql string) ([]ast.StmtNode, error) {
	p := tidbparser.New()
	stmts, warns, err := p.ParseSQL(sql)
	for _, w := range warns {
		log.L().Warn("parse sql warn", zap.Error(w))
	}
	if err != nil {
		return nil, err
	}
	return stmts, nil
}
