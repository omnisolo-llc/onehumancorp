package analytics

import (
	"context"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var meter = otel.Meter("github.com/onehumancorp/mono/lib/analytics")

var invitesSent metric.Int64Counter
var invitesAccepted metric.Int64Counter

func init() {
    var err error
    invitesSent, err = meter.Int64Counter("ohc_growth_invites_sent")
    if err != nil {
        panic(err)
    }
    invitesAccepted, err = meter.Int64Counter("ohc_growth_invites_accepted")
    if err != nil {
        panic(err)
    }
}

func RecordInvite(ctx context.Context) {
    invitesSent.Add(ctx, 1)
}

func RecordInviteAcceptance(ctx context.Context) {
    invitesAccepted.Add(ctx, 1)
}
