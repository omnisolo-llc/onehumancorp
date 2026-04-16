package billing

import (
	"context"
	"fmt"

	"github.com/onehumancorp/mono/lib/pricing"
)

type BillingService struct {
	// dependencies like db, etc. could go here
}

func NewBillingService() *BillingService {
	return &BillingService{}
}

func (s *BillingService) ProcessUsage(ctx context.Context, promptTokens, completionTokens int64) error {
	costAnalysis := pricing.CalculateCost(ctx, promptTokens, completionTokens)
	fmt.Printf("Processed usage: Total Tokens: %d, Estimated Cost: %f\n", costAnalysis.TotalTokens, costAnalysis.EstimatedCost)
	return nil
}
