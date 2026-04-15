package orchestration

import (
	"bytes"
	"compress/gzip"
	"context"
	"database/sql"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// compressUltraPlanData compresses the given byte slice using gzip and encodes it to base64 string.
func compressUltraPlanData(data []byte) (string, error) {
	var b bytes.Buffer
	w := gzip.NewWriter(&b)
	if _, err := w.Write(data); err != nil {
		return "", err
	}
	if err := w.Close(); err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(b.Bytes()), nil
}

// decompressUltraPlanData decodes the base64 string and decompresses the underlying byte slice using gzip.
func decompressUltraPlanData(base64Str string) ([]byte, error) {
	decodedBytes, err := base64.StdEncoding.DecodeString(base64Str)
	if err != nil {
		return nil, err
	}
	r, err := gzip.NewReader(bytes.NewReader(decodedBytes))
	if err != nil {
		return nil, err
	}
	defer r.Close()
	return io.ReadAll(r)
}

// compressStateMachine helper to compress the state machine json payload
func compressStateMachine(stateMachineJSON []byte) ([]byte, error) {
	compressedBase64, err := compressUltraPlanData(stateMachineJSON)
	if err != nil {
		return nil, err
	}
	wrapper := map[string]string{
		"_compressed_base64": compressedBase64,
	}
	return json.Marshal(wrapper)
}

// decompressStateMachine helper to optionally decompress the state machine json payload
func decompressStateMachine(stateMachineJSON []byte) ([]byte, error) {
	var wrapper struct {
		CompressedBase64 string `json:"_compressed_base64"`
	}
	if err := json.Unmarshal(stateMachineJSON, &wrapper); err == nil && wrapper.CompressedBase64 != "" {
		decompressedBytes, err := decompressUltraPlanData(wrapper.CompressedBase64)
		if err != nil {
			return nil, err
		}
		return decompressedBytes, nil
	}
	return stateMachineJSON, nil
}

// UltraPlan represents a deep-deliberation multi-step plan.
type UltraPlan struct {
	ID           string
	MissionID    string
	Status       string
	StateMachine map[string]interface{}
	CreatedAt    time.Time
	UpdatedAt    time.Time
}

// UltraPlanManager handles the state machine and data access for UltraPlans.
type UltraPlanManager struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode // Reusing CentrifugeNode for Mesh integration
	mu          sync.Mutex
}

// NewUltraPlanManager initializes a new UltraPlanManager.
func NewUltraPlanManager(provider db.Provider, redisClient rueidis.Client, hub *CentrifugeNode) *UltraPlanManager {
	return &UltraPlanManager{
		db:          provider,
		redisClient: redisClient,
		hub:         hub,
	}
}

// Critique represents a single peer review comment.
type Critique struct {
	AgentID  string `json:"agent_id"`
	Critique string `json:"critique"`
}

