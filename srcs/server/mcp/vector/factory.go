package vector

import (
	"database/sql"
)

// NewVectorStorageTransport creates the appropriate provider based on the mode
func NewVectorStorageTransport(mode string, db *sql.DB) VectorStorageProvider {
	if mode == "cloud" {
		return NewPGVectorProvider(db)
	}
	// Default to standalone/sqlite-vss
	return NewSQLiteVSSProvider(db)
}
