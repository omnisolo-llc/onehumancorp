package harness

type AttemptResult struct {
	Stdout    string
	Stderr    string
	ExitCode  int
	Compacted bool
}

type AgentHarness interface {
	RunAttempt(cmd string) (*AttemptResult, error)
	Compact() error
	Reset() error
}
