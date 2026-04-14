package growth

import (
	"context"
	"crypto/rand"
	"database/sql"
	"encoding/hex"
	"fmt"
	"sync"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter         = otel.Meter("services/growth")
	rewardsIssued metric.Int64Counter
)

func init() {
	var err error
	rewardsIssued, err = meter.Int64Counter("viral_loop_rewards_issued_total", metric.WithDescription("Total number of viral loop rewards issued"))
	if err != nil {
		fmt.Printf("failed to initialize metrics: %v\n", err)
	}
}

type ViralLoopReward struct {
	ID         string
	UserID     string
	RewardType string
	Amount     int
	IssuedAt   time.Time
}

// generateID creates a unique ID for a reward.
func generateID() (string, error) {
	b := make([]byte, 16)
	_, err := rand.Read(b)
	if err != nil {
		return "", err
	}
	return "reward-" + hex.EncodeToString(b), nil
}

// DBProvider defines the interface for database operations, compatible with *sql.DB and *sql.Tx.
type DBProvider interface {
	ExecContext(ctx context.Context, query string, args ...interface{}) (sql.Result, error)
	QueryRowContext(ctx context.Context, query string, args ...interface{}) *sql.Row
	QueryContext(ctx context.Context, query string, args ...interface{}) (*sql.Rows, error)
}

// ViralLoopManager manages viral loop rewards using a durable database provider.
type ViralLoopManager struct {
	db DBProvider
}

// NewViralLoopManager creates a new ViralLoopManager.
func NewViralLoopManager(db DBProvider) *ViralLoopManager {
	return &ViralLoopManager{
		db: db,
	}
}

// IssueReward issues a new reward and persists it to the database.
func (vlm *ViralLoopManager) IssueReward(ctx context.Context, userID, rewardType string, amount int) (ViralLoopReward, error) {
	id, err := generateID()
	if err != nil {
		return ViralLoopReward{}, fmt.Errorf("failed to generate reward ID: %w", err)
	}

	reward := ViralLoopReward{
		ID:         id,
		UserID:     userID,
		RewardType: rewardType,
		Amount:     amount,
		IssuedAt:   time.Now().UTC(),
	}

	// Persist to database
	query := `INSERT INTO viral_loop_rewards (id, user_id, reward_type, amount, issued_at) VALUES ($1, $2, $3, $4, $5)`
	_, err = vlm.db.ExecContext(ctx, query, reward.ID, reward.UserID, reward.RewardType, reward.Amount, reward.IssuedAt)
	if err != nil {
		return ViralLoopReward{}, fmt.Errorf("failed to issue reward: %w", err)
	}

	if rewardsIssued != nil {
		rewardsIssued.Add(ctx, 1)
	}

	return reward, nil
}

// GetRewards retrieves rewards for a user from the database.
func (vlm *ViralLoopManager) GetRewards(ctx context.Context, userID string) ([]ViralLoopReward, error) {
	query := `SELECT id, user_id, reward_type, amount, issued_at FROM viral_loop_rewards WHERE user_id = $1 ORDER BY issued_at DESC`
	rows, err := vlm.db.QueryContext(ctx, query, userID)
	if err != nil {
		return nil, fmt.Errorf("failed to query rewards: %w", err)
	}
	defer rows.Close()

	var rewards []ViralLoopReward
	for rows.Next() {
		var r ViralLoopReward
		if err := rows.Scan(&r.ID, &r.UserID, &r.RewardType, &r.Amount, &r.IssuedAt); err != nil {
			return nil, fmt.Errorf("failed to scan reward: %w", err)
		}
		rewards = append(rewards, r)
	}
	if err = rows.Err(); err != nil {
		return nil, fmt.Errorf("row iteration error: %w", err)
	}

	return rewards, nil
}

// InMemViralLoopManager manages viral loop rewards using an in-memory map.
type InMemViralLoopManager struct {
	mu      sync.RWMutex
	rewards map[string][]ViralLoopReward
}

func NewInMemViralLoopManager() *InMemViralLoopManager {
	return &InMemViralLoopManager{
		rewards: make(map[string][]ViralLoopReward),
	}
}

func (vlm *InMemViralLoopManager) IssueReward(ctx context.Context, userID, rewardType string, amount int) (ViralLoopReward, error) {
	id, err := generateID()
	if err != nil {
		return ViralLoopReward{}, fmt.Errorf("failed to generate reward ID: %w", err)
	}

	reward := ViralLoopReward{
		ID:         id,
		UserID:     userID,
		RewardType: rewardType,
		Amount:     amount,
		IssuedAt:   time.Now().UTC(),
	}

	vlm.mu.Lock()
	vlm.rewards[userID] = append(vlm.rewards[userID], reward)
	vlm.mu.Unlock()

	if rewardsIssued != nil {
		rewardsIssued.Add(ctx, 1)
	}

	return reward, nil
}

func (vlm *InMemViralLoopManager) GetRewards(ctx context.Context, userID string) ([]ViralLoopReward, error) {
	vlm.mu.RLock()
	defer vlm.mu.RUnlock()

	userRewards, ok := vlm.rewards[userID]
	if !ok {
		return []ViralLoopReward{}, nil
	}

	// Create a copy to reverse (simulate ORDER BY issued_at DESC)
	var reversed []ViralLoopReward
	for i := len(userRewards) - 1; i >= 0; i-- {
		reversed = append(reversed, userRewards[i])
	}
	return reversed, nil
}
