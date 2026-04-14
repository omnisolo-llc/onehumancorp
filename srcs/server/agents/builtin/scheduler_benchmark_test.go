package builtin

import (
	"context"
	"fmt"
	"sync/atomic"
	"testing"
)

func BenchmarkBuiltinSchedule100Plus(b *testing.B) {
	for i := 0; i < 50; i++ {
		totalAgents := 100 + i
		b.Run(fmt.Sprintf("subagents_%d", totalAgents), func(b *testing.B) {
			workers := 16
			if totalAgents < workers {
				workers = totalAgents
			}
			b.ReportAllocs()
			for n := 0; n < b.N; n++ {
				var done int64
				err := ScheduleSubagents(context.Background(), totalAgents, workers, func(ctx context.Context, index int) error {
					atomic.AddInt64(&done, 1)
					return nil
				})
				if err != nil {
					b.Fatalf("schedule error: %v", err)
				}
				if got := int(atomic.LoadInt64(&done)); got != totalAgents {
					b.Fatalf("expected %d tasks done, got %d", totalAgents, got)
				}
			}
		})
	}
}
