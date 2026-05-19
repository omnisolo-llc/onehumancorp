package orchestration

import (
	"context"
	"errors"
	"testing"

	"github.com/redis/rueidis"
	"github.com/redis/rueidis/mock"
	"go.uber.org/mock/gomock"
)

// We define a custom matcher to handle the dynamic UUID inside the SET NX command.
type setCmdMatcher struct{}

func (m setCmdMatcher) Matches(x interface{}) bool {
	cmds, ok := x.(rueidis.Completed)
	if !ok {
		return false
	}
	cmd := cmds.Commands()
	if len(cmd) < 5 {
		return false
	}
	// SET mesh:lock:task1 <uuid> NX EX 30
	return cmd[0] == "SET" && cmd[1] == "mesh:lock:task1" && cmd[3] == "NX" && cmd[4] == "EX" && cmd[5] == "30"
}

func (m setCmdMatcher) String() string {
	return "SET mesh:lock:task1 <uuid> NX EX 30"
}

func TestTransitionSuccess(t *testing.T) {
	ctrl := gomock.NewController(t)
	defer ctrl.Finish()

	client := mock.NewClient(ctrl)

	// Mock SET NX success
	client.EXPECT().
		Do(gomock.Any(), setCmdMatcher{}).
		Return(mock.Result(mock.RedisString("OK")))

	// Mock GET current state
	client.EXPECT().
		Do(gomock.Any(), mock.Match("GET", "mesh:state:task1")).
		Return(mock.Result(mock.RedisString("PENDING")))

	// Mock SET new state
	client.EXPECT().
		Do(gomock.Any(), mock.Match("SET", "mesh:state:task1", "EXECUTING")).
		Return(mock.Result(mock.RedisString("OK")))

	// Mock PUBLISH
	client.EXPECT().
		Do(gomock.Any(), mock.Match("PUBLISH", "mesh:events", "task:task1:EXECUTING")).
		Return(mock.Result(mock.RedisInt64(1)))

	// Mock EVAL (Lua script execution fallback/execution)
	client.EXPECT().
		Do(gomock.Any(), gomock.Any()).
		Return(mock.Result(mock.RedisInt64(1))).AnyTimes()

	sm := NewStateMachine(client)
	err := sm.Transition(context.Background(), "task1", "PENDING", "EXECUTING")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

func TestTransitionSuccessNilFromState(t *testing.T) {
	ctrl := gomock.NewController(t)
	defer ctrl.Finish()

	client := mock.NewClient(ctrl)

	// Mock SET NX success
	client.EXPECT().
		Do(gomock.Any(), setCmdMatcher{}).
		Return(mock.Result(mock.RedisString("OK")))

	// Mock GET current state (nil)
	client.EXPECT().
		Do(gomock.Any(), mock.Match("GET", "mesh:state:task1")).
		Return(mock.ErrorResult(rueidis.Nil))

	// Mock SET new state
	client.EXPECT().
		Do(gomock.Any(), mock.Match("SET", "mesh:state:task1", "EXECUTING")).
		Return(mock.Result(mock.RedisString("OK")))

	// Mock PUBLISH
	client.EXPECT().
		Do(gomock.Any(), mock.Match("PUBLISH", "mesh:events", "task:task1:EXECUTING")).
		Return(mock.Result(mock.RedisInt64(1)))

	// Mock EVAL success
	client.EXPECT().
		Do(gomock.Any(), gomock.Any()).
		Return(mock.Result(mock.RedisInt64(1))).AnyTimes()

	sm := NewStateMachine(client)
	err := sm.Transition(context.Background(), "task1", "", "EXECUTING")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

func TestTransitionLockFailed(t *testing.T) {
	ctrl := gomock.NewController(t)
	defer ctrl.Finish()

	client := mock.NewClient(ctrl)

	// Mock SET NX failure (Redis returns nil when NX fails)
	client.EXPECT().
		Do(gomock.Any(), setCmdMatcher{}).
		Return(mock.ErrorResult(rueidis.Nil))

	sm := NewStateMachine(client)
	err := sm.Transition(context.Background(), "task1", "PENDING", "EXECUTING")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	expectedMsg := "could not acquire lock for task task1, currently locked"
	if err.Error() != expectedMsg {
		t.Fatalf("expected %q, got %q", expectedMsg, err.Error())
	}
}

func TestTransitionSetError(t *testing.T) {
	ctrl := gomock.NewController(t)
	defer ctrl.Finish()

	client := mock.NewClient(ctrl)

	// Mock SET error
	client.EXPECT().
		Do(gomock.Any(), setCmdMatcher{}).
		Return(mock.ErrorResult(errors.New("redis error")))

	sm := NewStateMachine(client)
	err := sm.Transition(context.Background(), "task1", "PENDING", "EXECUTING")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestTransitionGetStateError(t *testing.T) {
	ctrl := gomock.NewController(t)
	defer ctrl.Finish()

	client := mock.NewClient(ctrl)

	client.EXPECT().
		Do(gomock.Any(), setCmdMatcher{}).
		Return(mock.Result(mock.RedisString("OK")))

	client.EXPECT().
		Do(gomock.Any(), mock.Match("GET", "mesh:state:task1")).
		Return(mock.ErrorResult(errors.New("get error")))

	client.EXPECT().
		Do(gomock.Any(), gomock.Any()).
		Return(mock.Result(mock.RedisInt64(1))).AnyTimes()

	sm := NewStateMachine(client)
	err := sm.Transition(context.Background(), "task1", "PENDING", "EXECUTING")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestTransitionStateMismatch(t *testing.T) {
	ctrl := gomock.NewController(t)
	defer ctrl.Finish()

	client := mock.NewClient(ctrl)

	client.EXPECT().
		Do(gomock.Any(), setCmdMatcher{}).
		Return(mock.Result(mock.RedisString("OK")))

	client.EXPECT().
		Do(gomock.Any(), mock.Match("GET", "mesh:state:task1")).
		Return(mock.Result(mock.RedisString("EXECUTING")))

	client.EXPECT().
		Do(gomock.Any(), gomock.Any()).
		Return(mock.Result(mock.RedisInt64(1))).AnyTimes()

	sm := NewStateMachine(client)
	err := sm.Transition(context.Background(), "task1", "PENDING", "EXECUTING")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestTransitionSetStateError(t *testing.T) {
	ctrl := gomock.NewController(t)
	defer ctrl.Finish()

	client := mock.NewClient(ctrl)

	client.EXPECT().
		Do(gomock.Any(), setCmdMatcher{}).
		Return(mock.Result(mock.RedisString("OK")))

	client.EXPECT().
		Do(gomock.Any(), mock.Match("GET", "mesh:state:task1")).
		Return(mock.Result(mock.RedisString("PENDING")))

	client.EXPECT().
		Do(gomock.Any(), mock.Match("SET", "mesh:state:task1", "EXECUTING")).
		Return(mock.ErrorResult(errors.New("set error")))

	client.EXPECT().
		Do(gomock.Any(), gomock.Any()).
		Return(mock.Result(mock.RedisInt64(1))).AnyTimes()

	sm := NewStateMachine(client)
	err := sm.Transition(context.Background(), "task1", "PENDING", "EXECUTING")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestTransitionPublishError(t *testing.T) {
	ctrl := gomock.NewController(t)
	defer ctrl.Finish()

	client := mock.NewClient(ctrl)

	client.EXPECT().
		Do(gomock.Any(), setCmdMatcher{}).
		Return(mock.Result(mock.RedisString("OK")))

	client.EXPECT().
		Do(gomock.Any(), mock.Match("GET", "mesh:state:task1")).
		Return(mock.Result(mock.RedisString("PENDING")))

	client.EXPECT().
		Do(gomock.Any(), mock.Match("SET", "mesh:state:task1", "EXECUTING")).
		Return(mock.Result(mock.RedisString("OK")))

	client.EXPECT().
		Do(gomock.Any(), mock.Match("PUBLISH", "mesh:events", "task:task1:EXECUTING")).
		Return(mock.ErrorResult(errors.New("publish error")))

	client.EXPECT().
		Do(gomock.Any(), gomock.Any()).
		Return(mock.Result(mock.RedisInt64(1))).AnyTimes()

	sm := NewStateMachine(client)
	err := sm.Transition(context.Background(), "task1", "PENDING", "EXECUTING")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}
