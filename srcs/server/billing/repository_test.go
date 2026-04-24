package billing

import (
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestPgUsageRepository_ActiveOrganizations(t *testing.T) {
	// Simple test to make sure it compiles
	_ = &PgUsageRepository{
		pool: db.Provider(nil),
	}
}

func TestSqliteUsageRepository_ActiveOrganizations(t *testing.T) {
	// Simple test to make sure it compiles
	_ = &SqliteUsageRepository{
		pool: db.Provider(nil),
	}
}
