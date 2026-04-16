package analytics

import (
    "context"
    "testing"
)

func TestRecordInvite(t *testing.T) {
    RecordInvite(context.Background())
}

func TestRecordInviteAcceptance(t *testing.T) {
    RecordInviteAcceptance(context.Background())
}
