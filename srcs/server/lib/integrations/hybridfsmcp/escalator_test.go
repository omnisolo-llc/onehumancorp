package hybridfsmcp

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestEscalator(t *testing.T) {
	tempCloud := t.TempDir()
	tempLocal := t.TempDir()
	cloudProv := NewCloudFSProvider(tempCloud)
	localProv := NewLocalFSProvider(tempLocal)

	claims := &auth.Claims{OrganizationID: "org-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_ = cloudProv.WriteFile(ctx, "complex.txt", []byte("cloud result"))
	_ = localProv.WriteFile(ctx, "simple.txt", []byte("local result"))

	escalator := NewHybridEscalator(cloudProv, localProv)

	// Test local
	res, err := escalator.AnalyzeAndExecute(ctx, "simple.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res != "local result" {
		t.Errorf("expected 'local result', got '%v'", res)
	}

	// Test escalate fallback (fallback since simple.txt isn't in cloud, but has 'escalate' keyword)
	// We'll write to a file name that is over 50 chars to trigger it
	longName := "simple.txt-this-is-a-very-long-name-to-trigger-the-escalation-heuristic-fallback.txt"
	_ = localProv.WriteFile(ctx, longName, []byte("local result fallback"))
	res, err = escalator.AnalyzeAndExecute(ctx, longName)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res != "local result fallback" {
		t.Errorf("expected 'local result fallback', got '%v'", res)
	}

	// Test escalate cloud success
	longCloudName := "complex.txt-this-is-a-very-long-name-to-trigger-the-escalation-heuristic.txt"
	_ = cloudProv.WriteFile(ctx, longCloudName, []byte("cloud result"))
	res, err = escalator.AnalyzeAndExecute(ctx, longCloudName)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res != "cloud result" {
		t.Errorf("expected 'cloud result', got '%v'", res)
	}
}
