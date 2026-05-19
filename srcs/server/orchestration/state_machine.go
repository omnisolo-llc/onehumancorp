package orchestration

import (
	"context"
	"fmt"
	"time"
	"github.com/google/uuid"

	"github.com/redis/rueidis"
)

var releaseLockScript = rueidis.NewLuaScript(`
	if redis.call("GET", KEYS[1]) == ARGV[1] then
		return redis.call("DEL", KEYS[1])
	else
		return 0
	end
`)

type StateMachine struct {
	client rueidis.Client
}

func NewStateMachine(client rueidis.Client) *StateMachine {
	return &StateMachine{
		client: client,
	}
}

// Transition attempts to acquire a lock and transition a task from fromState to toState.
func (s *StateMachine) Transition(ctx context.Context, taskID, fromState, toState string) error {
	lockKey := fmt.Sprintf("mesh:lock:%s", taskID)
	stateKey := fmt.Sprintf("mesh:state:%s", taskID)

	// Create a unique lock value to ensure we only release our own lock
	lockValue := uuid.New().String()

	// Acquire lock using SET NX
	setCmd := s.client.B().Set().Key(lockKey).Value(lockValue).Nx().Ex(30 * time.Second).Build()
	resp := s.client.Do(ctx, setCmd)
	if err := resp.Error(); err != nil {
		if rueidis.IsRedisNil(err) {
			return fmt.Errorf("could not acquire lock for task %s, currently locked", taskID)
		}
		return fmt.Errorf("error acquiring lock for task %s: %w", taskID, err)
	}

	// Safely release lock using Lua script to prevent releasing a lock we no longer own
	defer func() {
		_ = releaseLockScript.Exec(context.Background(), s.client, []string{lockKey}, []string{lockValue}).Error()
	}()

	// 1. Check if the current state == fromState
	getCmd := s.client.B().Get().Key(stateKey).Build()
	currentState, err := s.client.Do(ctx, getCmd).ToString()
	if err != nil && !rueidis.IsRedisNil(err) {
		return fmt.Errorf("error getting current state for task %s: %w", taskID, err)
	}

	// Treat nil as empty string
	if rueidis.IsRedisNil(err) {
		currentState = ""
	}

	if currentState != fromState {
		return fmt.Errorf("invalid state transition for task %s: current state is %q, expected %q", taskID, currentState, fromState)
	}

	// 2. Update the state to toState
	setCmdState := s.client.B().Set().Key(stateKey).Value(toState).Build()
	if err := s.client.Do(ctx, setCmdState).Error(); err != nil {
		return fmt.Errorf("error updating state for task %s: %w", taskID, err)
	}

	// 3. Broadcast event
	pubCmd := s.client.B().Publish().Channel("mesh:events").Message(fmt.Sprintf("task:%s:%s", taskID, toState)).Build()
	if err := s.client.Do(ctx, pubCmd).Error(); err != nil {
		return fmt.Errorf("error broadcasting event for task %s: %w", taskID, err)
	}

	return nil
}
