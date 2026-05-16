package crdtmcp

import (
	"database/sql"
)

func NewProvider(db *sql.DB) Provider {
	return &LocalProvider{DB: db}
}
