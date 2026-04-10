package billing

import (
	"errors"
	"github.com/onehumancorp/mono/lib/pricing"
)

type BillingService struct {
	calculator *pricing.TokenCostCalculator
}

func NewBillingService() *BillingService {
	return &BillingService{
		calculator: pricing.NewTokenCostCalculator(),
	}
}

func (s *BillingService) BillCustomer(customerID, model string, input, output, cached int) (float64, error) {
	if customerID == "" {
		return 0, errors.New("invalid customer ID")
	}
	return s.calculator.CalculateCost(model, input, output, cached)
}
