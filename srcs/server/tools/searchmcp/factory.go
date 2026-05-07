package searchmcp

import (
	"database/sql"
	"os"
)

func NewProvider(db *sql.DB) SearchProvider {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	if isStandalone {
		return NewLocalSearchProvider(db)
	}
	return NewCloudSearchProvider(db)
}
