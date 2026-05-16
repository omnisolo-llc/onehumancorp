package restic

import (
	"context"
	"fmt"
	"os"
	"os/exec"
)

type Provider struct{}

func NewProvider() *Provider {
	return &Provider{}
}

func (p *Provider) Name() string {
	return "restic"
}

func (p *Provider) Status() string {
	if os.Getenv("OHC_EXECUTION_MODE") == "cloud" {
		return "unsupported"
	}
	return "active"
}

func (p *Provider) ResticSnapshot(ctx context.Context, repo string, password string, paths []string) (string, error) {
	if p.Status() == "unsupported" {
		return "", fmt.Errorf("restic snapshot unsupported in cloud mode")
	}

	args := []string{"backup", "--repo", repo}
	args = append(args, paths...)

	cmd := exec.CommandContext(ctx, "restic", args...)
	cmd.Env = append(os.Environ(), "RESTIC_PASSWORD="+password)

	out, err := cmd.CombinedOutput()
	if err != nil {
		return string(out), fmt.Errorf("restic snapshot failed: %w", err)
	}

	return string(out), nil
}

func (p *Provider) ResticRestore(ctx context.Context, repo string, password string, snapshotID string, targetDir string) (string, error) {
	if p.Status() == "unsupported" {
		return "", fmt.Errorf("restic restore unsupported in cloud mode")
	}

	cmd := exec.CommandContext(ctx, "restic", "restore", snapshotID, "--target", targetDir, "--repo", repo)
	cmd.Env = append(os.Environ(), "RESTIC_PASSWORD="+password)

	out, err := cmd.CombinedOutput()
	if err != nil {
		return string(out), fmt.Errorf("restic restore failed: %w", err)
	}

	return string(out), nil
}

func (p *Provider) ResticStatus(ctx context.Context, repo string, password string) (string, error) {
	if p.Status() == "unsupported" {
		return "", fmt.Errorf("restic status unsupported in cloud mode")
	}

	cmd := exec.CommandContext(ctx, "restic", "snapshots", "--repo", repo)
	cmd.Env = append(os.Environ(), "RESTIC_PASSWORD="+password)

	out, err := cmd.CombinedOutput()
	if err != nil {
		return string(out), fmt.Errorf("restic status failed: %w", err)
	}

	return string(out), nil
}
