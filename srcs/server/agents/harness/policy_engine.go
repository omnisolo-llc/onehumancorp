package harness

import (
	"context"
	"fmt"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type PolicyEngine struct {
	dbProvider db.Provider
}

func NewPolicyEngine(dbProvider db.Provider) *PolicyEngine {
	return &PolicyEngine{dbProvider: dbProvider}
}

func (p *PolicyEngine) CheckPolicy(ctx context.Context, command string) (bool, error) {
	if p.dbProvider == nil {
		return false, fmt.Errorf("db provider is nil")
	}

	query := `SELECT action FROM execution_policies WHERE $1 LIKE pattern LIMIT 1`

	rows, err := p.dbProvider.Query(ctx, query, command)
	if err != nil {
		if strings.Contains(err.Error(), "no such table") || strings.Contains(err.Error(), "does not exist") {
			return !strings.Contains(command, "rm -rf"), nil
		}
		return false, err
	}
	defer rows.Close()

	if rows.Next() {
		var action string
		if err := rows.Scan(&action); err != nil {
			return false, err
		}
		return action == "ALLOW", nil
	}

	return false, nil
}
