package agents

import (
	"context"
	"testing"
	"strings"
	"os/exec"
    "os"
    "fmt"
    "io"
	"github.com/redis/go-redis/v9"
)

type MockIsolationStrategy struct {
	called bool
}

func (m *MockIsolationStrategy) RunInIsolation(ctx context.Context, worktree string, rdb *redis.Client) error {
	m.called = true
	return nil
}

func TestProvider_RunInIsolation(t *testing.T) {
	mockStrategy := &MockIsolationStrategy{}
	provider := NewProvider(mockStrategy, nil)

	err := provider.RunInIsolation(context.Background(), "/tmp/worktree")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	if !mockStrategy.called {
		t.Errorf("expected RunInIsolation to be called on strategy")
	}
}

func TestPipeToRedis(t *testing.T) {
    reader := strings.NewReader("hello\nworld")
    pipeToRedis(context.Background(), nil, "test", reader)
    pipeToRedis(context.Background(), nil, "test", nil)
}

type errReader struct{}
func (e *errReader) Read(p []byte) (n int, err error) {
    return 1, os.ErrPermission
}
func TestPipeToRedis_Error(t *testing.T) {
    reader := &errReader{}
    pipeToRedis(context.Background(), nil, "test", reader)
}

type eofReader struct{}
func (e *eofReader) Read(p []byte) (n int, err error) {
    return 0, io.EOF
}
func TestPipeToRedis_EOF(t *testing.T) {
    reader := &eofReader{}
    pipeToRedis(context.Background(), nil, "test", reader)
}

func TestPipeToRedis_RedisNotNull(t *testing.T) {
    reader := strings.NewReader("hello\nworld")
    rdb := redis.NewClient(&redis.Options{})
    pipeToRedis(context.Background(), rdb, "test", reader)
}

func TestBwrapIsolationStrategy_RunInIsolation_FailStart(t *testing.T) {
    strategy := NewBwrapIsolationStrategy("")
	strategy.CmdFactory = func(ctx context.Context, name string, arg ...string) CmdInterface {
		return &mockCmd{failStart: true}
	}
    err := strategy.RunInIsolation(context.Background(), "/tmp", nil)
    if err == nil {
        t.Errorf("expected error running bwrap without real env, got nil")
    }
}

func TestBwrapIsolationStrategy_RunInIsolation_WaitError(t *testing.T) {
    strategy := NewBwrapIsolationStrategy("echo 'test'")
	strategy.CmdFactory = func(ctx context.Context, name string, arg ...string) CmdInterface {
		return &mockCmd{failWait: true}
	}
    err := strategy.RunInIsolation(context.Background(), "/tmp", nil)
    if err == nil {
        t.Errorf("expected error due to failing process")
    }
}

func TestBwrapIsolationStrategy_RunInIsolation_Success(t *testing.T) {
    strategy := NewBwrapIsolationStrategy("echo 'test'")
	strategy.CmdFactory = func(ctx context.Context, name string, arg ...string) CmdInterface {
		return &mockCmd{}
	}
    err := strategy.RunInIsolation(context.Background(), "/tmp", nil)
    if err != nil {
        t.Errorf("expected no error, got %v", err)
    }
}

func TestBwrapIsolationStrategy_Factory(t *testing.T) {
    strategy := NewBwrapIsolationStrategy("echo 'hello'")
    cmd := strategy.CmdFactory(context.Background(), "echo", "hello")
    if cmd == nil {
        t.Errorf("expected cmd")
    }
}

type mockCmd struct {
	failStdout bool
	failStderr bool
    failStart  bool
    failWait   bool
}

func (m *mockCmd) StdoutPipe() (io.ReadCloser, error) {
	if m.failStdout {
		return nil, fmt.Errorf("stdout err")
	}
	return io.NopCloser(strings.NewReader("out\n")), nil
}

func (m *mockCmd) StderrPipe() (io.ReadCloser, error) {
	if m.failStderr {
		return nil, fmt.Errorf("stderr err")
	}
	return io.NopCloser(strings.NewReader("err\n")), nil
}

func (m *mockCmd) Start() error {
    if m.failStart {
        return fmt.Errorf("start err")
    }
	return nil
}

func (m *mockCmd) Wait() error {
    if m.failWait {
        return fmt.Errorf("wait err")
    }
	return nil
}

func TestBwrapIsolationStrategy_PipeErrors(t *testing.T) {
	strategy := NewBwrapIsolationStrategy("echo 'test'")
	strategy.CmdFactory = func(ctx context.Context, name string, arg ...string) CmdInterface {
		return &mockCmd{failStdout: true}
	}
	err := strategy.RunInIsolation(context.Background(), "/tmp", nil)
	if err == nil || !strings.Contains(err.Error(), "stdout err") {
		t.Errorf("expected stdout pipe error, got: %v", err)
	}

	strategy.CmdFactory = func(ctx context.Context, name string, arg ...string) CmdInterface {
		return &mockCmd{failStderr: true}
	}
	err = strategy.RunInIsolation(context.Background(), "/tmp", nil)
	if err == nil || !strings.Contains(err.Error(), "stderr err") {
		t.Errorf("expected stderr pipe error, got: %v", err)
	}
}

func TestExecCmdWrapper_All(t *testing.T) {
    cmd := exec.Command("echo", "hello")
    w := &execCmdWrapper{cmd}

    out, err := w.StdoutPipe()
    if err != nil {
        t.Fatalf("StdoutPipe err: %v", err)
    }
    if out == nil {
        t.Fatalf("StdoutPipe returned nil")
    }

    errOut, err := w.StderrPipe()
    if err != nil {
        t.Fatalf("StderrPipe err: %v", err)
    }
    if errOut == nil {
        t.Fatalf("StderrPipe returned nil")
    }

    if err := w.Start(); err != nil {
        t.Fatalf("Start err: %v", err)
    }

    if err := w.Wait(); err != nil {
        t.Fatalf("Wait err: %v", err)
    }
}
