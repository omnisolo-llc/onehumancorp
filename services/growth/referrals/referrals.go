package referrals

import (
	"context"
	"crypto/rand"
	"database/sql"
	"encoding/hex"
	"errors"
	"fmt"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var meter = otel.Meter("growth.referrals")

type ReferralSystem struct {
	db *sql.DB

	generatedTotal metric.Int64Counter
	usedTotal      metric.Int64Counter
}

func NewReferralSystem(db *sql.DB) (*ReferralSystem, error) {
	generatedTotal, err := meter.Int64Counter(
		"referrals.generated.total",
		metric.WithDescription("Total number of referral codes generated"),
	)
	if err != nil {
		return nil, err
	}

	usedTotal, err := meter.Int64Counter(
		"referrals.used.total",
		metric.WithDescription("Total number of referral codes used"),
	)
	if err != nil {
		return nil, err
	}

	return &ReferralSystem{
		db:             db,
		generatedTotal: generatedTotal,
		usedTotal:      usedTotal,
	}, nil
}

// isSQLite is a helper function to determine the dialect, as OHC uses both PostgreSQL and SQLite.
func (rs *ReferralSystem) isSQLite() bool {
	driverName := fmt.Sprintf("%T", rs.db.Driver())
	return driverName == "*sqlite.Driver" || driverName == "*sqlite3.SQLiteDriver"
}

func (rs *ReferralSystem) GenerateCode(ctx context.Context, tenantID, userID string) (string, error) {
	bytes := make([]byte, 8)
	if _, err := rand.Read(bytes); err != nil {
		return "", err
	}
	code := hex.EncodeToString(bytes)

	var err error
	if rs.isSQLite() {
		_, err = rs.db.ExecContext(ctx, "INSERT INTO referrals (tenant_id, code, user_id, usages) VALUES (?, ?, ?, 0)", tenantID, code, userID)
	} else {
		_, err = rs.db.ExecContext(ctx, "INSERT INTO referrals (tenant_id, code, user_id, usages) VALUES ($1, $2, $3, 0)", tenantID, code, userID)
	}

	if err != nil {
		return "", err
	}

	rs.generatedTotal.Add(ctx, 1)
	return code, nil
}

func (rs *ReferralSystem) UseCode(ctx context.Context, tenantID, code string) (string, error) {
	tx, err := rs.db.BeginTx(ctx, nil)
	if err != nil {
		return "", err
	}
	defer tx.Rollback()

	var userID string
	if rs.isSQLite() {
		err = tx.QueryRowContext(ctx, "SELECT user_id FROM referrals WHERE tenant_id = ? AND code = ?", tenantID, code).Scan(&userID)
	} else {
		err = tx.QueryRowContext(ctx, "SELECT user_id FROM referrals WHERE tenant_id = $1 AND code = $2", tenantID, code).Scan(&userID)
	}

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return "", errors.New("invalid referral code")
		}
		return "", err
	}

	if rs.isSQLite() {
		_, err = tx.ExecContext(ctx, "UPDATE referrals SET usages = usages + 1 WHERE tenant_id = ? AND code = ?", tenantID, code)
	} else {
		_, err = tx.ExecContext(ctx, "UPDATE referrals SET usages = usages + 1 WHERE tenant_id = $1 AND code = $2", tenantID, code)
	}

	if err != nil {
		return "", err
	}

	if err = tx.Commit(); err != nil {
		return "", err
	}

	rs.usedTotal.Add(ctx, 1)
	return userID, nil
}

func (rs *ReferralSystem) GetViralCoefficient(ctx context.Context, tenantID string) (float64, error) {
	var totalUsages sql.NullInt64
	var totalInvitingUsers sql.NullInt64
	var err error

	if rs.isSQLite() {
		err = rs.db.QueryRowContext(ctx, "SELECT SUM(usages) FROM referrals WHERE tenant_id = ?", tenantID).Scan(&totalUsages)
	} else {
		err = rs.db.QueryRowContext(ctx, "SELECT SUM(usages) FROM referrals WHERE tenant_id = $1", tenantID).Scan(&totalUsages)
	}
	if err != nil {
		return 0, err
	}

	if rs.isSQLite() {
		err = rs.db.QueryRowContext(ctx, "SELECT COUNT(DISTINCT user_id) FROM referrals WHERE tenant_id = ?", tenantID).Scan(&totalInvitingUsers)
	} else {
		err = rs.db.QueryRowContext(ctx, "SELECT COUNT(DISTINCT user_id) FROM referrals WHERE tenant_id = $1", tenantID).Scan(&totalInvitingUsers)
	}
	if err != nil {
		return 0, err
	}

	if !totalUsages.Valid || !totalInvitingUsers.Valid || totalInvitingUsers.Int64 == 0 {
		return 0.0, nil
	}

	return float64(totalUsages.Int64) / float64(totalInvitingUsers.Int64), nil
}

func (rs *ReferralSystem) GetStats(ctx context.Context, tenantID, userID string) (int, error) {
	var totalUsages sql.NullInt64
	var err error
	if rs.isSQLite() {
		err = rs.db.QueryRowContext(ctx, "SELECT SUM(usages) FROM referrals WHERE tenant_id = ? AND user_id = ?", tenantID, userID).Scan(&totalUsages)
	} else {
		err = rs.db.QueryRowContext(ctx, "SELECT SUM(usages) FROM referrals WHERE tenant_id = $1 AND user_id = $2", tenantID, userID).Scan(&totalUsages)
	}

	if err != nil {
		return 0, err
	}
	if !totalUsages.Valid {
		return 0, nil
	}
	return int(totalUsages.Int64), nil
}
