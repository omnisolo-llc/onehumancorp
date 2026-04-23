package interop

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/proto/interop"
)

func TestReliableMesh(t *testing.T) {
	baseMesh := NewTeammateMeshWithClient(nil)
	rm, err := NewReliableMesh(baseMesh, "test.acks")
	if err != nil {
		t.Fatalf("failed to create reliable mesh: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	channel := "test.reliable"

	// Subscribe
	handled := make(chan bool, 1)
	err = rm.SubscribeReliable(ctx, channel, func(job *interoppb.JobDispatch) error {
		if job.JobId == "job-1" {
			handled <- true
			return nil
		}
		return nil
	})
	if err != nil {
		t.Fatalf("subscribe failed: %v", err)
	}

	// Wait a tiny bit for subscriber to be ready
	time.Sleep(50 * time.Millisecond)

	// Publish
	err = rm.PublishReliable(ctx, channel, &interoppb.JobDispatch{
		JobId: "job-1",
	})
	if err != nil {
		t.Fatalf("publish failed: %v", err)
	}

	select {
	case <-handled:
		// Success
	case <-ctx.Done():
		t.Fatal("timeout waiting for job handling")
	}
}

func TestReliableMesh_RetryExhausted(t *testing.T) {
	baseMesh := NewTeammateMeshWithClient(nil)
	rm, err := NewReliableMesh(baseMesh, "test.acks2")
	if err != nil {
		t.Fatalf("failed to create reliable mesh: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	channel := "test.reliable2"

	// Publish without subscriber - should fail after retries
	err = rm.PublishReliable(ctx, channel, &interoppb.JobDispatch{
		JobId: "job-fail",
		MaxRetries: 1, // quick fail
	})

	if err == nil {
		t.Fatal("expected publish to fail due to timeout/no ack")
	}
}
