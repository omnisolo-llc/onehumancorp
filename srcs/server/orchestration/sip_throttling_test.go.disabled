package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestStandaloneThrottling(t *testing.T) {
	defer ClearSemaphore()
	ClearSemaphore() // Ensure it's clear before starting
	t.Setenv("OHC_MULTITENANT", "false")

	// Force acquireThrottle initialization if not done yet
	acquireThrottle(context.Background())
	releaseThrottle()

	provider := db.NewSqliteProvider(nil) // It doesn't actually need the db for the initial logic check, but let's mock it if possible or use a real db
	s, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	s.db = provider

	// Fill the semaphore
	standaloneThrottle <- struct{}{}

	// DelegateMission should block now. We test that it blocks by using a context with a short timeout.
	shortCtx, shortCancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer shortCancel()

	err = s.DelegateMission(shortCtx, "mission-1", "test-role", Message{Content: "test"})
	if err != shortCtx.Err() {
		t.Errorf("expected context deadline exceeded, got: %v", err)
	}

	// Drain the semaphore to clean up
	<-standaloneThrottle

	// Now it should pass the semaphore check and fail on the DB exec (since we passed a nil db for the provider, or an uninitialized memory db)
	// We just want to check it doesn't block.
}

func TestUpsertMissionThrottling(t *testing.T) {
	defer ClearSemaphore()
	ClearSemaphore() // Ensure it's clear before starting
	t.Setenv("OHC_MULTITENANT", "false")

	// Force acquireThrottle initialization if not done yet
	acquireThrottle(context.Background())
	releaseThrottle()

	s, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}

	// Fill the semaphore
	standaloneThrottle <- struct{}{}

	shortCtx, shortCancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer shortCancel()

	err = s.UpsertMission(shortCtx, "mission-2", "PENDING", "{}", false)
	if err != shortCtx.Err() {
		t.Errorf("expected context deadline exceeded, got: %v", err)
	}

	// Drain the semaphore to clean up
	<-standaloneThrottle
}
