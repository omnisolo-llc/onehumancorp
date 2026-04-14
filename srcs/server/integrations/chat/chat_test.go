package chat_test

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/integrations/chat"
)

// ─── MemStore tests ────────────────────────────────────────────────────────────

func TestMemStoreAppendReplay(t *testing.T) {
	ctx := context.Background()
	store := chat.NewMemStore()

	msg := chat.Message{
		ConversationID: "conv-1",
		Channel:        "#general",
		Sender:         "alice",
		Text:           "Hello!",
	}
	id, err := store.Append(ctx, msg)
	if err != nil {
		t.Fatalf("Append: %v", err)
	}
	if id == "" {
		t.Fatal("expected non-empty ID")
	}

	msgs, err := store.Replay(ctx, "conv-1", 100)
	if err != nil {
		t.Fatalf("Replay: %v", err)
	}
	if len(msgs) != 1 {
		t.Fatalf("expected 1 message, got %d", len(msgs))
	}
	if msgs[0].Text != "Hello!" {
		t.Errorf("unexpected text: %q", msgs[0].Text)
	}
}

func TestMemStoreReplayFiltering(t *testing.T) {
	ctx := context.Background()
	store := chat.NewMemStore()

	for i := 0; i < 5; i++ {
		cid := "conv-A"
		if i%2 == 0 {
			cid = "conv-B"
		}
		_, _ = store.Append(ctx, chat.Message{
			ConversationID: cid,
			Text:           fmt.Sprintf("msg-%d", i),
		})
	}

	msgsA, _ := store.Replay(ctx, "conv-A", 0)
	msgsB, _ := store.Replay(ctx, "conv-B", 0)
	all, _ := store.Replay(ctx, "", 0)

	if len(msgsA) != 2 {
		t.Errorf("expected 2 conv-A messages, got %d", len(msgsA))
	}
	if len(msgsB) != 3 {
		t.Errorf("expected 3 conv-B messages, got %d", len(msgsB))
	}
	if len(all) != 5 {
		t.Errorf("expected 5 total messages, got %d", len(all))
	}
}

func TestMemStoreReplayLimit(t *testing.T) {
	ctx := context.Background()
	store := chat.NewMemStore()

	for i := 0; i < 20; i++ {
		_, _ = store.Append(ctx, chat.Message{
			ConversationID: "limited",
			Text:           fmt.Sprintf("msg-%d", i),
		})
	}

	msgs, err := store.Replay(ctx, "limited", 5)
	if err != nil {
		t.Fatalf("Replay: %v", err)
	}
	if len(msgs) != 5 {
		t.Errorf("expected 5 messages with limit, got %d", len(msgs))
	}
}

func TestMemStoreMissingConversationID(t *testing.T) {
	ctx := context.Background()
	store := chat.NewMemStore()
	_, err := store.Append(ctx, chat.Message{Text: "no conv"})
	if err == nil {
		t.Fatal("expected error for missing conversation_id")
	}
}

func TestMemStoreTimestamp(t *testing.T) {
	ctx := context.Background()
	store := chat.NewMemStore()
	before := time.Now()
	_, _ = store.Append(ctx, chat.Message{ConversationID: "ts-test", Text: "ts"})
	after := time.Now()

	msgs, _ := store.Replay(ctx, "ts-test", 1)
	if len(msgs) == 0 {
		t.Fatal("no messages")
	}
	ts := msgs[0].Timestamp
	if ts.Before(before.Add(-time.Second)) || ts.After(after.Add(time.Second)) {
		t.Errorf("timestamp out of range: %v (want between %v and %v)", ts, before, after)
	}
}

func TestMemStoreIDUniqueness(t *testing.T) {
	ctx := context.Background()
	store := chat.NewMemStore()
	seen := map[string]bool{}
	for i := 0; i < 100; i++ {
		id, err := store.Append(ctx, chat.Message{
			ConversationID: "uniq",
			Text:           "msg",
		})
		if err != nil {
			t.Fatalf("Append %d: %v", i, err)
		}
		if seen[id] {
			t.Fatalf("duplicate ID at iteration %d: %q", i, id)
		}
		seen[id] = true
	}
}

func TestMemStoreLen(t *testing.T) {
	ctx := context.Background()
	store := chat.NewMemStore()
	if store.Len() != 0 {
		t.Fatalf("expected 0, got %d", store.Len())
	}
	for i := 0; i < 7; i++ {
		_, _ = store.Append(ctx, chat.Message{ConversationID: "len", Text: "x"})
	}
	if store.Len() != 7 {
		t.Fatalf("expected 7, got %d", store.Len())
	}
}

func TestMemStoreReplayInOrder(t *testing.T) {
	ctx := context.Background()
	store := chat.NewMemStore()

	texts := []string{"alpha", "beta", "gamma", "delta"}
	for _, text := range texts {
		_, _ = store.Append(ctx, chat.Message{ConversationID: "order", Text: text})
		time.Sleep(time.Millisecond) // ensure distinct timestamps
	}

	msgs, _ := store.Replay(ctx, "order", 0)
	if len(msgs) != len(texts) {
		t.Fatalf("expected %d messages, got %d", len(texts), len(msgs))
	}
	for i, m := range msgs {
		if m.Text != texts[i] {
			t.Errorf("position %d: expected %q, got %q", i, texts[i], m.Text)
		}
	}
}

// ─── Store interface compliance ────────────────────────────────────────────────

// Verify MemStore satisfies the Store interface at compile time.
var _ chat.Store = (*chat.MemStore)(nil)

// ─── Concurrent access test ────────────────────────────────────────────────────

func TestMemStoreConcurrentAppend(t *testing.T) {
	ctx := context.Background()
	store := chat.NewMemStore()
	done := make(chan struct{})

	for i := 0; i < 50; i++ {
		go func(i int) {
			_, _ = store.Append(ctx, chat.Message{
				ConversationID: "concurrent",
				Text:           fmt.Sprintf("worker-%d", i),
			})
			done <- struct{}{}
		}(i)
	}

	for i := 0; i < 50; i++ {
		<-done
	}

	msgs, _ := store.Replay(ctx, "concurrent", 0)
	if len(msgs) != 50 {
		t.Errorf("expected 50 messages, got %d", len(msgs))
	}
}

// ─── Chat message structure tests ─────────────────────────────────────────────

func TestMessageFields(t *testing.T) {
	ctx := context.Background()
	store := chat.NewMemStore()
	msg := chat.Message{
		ConversationID: "fields-test",
		Channel:        "#devops",
		Sender:         "bot",
		Text:           "Deployment complete",
	}
	id, err := store.Append(ctx, msg)
	if err != nil {
		t.Fatalf("Append: %v", err)
	}

	msgs, _ := store.Replay(ctx, "fields-test", 1)
	if len(msgs) == 0 {
		t.Fatal("no messages")
	}
	m := msgs[0]
	if m.ID != id {
		t.Errorf("ID mismatch: %q vs %q", m.ID, id)
	}
	if m.Channel != "#devops" {
		t.Errorf("Channel: %q", m.Channel)
	}
	if m.Sender != "bot" {
		t.Errorf("Sender: %q", m.Sender)
	}
	if m.Text != "Deployment complete" {
		t.Errorf("Text: %q", m.Text)
	}
}
