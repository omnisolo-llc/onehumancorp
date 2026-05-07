package db

import "os"

type Provider struct{}

func (p *Provider) IsSQLite() bool {
	return os.Getenv("OHC_STANDALONE") == "true"
}

var GlobalProvider = &Provider{}
