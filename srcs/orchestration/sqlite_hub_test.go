package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

func TestSqliteHubRepository(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite memory db: %v", err)
	}
	defer db.Close()

	if err := initializeTables(db); err != nil {
		t.Fatalf("failed to initialize tables: %v", err)
	}

	repo := NewSqliteHubRepository(db)
	ctx := context.Background()

	// Agent Registry
	agent := Agent{
		ID:             "agent-1",
		Name:           "Test Agent",
		Role:           "Tester",
		OrganizationID: "org-1",
		Status:         StatusIdle,
	}

	if err := repo.RegisterAgent(ctx, agent); err != nil {
		t.Fatalf("RegisterAgent: %v", err)
	}

	a, ok, err := repo.GetAgent(ctx, "agent-1")
	if err != nil || !ok {
		t.Fatalf("GetAgent failed: %v, ok=%v", err, ok)
	}
	if a.Name != "Test Agent" {
		t.Errorf("Expected Test Agent, got %s", a.Name)
	}

	if err := repo.UpdateAgentStatus(ctx, "agent-1", StatusActive); err != nil {
		t.Fatalf("UpdateAgentStatus: %v", err)
	}

	a, _, _ = repo.GetAgent(ctx, "agent-1")
	if a.Status != StatusActive {
		t.Errorf("Expected ACTIVE status, got %s", a.Status)
	}

	// Message Inbox
	msg := Message{
		ID:         "msg-1",
		FromAgent:  "agent-2",
		ToAgent:    "agent-1",
		Type:       "test-message",
		Content:    "hello world",
		OccurredAt: time.Now().UTC(),
	}

	if err := repo.PushMessage(ctx, "agent-1", msg); err != nil {
		t.Fatalf("PushMessage: %v", err)
	}

	msgs, err := repo.PeekMessages(ctx, "agent-1")
	if err != nil || len(msgs) != 1 {
		t.Fatalf("PeekMessages failed: err=%v, len=%d", err, len(msgs))
	}

	msgs, err = repo.PopMessages(ctx, "agent-1")
	if err != nil || len(msgs) != 1 {
		t.Fatalf("PopMessages failed: err=%v, len=%d", err, len(msgs))
	}

	msgs, _ = repo.PeekMessages(ctx, "agent-1")
	if len(msgs) != 0 {
		t.Fatalf("Expected 0 messages after pop, got %d", len(msgs))
	}

	// Meeting Rooms
	room := MeetingRoom{
		ID:           "room-1",
		Agenda:       "Test Agenda",
		Participants: []string{"agent-1", "agent-2"},
	}

	if err := repo.CreateMeeting(ctx, room); err != nil {
		t.Fatalf("CreateMeeting: %v", err)
	}

	r, ok, err := repo.GetMeeting(ctx, "room-1")
	if err != nil || !ok {
		t.Fatalf("GetMeeting failed: %v, ok=%v", err, ok)
	}
	if len(r.Participants) != 2 || r.Participants[0] != "agent-1" {
		t.Errorf("Expected 2 participants, got %v", r.Participants)
	}

	msg2 := Message{
		ID:         "msg-2",
		FromAgent:  "agent-1",
		Type:       "chat",
		Content:    "hi",
		MeetingID:  "room-1",
		OccurredAt: time.Now().UTC(),
	}
	if err := repo.AppendTranscript(ctx, "room-1", msg2); err != nil {
		t.Fatalf("AppendTranscript: %v", err)
	}

	r, _, _ = repo.GetMeeting(ctx, "room-1")
	if len(r.Transcript) != 1 {
		t.Fatalf("Expected 1 transcript message, got %d", len(r.Transcript))
	}
}
