package growth

import (
    "context"
    "testing"
)

func TestGenerateAndAcceptInvite(t *testing.T) {
    rm := NewReferralManager()
    ctx := context.Background()
    code := rm.GenerateInvite(ctx, "user123")
    if code == "" {
        t.Fatal("expected a code, got empty string")
    }

    err := rm.AcceptInvite(ctx, code)
    if err != nil {
        t.Fatalf("expected no error on valid code, got %v", err)
    }

    err = rm.AcceptInvite(ctx, "invalid-code")
    if err == nil {
        t.Fatal("expected error on invalid code, got nil")
    }
}
