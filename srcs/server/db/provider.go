package db

import "os"
import "context"

type Database interface {
	Exec(ctx context.Context, query string, args ...interface{}) (interface{}, error)
}

type Provider struct{}

func (p *Provider) IsSQLite() bool {
	return os.Getenv("OHC_STANDALONE") == "true"
}

var GlobalProvider = &Provider{}
