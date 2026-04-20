package sandbox

import "context"

type RunOptions struct {
	Command []string
	AllowedDomains []string
	ReadOnlyDirs []string
	ReadWriteDirs []string
	Env map[string]string
}

type SandboxHarness interface {
	Run(ctx context.Context, opts RunOptions) (string, error)
}
