package growth

import (
    "context"
    "fmt"
    "github.com/onehumancorp/mono/lib/analytics"
    "github.com/google/uuid"
)

type ReferralManager struct {
    codes map[string]string
}

func NewReferralManager() *ReferralManager {
    return &ReferralManager{
        codes: make(map[string]string),
    }
}

func (rm *ReferralManager) GenerateInvite(ctx context.Context, userID string) string {
    code := uuid.New().String()
    rm.codes[code] = userID
    analytics.RecordInvite(ctx)
    return code
}

func (rm *ReferralManager) AcceptInvite(ctx context.Context, code string) error {
    if _, exists := rm.codes[code]; !exists {
        return fmt.Errorf("invalid referral code")
    }
    analytics.RecordInviteAcceptance(ctx)
    return nil
}
