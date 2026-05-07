package agents

import (
	"bufio"
	"context"
	"fmt"
	"io"
	"os/exec"
	"sync"
	"github.com/redis/go-redis/v9"
)

type CmdInterface interface {
	StdoutPipe() (io.ReadCloser, error)
	StderrPipe() (io.ReadCloser, error)
	Start() error
	Wait() error
}

type execCmdWrapper struct {
	*exec.Cmd
}

func (w *execCmdWrapper) StdoutPipe() (io.ReadCloser, error) {
	return w.Cmd.StdoutPipe()
}

func (w *execCmdWrapper) StderrPipe() (io.ReadCloser, error) {
	return w.Cmd.StderrPipe()
}

func (w *execCmdWrapper) Start() error {
	return w.Cmd.Start()
}

func (w *execCmdWrapper) Wait() error {
	return w.Cmd.Wait()
}

// IsolationStrategy defines an isolation abstraction layer supporting worktrees,
// process sandboxes, and specific telemetry tracking per subagent invocation.
type IsolationStrategy interface {
	RunInIsolation(ctx context.Context, worktree string, rdb *redis.Client) error
}

type Provider struct {
	strategy IsolationStrategy
	rdb      *redis.Client
}

func NewProvider(strategy IsolationStrategy, rdb *redis.Client) *Provider {
	return &Provider{
		strategy: strategy,
		rdb:      rdb,
	}
}

func (p *Provider) RunInIsolation(ctx context.Context, worktree string) error {
	return p.strategy.RunInIsolation(ctx, worktree, p.rdb)
}

// BwrapIsolationStrategy is an implementation of IsolationStrategy using Bubblewrap.
type BwrapIsolationStrategy struct{
	CmdFactory func(ctx context.Context, name string, arg ...string) CmdInterface
	Command    string
}

func NewBwrapIsolationStrategy(cmd string) *BwrapIsolationStrategy {
	return &BwrapIsolationStrategy{
		CmdFactory: func(ctx context.Context, name string, arg ...string) CmdInterface {
			return &execCmdWrapper{exec.CommandContext(ctx, name, arg...)}
		},
		Command: cmd,
	}
}

func (s *BwrapIsolationStrategy) RunInIsolation(ctx context.Context, worktree string, rdb *redis.Client) error {
	cmdStr := s.Command
	if cmdStr == "" {
		cmdStr = "bash" // Fallback if empty, though real-world should inject correct agent binary/script
	}

	cmd := s.CmdFactory(ctx, "bwrap",
		"--ro-bind", "/", "/",
		"--bind", worktree, worktree,
		"--unshare-all",
		"--share-net",
		"--die-with-parent",
		"--dir", "/tmp",
		"--setenv", "WORKTREE", worktree,
		"bash", "-c", cmdStr,
	)

	stdoutPipe, err := cmd.StdoutPipe()
	if err != nil {
		return fmt.Errorf("failed to get stdout pipe: %w", err)
	}

	stderrPipe, err := cmd.StderrPipe()
	if err != nil {
		return fmt.Errorf("failed to get stderr pipe: %w", err)
	}

	if err := cmd.Start(); err != nil {
		return fmt.Errorf("failed to start isolated process: %w", err)
	}

	var wg sync.WaitGroup
	if stdoutPipe != nil {
		wg.Add(1)
		go func() {
			defer wg.Done()
			pipeToRedis(ctx, rdb, "mesh:tasks:stdout", stdoutPipe)
		}()
	}
	if stderrPipe != nil {
		wg.Add(1)
		go func() {
			defer wg.Done()
			pipeToRedis(ctx, rdb, "mesh:tasks:stderr", stderrPipe)
		}()
	}

	// Wait for readers to finish reading before Wait closes the pipes
	wg.Wait()

	if err := cmd.Wait(); err != nil {
		return fmt.Errorf("isolated process failed: %w", err)
	}

	return nil
}

func pipeToRedis(ctx context.Context, rdb *redis.Client, channel string, reader io.Reader) {
	if reader == nil {
		return
	}
	scanner := bufio.NewScanner(reader)
	for scanner.Scan() {
		if rdb != nil {
			rdb.Publish(ctx, channel, scanner.Text())
		}
	}
}
