package vector

import (
	"database/sql"
	"fmt"
)

// NewVectorStorageTransport creates and returns a VectorStorageProvider
// based on the provided mode string ("cloud" or "standalone").
func NewVectorStorageTransport(mode string, db *sql.DB) (VectorStorageProvider, error) {
	switch mode {
	case "cloud":
		return newPGVectorProvider(db), nil
	case "standalone":
		return newSQLiteProvider(db), nil
	default:
		return nil, fmt.Errorf("unsupported vector storage mode: %s", mode)
	}
}
