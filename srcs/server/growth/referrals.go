package growth

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"errors"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	referralsCreatedTotal metric.Int64Counter
	referralsUsedTotal    metric.Int64Counter
)

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/growth")

	var err error
	referralsCreatedTotal, err = meter.Int64Counter("referrals_created_total", metric.WithDescription("Total number of referral codes created"))
	if err != nil {
		panic(err)
	}

	referralsUsedTotal, err = meter.Int64Counter("referrals_used_total", metric.WithDescription("Total number of times a referral code was used"))
	if err != nil {
		panic(err)
	}
}

type ReferralService struct {
	db db.Provider
}

func NewReferralService(provider db.Provider) *ReferralService {
	return &ReferralService{
		db: provider,
	}
}

func generateID() (string, error) {
	b := make([]byte, 16)
	_, err := rand.Read(b)
	if err != nil {
		return "", err
	}
	return hex.EncodeToString(b), nil
}

func generateCode() (string, error) {
	b := make([]byte, 4)
	_, err := rand.Read(b)
	if err != nil {
		return "", err
	}
	return hex.EncodeToString(b), nil
}

// CreateReferralCode generates a new referral code for the user
func (s *ReferralService) CreateReferralCode(ctx context.Context, userID string) (string, error) {
	id, err := generateID()
	if err != nil {
		return "", err
	}

	code, err := generateCode()
	if err != nil {
		return "", err
	}

	// For simple insert, hybrid compatible
	query := `INSERT INTO referral_links (id, user_id, code, uses_count) VALUES ($1, $2, $3, $4)`
	_, err = s.db.Exec(ctx, query, id, userID, code, 0)
	if err != nil {
		return "", err
	}

	referralsCreatedTotal.Add(ctx, 1)

	return code, nil
}

// RecordReferralUsage increments the usage count of a given code
func (s *ReferralService) RecordReferralUsage(ctx context.Context, code string) error {
	// Hybrid UPSERT logic isn't exactly needed if we just do an UPDATE for existing records,
	// but we must check if code exists. A simple UPDATE works for both SQLite and Postgres.
	query := `UPDATE referral_links SET uses_count = uses_count + 1 WHERE code = $1`

	rowsAffected, err := s.db.Exec(ctx, query, code)
	if err != nil {
		return err
	}

	if rowsAffected == 0 {
		return errors.New("referral code not found")
	}

	referralsUsedTotal.Add(ctx, 1)
	return nil
}
