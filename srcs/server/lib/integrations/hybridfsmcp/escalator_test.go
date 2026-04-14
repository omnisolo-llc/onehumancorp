package hybridfsmcp

import (
	"context"
	"strings"
	"testing"
)

func TestEscalator_ShouldEscalate(t *testing.T) {
	e := NewDefaultEscalator()
	ctx := context.Background()

	// Short query should not escalate
	shortQuery := "small local query"
	if e.ShouldEscalate(ctx, shortQuery) {
		t.Errorf("expected short query to NOT escalate")
	}

	// Long query should escalate
	longQuery := strings.Repeat("a", 501)
	if !e.ShouldEscalate(ctx, longQuery) {
		t.Errorf("expected long query to escalate")
	}
}

func TestEscalator_Escalate(t *testing.T) {
	ctx := context.Background()
	e := NewDefaultEscalator()

	// Cloud is reachable
	e.CloudModeReachable = true
	res, err := e.Escalate(ctx, "some query")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res != "escalated_cloud_result" {
		t.Errorf("unexpected result: %s", res)
	}

	// Cloud is unreachable
	e.CloudModeReachable = false
	_, err = e.Escalate(ctx, "some query")
	if err == nil {
		t.Errorf("expected error when cloud is unreachable")
	}
	if err.Error() != "cloud mode unreachable" {
		t.Errorf("unexpected error message: %v", err)
	}
}