// CreatePlan creates a new UltraPlan with DELIBERATING status.
func (m *UltraPlanManager) CreatePlan(ctx context.Context, missionID string, stateMachine map[string]interface{}) (*UltraPlan, error) {
	stateMachineJSON, err := json.Marshal(stateMachine)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal state_machine: %w", err)
	}

	compressedStateMachineJSON, err := compressStateMachine(stateMachineJSON)
	if err != nil {
		return nil, fmt.Errorf("failed to compress state_machine: %w", err)
	}

	var plan UltraPlan
	var query string
	if m.db.IsSQLite() {
		plan.ID = generateID()
		query = `
			INSERT INTO swarm_ultra_plans (id, mission_id, state_machine, status, created_at, updated_at)
			VALUES ($1, $2, $3, 'DELIBERATING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
			RETURNING id, mission_id, state_machine, status, created_at, updated_at
		`
		err = m.db.QueryRow(ctx, query, plan.ID, missionID, compressedStateMachineJSON).Scan(
			&plan.ID, &plan.MissionID, &compressedStateMachineJSON, &plan.Status, &plan.CreatedAt, &plan.UpdatedAt,
		)
	} else {
		query = `
			INSERT INTO swarm_ultra_plans (mission_id, state_machine, status)
			VALUES ($1, $2, 'DELIBERATING')
			RETURNING id, mission_id, state_machine, status, created_at, updated_at
		`
		err = m.db.QueryRow(ctx, query, missionID, compressedStateMachineJSON).Scan(
			&plan.ID, &plan.MissionID, &compressedStateMachineJSON, &plan.Status, &plan.CreatedAt, &plan.UpdatedAt,
		)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to create ultra plan: %w", err)
	}

	stateMachineJSON, err = decompressStateMachine(compressedStateMachineJSON)
	if err == nil {
		_ = json.Unmarshal(stateMachineJSON, &plan.StateMachine)
	}

	if m.hub != nil {
		// Use Publish function instead of the hallucinated PublishTaskBroadcast
		go func() {
			msg := Message{
				ID:        generateID(),
				FromAgent: "system",
				ToAgent:   "system",
				Type:      "ULTRAPLAN_CREATE",
				Payload:   string(stateMachineJSON),
				Status:    plan.Status,
			}
			_ = m.hub.Publish(msg)
		}()
	}

	return &plan, nil
}

// SubmitCritique adds a critique to the deliberation phase and updates the phase if necessary.
func (m *UltraPlanManager) SubmitCritique(ctx context.Context, planID string, agentID string, critique string) error {
	return m.modifyStateMachine(ctx, planID, func(plan *UltraPlan) (bool, error) {
		critiquesList, ok := plan.StateMachine["critiques"].([]interface{})
		if !ok {
			critiquesList = []interface{}{}
		}

		newCritique := map[string]interface{}{
			"agent_id": agentID,
			"critique": critique,
		}
		plan.StateMachine["critiques"] = append(critiquesList, newCritique)
		plan.StateMachine["phase"] = "REVISION_REQUIRED" // Transition to REVISION_REQUIRED upon critique

		return true, nil
	}, "VOTE_CAST")
}

// ApprovePlan adds an agent's approval to the deliberation phase.
func (m *UltraPlanManager) ApprovePlan(ctx context.Context, planID string, agentID string) error {
	return m.modifyStateMachine(ctx, planID, func(plan *UltraPlan) (bool, error) {
		approvals, ok := plan.StateMachine["approvals"].([]interface{})
		if !ok {
			approvals = []interface{}{}
		}

		// Ensure no double voting
		for _, a := range approvals {
			if aStr, ok := a.(string); ok && aStr == agentID {
				return false, nil // Already approved
			}
		}

		approvals = append(approvals, agentID)
		plan.StateMachine["approvals"] = approvals

		// Check for target votes
		targetVotes := 1 // Default target
		if tv, ok := plan.StateMachine["target_votes"].(float64); ok {
			targetVotes = int(tv)
		}

		if len(approvals) >= targetVotes {
			plan.StateMachine["phase"] = "APPROVED"
		}

		return true, nil
	}, "PHASE_CHANGED")
}

