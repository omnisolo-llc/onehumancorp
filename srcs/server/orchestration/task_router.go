package orchestration

import (
	"context"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

type TaskAvailableEvent struct {
	TaskID         string   `json:"task_id"`
	RequiredSkills []string `json:"required_skills"`
}

type TaskClaimEvent struct {
	AgentID         string  `json:"agent_id"`
	TaskID          string  `json:"task_id"`
	CapabilityScore float64 `json:"capability_score"`
}

type DynamicTaskRouter struct {
	redis rueidis.Client
	db    db.Provider
}

func NewDynamicTaskRouter(redis rueidis.Client, db db.Provider) *DynamicTaskRouter {
	return &DynamicTaskRouter{
		redis: redis,
		db:    db,
	}
}

func (r *DynamicTaskRouter) BroadcastTaskAvailable(ctx context.Context, taskID string, skills []string) error {
	event := TaskAvailableEvent{
		TaskID:         taskID,
		RequiredSkills: skills,
	}
	data, err := json.Marshal(event)
	if err != nil {
		return err
	}

	cmd := r.redis.B().Publish().Channel("task.available").Message(string(data)).Build()
	return r.redis.Do(ctx, cmd).Error()
}

func (r *DynamicTaskRouter) ListenForClaims(ctx context.Context) error {
	return nil
}

func (r *DynamicTaskRouter) ClaimTask(ctx context.Context, agentID, taskID string, score float64) (bool, error) {
	tx, err := r.db.Begin(ctx)
	if err != nil {
		return false, err
	}
	defer tx.Rollback(ctx)

	var claimedBy *string
	var claimStatus *string

	if r.db.IsSQLite() {
		query := `SELECT claimed_by, claim_status FROM shared_tasks WHERE id = $1`
		err = tx.QueryRow(ctx, query, taskID).Scan(&claimedBy, &claimStatus)
		if err != nil {
			return false, err
		}

		if claimedBy != nil && claimStatus != nil && *claimStatus == "CLAIMED" {
			return false, nil
		}

		updateQuery := `UPDATE shared_tasks SET claimed_by = $1, claim_status = 'CLAIMED', updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err = tx.Exec(ctx, updateQuery, agentID, taskID)
		if err != nil {
			return false, err
		}

	} else {
		query := `SELECT claimed_by, claim_status FROM shared_tasks WHERE id = $1 FOR UPDATE SKIP LOCKED`
		err = tx.QueryRow(ctx, query, taskID).Scan(&claimedBy, &claimStatus)
		if err != nil {
			return false, err
		}

		if claimedBy != nil && claimStatus != nil && *claimStatus == "CLAIMED" {
			return false, nil
		}

		updateQuery := `UPDATE shared_tasks SET claimed_by = $1, claim_status = 'CLAIMED', updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err = tx.Exec(ctx, updateQuery, agentID, taskID)
		if err != nil {
			return false, err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return false, err
	}

	return true, nil
}
