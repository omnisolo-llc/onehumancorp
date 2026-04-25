package analytics

import (
	"context"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

type Tracker struct {
}

func NewTracker() *Tracker {
	return &Tracker{}
}

func (t *Tracker) TrackEvent(ctx context.Context, name string, props map[string]interface{}) {
	var attrs []attribute.KeyValue
	for k, v := range props {
		if strVal, ok := v.(string); ok {
			attrs = append(attrs, attribute.String(k, strVal))
		}
	}
	opt := metric.WithAttributes(attrs...)

	if name == "invite_sent" && inviteSentCounter != nil {
		inviteSentCounter.Add(ctx, 1, opt)
	} else if name == "invite_accepted" && inviteAcceptedCounter != nil {
		inviteAcceptedCounter.Add(ctx, 1, opt)
	} else if name == "ab_test_impression" && abTestImpressionCounter != nil {
		abTestImpressionCounter.Add(ctx, 1, opt)
	} else if name == "ab_test_conversion" && abTestConversionCounter != nil {
		abTestConversionCounter.Add(ctx, 1, opt)
	} else if name == "quota_exceeded" && quotaExceededCounter != nil {
		quotaExceededCounter.Add(ctx, 1, opt)
	} else if name == "quota_usage_incremented" && quotaUsageCounter != nil {
		quotaUsageCounter.Add(ctx, 1, opt)
	} else if name == "team_invite_sent" && teamInviteSentCounter != nil {
		teamInviteSentCounter.Add(ctx, 1, opt)
	} else if name == "team_invite_accepted" && teamInviteAcceptedCounter != nil {
		teamInviteAcceptedCounter.Add(ctx, 1, opt)
	}
}