// modifyStateMachine abstracts the boilerplate for fetching, updating, and locking the state machine.
func (m *UltraPlanManager) modifyStateMachine(ctx context.Context, planID string, modifier func(*UltraPlan) (bool, error), eventType string) error {
	if m.redisClient != nil {
		lockKey := "lock:ultraplan:" + planID
		cmd := m.redisClient.B().Set().Key(lockKey).Value("system").Nx().Ex(30 * time.Second).Build()
		err := m.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return errors.New("ultra plan is currently locked")
			}
			return fmt.Errorf("failed to acquire lock: %w", err)
		}
		defer func() {
			delCmd := m.redisClient.B().Del().Key(lockKey).Build()
			_ = m.redisClient.Do(ctx, delCmd).Error()
		}()
	} else if m.db.IsSQLite() {
		m.mu.Lock()
		defer m.mu.Unlock()
	}

	tx, err := m.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var plan UltraPlan
	var stateMachineJSON []byte

	query := `SELECT id, mission_id, state_machine, status, created_at, updated_at FROM swarm_ultra_plans WHERE id = $1`
	if !m.db.IsSQLite() {
		// PostgreSQL native mode: strict transaction concurrency to prevent TOCTOU.
		// "Ensure you use FOR UPDATE SKIP LOCKED in PostgreSQL mode when tallying votes or adding critiques to prevent race conditions."
		query += ` FOR UPDATE SKIP LOCKED`
	}

	err = tx.QueryRow(ctx, query, planID).Scan(
		&plan.ID, &plan.MissionID, &stateMachineJSON, &plan.Status, &plan.CreatedAt, &plan.UpdatedAt,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return errors.New("ultra plan not found or currently locked by another agent")
		}
		return fmt.Errorf("fetch ultra plan: %w", err)
	}

	stateMachineJSON, err = decompressStateMachine(stateMachineJSON)
	if err != nil {
		return fmt.Errorf("decompress state machine: %w", err)
	}

	if err := json.Unmarshal(stateMachineJSON, &plan.StateMachine); err != nil {
		return fmt.Errorf("unmarshal state machine: %w", err)
	}
	if plan.StateMachine == nil {
		plan.StateMachine = make(map[string]interface{})
	}

	oldPhase, _ := plan.StateMachine["phase"].(string)

	changed, err := modifier(&plan)
	if err != nil {
		return err
	}
	if !changed {
		return nil // No changes needed
	}

	newPhase, _ := plan.StateMachine["phase"].(string)
	if oldPhase != "" && newPhase != "" && oldPhase != newPhase {
		duration := time.Since(plan.UpdatedAt).Seconds()
		telemetry.RecordDeliberationPhaseDuration(ctx, planID, oldPhase, duration)
		// Fire-and-forget push of metrics, using a background context with timeout
		go func(phase string) {
			bgCtx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
			defer cancel()
			_ = telemetry.PushMetrics(bgCtx, "kairos_phase_"+phase)
		}(newPhase)
	}

	updatedJSON, err := json.Marshal(plan.StateMachine)
	if err != nil {
		return fmt.Errorf("marshal state machine: %w", err)
	}

	compressedUpdatedJSON, err := compressStateMachine(updatedJSON)
	if err != nil {
		return fmt.Errorf("compress state machine: %w", err)
	}

	updateQuery := `
		UPDATE swarm_ultra_plans
		SET state_machine = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`
	rowsAffected, err := tx.Exec(ctx, updateQuery, compressedUpdatedJSON, planID)
	if err != nil {
		return fmt.Errorf("update status: %w", err)
	}
	if rowsAffected == 0 {
		return fmt.Errorf("update status no rows affected")
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("commit tx: %w", err)
	}

	if m.hub != nil {
		go func() {
			msg := Message{
				ID:        generateID(),
				FromAgent: "system",
				ToAgent:   "system",
				Type:      eventType,
				Payload:   string(updatedJSON),
				Status:    plan.Status,
			}
			_ = m.hub.Publish(msg)
		}()
	}

	return nil
}

