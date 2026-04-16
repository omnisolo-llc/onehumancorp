package analytics

import (
	"context"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	inviteSentCounter         metric.Int64Counter
	inviteAcceptedCounter     metric.Int64Counter
	teamInviteSentCounter     metric.Int64Counter
	teamInviteAcceptedCounter metric.Int64Counter
	abTestImpressionCounter   metric.Int64Counter
	abTestConversionCounter   metric.Int64Counter
	quotaExceededCounter      metric.Int64Counter
	quotaUsageCounter         metric.Int64Counter
)

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc")
	var err error
	inviteSentCounter, err = meter.Int64Counter("growth_viral_invite_sent_total")
	if err != nil {
		panic(err)
	}
	inviteAcceptedCounter, err = meter.Int64Counter("growth_viral_invite_accepted_total")
	if err != nil {
		panic(err)
	}
	teamInviteSentCounter, err = meter.Int64Counter("growth_team_invite_sent_total")
	if err != nil {
		panic(err)
	}
	teamInviteAcceptedCounter, err = meter.Int64Counter("growth_team_invite_accepted_total")
	if err != nil {
		panic(err)
	}
	abTestImpressionCounter, err = meter.Int64Counter("growth_ab_test_impression_total")
	if err != nil {
		panic(err)
	}
	abTestConversionCounter, err = meter.Int64Counter("growth_ab_test_conversion_total")
	if err != nil {
		panic(err)
	}
	quotaExceededCounter, err = meter.Int64Counter("ohc_growth_quota_exceeded_total")
	if err != nil {
		panic(err)
	}
	quotaUsageCounter, err = meter.Int64Counter("ohc_growth_quota_usage_incremented_total")
	if err != nil {
		panic(err)
	}
}

type Tracker struct {
}

func NewTracker() *Tracker {
	return &Tracker{}
}

func (t *Tracker) TrackEvent(ctx context.Context, name string, props map[string]interface{}) {
	if name == "invite_sent" && inviteSentCounter != nil {
		inviteSentCounter.Add(ctx, 1)
	} else if name == "invite_accepted" && inviteAcceptedCounter != nil {
		inviteAcceptedCounter.Add(ctx, 1)
	} else if name == "team_invite_sent" && teamInviteSentCounter != nil {
		teamInviteSentCounter.Add(ctx, 1)
	} else if name == "team_invite_accepted" && teamInviteAcceptedCounter != nil {
		teamInviteAcceptedCounter.Add(ctx, 1)
	} else if name == "ab_test_impression" && abTestImpressionCounter != nil {
		abTestImpressionCounter.Add(ctx, 1)
	} else if name == "ab_test_conversion" && abTestConversionCounter != nil {
		abTestConversionCounter.Add(ctx, 1)
	} else if name == "quota_exceeded" && quotaExceededCounter != nil {
		quotaExceededCounter.Add(ctx, 1)
	} else if name == "quota_usage_incremented" && quotaUsageCounter != nil {
		quotaUsageCounter.Add(ctx, 1)
	}
}
