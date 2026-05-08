package db

import "context"

type Database interface {
	Exec(ctx context.Context, sql string, arguments ...interface{}) (interface{}, error)
	Query(ctx context.Context, sql string, args ...interface{}) (interface{}, error)
	QueryRow(ctx context.Context, sql string, args ...interface{}) interface{}
	Begin(ctx context.Context) (interface{}, error)
	Close()
}