// UpdatePlanStatus updates the status and state machine of an UltraPlan, using distributed locks if configured.
func (m *UltraPlanManager) UpdatePlanStatus(ctx context.Context, planID string, newStatus string, stateMachine map[string]interface{}) error {
	if newStatus != "DELIBERATING" && newStatus != "EXECUTING" && newStatus != "COMPLETED" && newStatus != "FAILED" {
		return errors.New("invalid status")
	}

	stateMachineJSON, err := json.Marshal(stateMachine)
	if err != nil {
		return fmt.Errorf("failed to marshal state_machine: %w", err)
	}

	compressedStateMachineJSON, err := compressStateMachine(stateMachineJSON)
	if err != nil {
		return fmt.Errorf("failed to compress state_machine: %w", err)
	}

	if m.redisClient != nil {
		lockKey := "lock:ultraplan:" + planID
		cmd := m.redisClient.B().Set().Key(lockKey).Value("system").Nx().Ex(30 * time.Second).Build()
		err := m.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return errors.New("ultra plan is currently locked")
			}
			return fmt.Errorf("failed to acquire lock: %w", err)
		}
		defer func() {
			delCmd := m.redisClient.B().Del().Key(lockKey).Build()
			_ = m.redisClient.Do(ctx, delCmd).Error()
		}()
	} else if m.db.IsSQLite() {
		m.mu.Lock()
		defer m.mu.Unlock()
	}

	// Begin TX for atomicity
	tx, err := m.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var currentStatus string
	query := `SELECT status FROM swarm_ultra_plans WHERE id = $1`
	if !m.db.IsSQLite() {
		query += ` FOR UPDATE`
	}

	err = tx.QueryRow(ctx, query, planID).Scan(&currentStatus)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return errors.New("ultra plan not found")
		}
		return fmt.Errorf("fetch status: %w", err)
	}

	var oldStateMachineJSON []byte
	var oldPlan UltraPlan
	oldPlanQuery := `SELECT state_machine, updated_at FROM swarm_ultra_plans WHERE id = $1`
	err = tx.QueryRow(ctx, oldPlanQuery, planID).Scan(&oldStateMachineJSON, &oldPlan.UpdatedAt)
	if err == nil {
		decompressed, decompErr := decompressStateMachine(oldStateMachineJSON)
		if decompErr == nil {
			var oldSM map[string]interface{}
			if json.Unmarshal(decompressed, &oldSM) == nil {
				oldPhase, _ := oldSM["phase"].(string)
				newPhase, _ := stateMachine["phase"].(string)
				if oldPhase != "" && (oldPhase != newPhase || currentStatus != newStatus) {
					duration := time.Since(oldPlan.UpdatedAt).Seconds()
					telemetry.RecordDeliberationPhaseDuration(ctx, planID, oldPhase, duration)
					// Fire-and-forget push of metrics, using a background context with timeout
					go func(phase string) {
						bgCtx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
						defer cancel()
						_ = telemetry.PushMetrics(bgCtx, "kairos_phase_"+phase)
					}(newPhase)
				}
			}
		}
	}

	updateQuery := `
		UPDATE swarm_ultra_plans
		SET status = $1, state_machine = $2, updated_at = CURRENT_TIMESTAMP
		WHERE id = $3
	`
	rowsAffected, err := tx.Exec(ctx, updateQuery, newStatus, compressedStateMachineJSON, planID)
	if err != nil {
		return fmt.Errorf("update status: %w", err)
	}

	if rowsAffected == 0 {
		return fmt.Errorf("update status no rows affected")
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("commit tx: %w", err)
	}

	if m.hub != nil {
		// Use Publish function or custom logic instead of the hallucinated PublishTaskBroadcast
		go func() {
			msg := Message{
				ID:        generateID(),
				FromAgent: "system",
				ToAgent:   "system",
				Type:      "ULTRAPLAN_UPDATE",
				Payload:   string(stateMachineJSON),
				Status:    newStatus,
			}
			_ = m.hub.Publish(msg)
		}()
	}

	return nil
}

// GetUltraPlan fetches an UltraPlan by ID.
func (m *UltraPlanManager) GetUltraPlan(ctx context.Context, planID string) (*UltraPlan, error) {
	var plan UltraPlan
	var stateMachineJSON []byte

	query := `SELECT id, mission_id, state_machine, status, created_at, updated_at FROM swarm_ultra_plans WHERE id = $1`
	err := m.db.QueryRow(ctx, query, planID).Scan(
		&plan.ID, &plan.MissionID, &stateMachineJSON, &plan.Status, &plan.CreatedAt, &plan.UpdatedAt,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, errors.New("ultra plan not found")
		}
		return nil, fmt.Errorf("failed to fetch ultra plan: %w", err)
	}

	stateMachineJSON, err = decompressStateMachine(stateMachineJSON)
	if err == nil {
		_ = json.Unmarshal(stateMachineJSON, &plan.StateMachine)
	}
	return &plan, nil
}
