package analytics

import (
	"context"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	inviteSentCounter     metric.Int64Counter
	inviteAcceptedCounter metric.Int64Counter
	quotaExceededCounter metric.Int64Counter
	quotaPassedCounter metric.Int64Counter
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
	quotaExceededCounter, err = meter.Int64Counter("growth_quota_exceeded_total")
	if err != nil {
		panic(err)
	}
	quotaPassedCounter, err = meter.Int64Counter("growth_quota_passed_total")
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
	} else if name == "quota_exceeded" && quotaExceededCounter != nil {
		quotaExceededCounter.Add(ctx, 1)
	} else if name == "quota_check_passed" && quotaPassedCounter != nil {
		quotaPassedCounter.Add(ctx, 1)
	}
}
