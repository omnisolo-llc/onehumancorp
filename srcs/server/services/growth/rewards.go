package growth

import (
	"context"
	"fmt"
)

type RewardsManager struct {
	tracker *InviteTracker
}

func NewRewardsManager(tracker *InviteTracker) *RewardsManager {
	return &RewardsManager{tracker: tracker}
}

func (rm *RewardsManager) CheckAndGrantReward(ctx context.Context, teamID string) (string, error) {
	count, err := rm.tracker.GetTeamInvitesCount(ctx, teamID)
	if err != nil {
		return "", err
	}

	if count >= 5 {
		return "power_team_badge", nil
	} else if count >= 1 {
		return "active_team_badge", nil
	}

	return "", fmt.Errorf("no rewards eligible")
}
