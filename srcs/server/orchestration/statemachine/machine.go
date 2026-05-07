package statemachine

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/redis/go-redis/v9"
	"onehumancorp/srcs/server/db"
)

const (
	StatePending           = "PENDING"
	StateAssigned          = "ASSIGNED"
	StateExecuting         = "EXECUTING"
	StateWaitingDelegation = "WAITING_DELEGATION"
	StateReview            = "REVIEW"
	StateSuccess           = "SUCCESS" // DONE
	StateTerminatedError   = "TERMINATED_ERROR" // FAILED
	StateDone              = "DONE"
	StateFailed            = "FAILED"
	StateCloudProcessing   = "CLOUD_PROCESSING"
)

var allowedTransitions = map[string][]string{
	StatePending:           {StateAssigned},
	StateAssigned:          {StateExecuting, StatePending}, // can go back to pending if agent dies
	StateExecuting:         {StateWaitingDelegation, StateReview, StateDone, StateFailed},
	StateWaitingDelegation: {StateExecuting},
	StateReview:            {StateDone, StateFailed, StateExecuting},
}

type MeshHub interface {
	Publish(ctx context.Context, channel string, data []byte) error
	Subscribe(ctx context.Context, channel string, handler func(data []byte)) error
}

type StateMachine struct {
	db     *sql.DB
	redis  *redis.Client
	mesh   MeshHub
}

func NewStateMachine(database *sql.DB, rdb *redis.Client, mesh MeshHub) *StateMachine {
	return &StateMachine{
		db:    database,
		redis: rdb,
		mesh:  mesh,
	}
}

func isValidTransition(from, to string) bool {
	// If it's the same state, allow it (idempotency or no-op)
	if from == to {
		return true
	}
	allowed, ok := allowedTransitions[from]
	if ok {
		for _, a := range allowed {
			if a == to {
				return true
			}
		}
	}
	// For legacy support or tests, map general valid terminal transitions and cloud processing
	if (to == StateDone || to == StateFailed || to == StateCloudProcessing) && (from != StateDone && from != StateFailed) {
		return true
	}
	return false
}

func (sm *StateMachine) Transition(ctx context.Context, entityID string, toState string, agentID string) error {
	isSQLite := db.GlobalProvider.IsSQLite()

	// 1. Acquire Distributed Lock
	if sm.redis != nil && !isSQLite {
		lockKey := fmt.Sprintf("lock:statemachine:%s", entityID)
		lockToken := uuid.New().String()
		locked, err := sm.redis.SetNX(ctx, lockKey, lockToken, 10*time.Second).Result()
		if err != nil {
			return fmt.Errorf("failed to acquire redis lock: %w", err)
		}
		if !locked {
			return fmt.Errorf("could not acquire lock for entity %s", entityID)
		}
		defer func() {
			script := `
                if redis.call("get", KEYS[1]) == ARGV[1] then
                    return redis.call("del", KEYS[1])
                else
                    return 0
                end
            `
			_ = sm.redis.Eval(context.Background(), script, []string{lockKey}, lockToken).Err()
		}()
	}

	// 2. DB Transaction
	tx, err := sm.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	err = sm.TransitionTx(ctx, tx, entityID, toState, agentID)
	if err != nil {
		return err
	}

	return tx.Commit()
}

func (sm *StateMachine) TransitionTx(ctx context.Context, tx *sql.Tx, entityID string, toState string, agentID string) error {
	isSQLite := db.GlobalProvider.IsSQLite()

	// 3. Read Current State
	var currentState string
	query := "SELECT status FROM shared_tasks WHERE id = $1"
	if !isSQLite {
		query += " FOR UPDATE"
		err := tx.QueryRowContext(ctx, query, entityID).Scan(&currentState)
		if err != nil {
			if errors.Is(err, sql.ErrNoRows) {
				return fmt.Errorf("entity not found: %s", entityID)
			}
			return err
		}
	} else {
		// Sqlite needs to substitute $1 to ?
		query = "SELECT status FROM shared_tasks WHERE id = ?"
		err := tx.QueryRowContext(ctx, query, entityID).Scan(&currentState)
		if err != nil {
			if errors.Is(err, sql.ErrNoRows) {
				return fmt.Errorf("entity not found: %s", entityID)
			}
			return err
		}
	}

	// 4. Validate Transition
	if !isValidTransition(currentState, toState) {
		return fmt.Errorf("invalid transition from %s to %s", currentState, toState)
	}

	// If no state change, just return
	if currentState == toState {
		return nil
	}

	// 5. Update Entity
	if isSQLite {
		if agentID != "" {
			updateQuery := "UPDATE shared_tasks SET status = ?, agent_id = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
			_, err := tx.ExecContext(ctx, updateQuery, toState, agentID, entityID)
			if err != nil {
				return fmt.Errorf("failed to update entity state: %w", err)
			}
		} else {
			updateQuery := "UPDATE shared_tasks SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
			_, err := tx.ExecContext(ctx, updateQuery, toState, entityID)
			if err != nil {
				return fmt.Errorf("failed to update entity state: %w", err)
			}
		}
	} else {
		if agentID != "" {
			updateQuery := "UPDATE shared_tasks SET status = $1, agent_id = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3"
			_, err := tx.ExecContext(ctx, updateQuery, toState, agentID, entityID)
			if err != nil {
				return fmt.Errorf("failed to update entity state: %w", err)
			}
		} else {
			updateQuery := "UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2"
			_, err := tx.ExecContext(ctx, updateQuery, toState, entityID)
			if err != nil {
				return fmt.Errorf("failed to update entity state: %w", err)
			}
		}
	}

	// 6. Record Audit Log
	transitionID := uuid.New().String()
	if isSQLite {
		auditQuery := `
		    INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, occurred_at)
		    VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
	    `
		_, err := tx.ExecContext(ctx, auditQuery, transitionID, entityID, "SHARED_TASK", currentState, toState, agentID)
		if err != nil {
			return fmt.Errorf("failed to insert audit log: %w", err)
		}
	} else {
		auditQuery := `
		    INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, occurred_at)
		    VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP)
	    `
		_, err := tx.ExecContext(ctx, auditQuery, transitionID, entityID, "SHARED_TASK", currentState, toState, agentID)
		if err != nil {
			return fmt.Errorf("failed to insert audit log: %w", err)
		}
	}

	// 7. Emit Teammate Mesh Broadcast
	if sm.mesh != nil {
		event := map[string]string{
			"entity_id":   entityID,
			"entity_type": "SHARED_TASK",
			"from_state":  currentState,
			"to_state":    toState,
			"agent_id":    agentID,
		}
		data, _ := json.Marshal(event)
		_ = sm.mesh.Publish(ctx, "state_transitions", data)
	}

	return nil
}
