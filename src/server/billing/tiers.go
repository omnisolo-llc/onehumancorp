package billing

import (
	"errors"
)

type SaasTier string

const (
	TierFree     SaasTier = "free"
	TierStarter  SaasTier = "starter"
	TierPro      SaasTier = "pro"
	TierBusiness SaasTier = "business"
)

type TierLimits struct {
	MaxProducts    int
	MaxDepartments int
	MaxAIActions   int
	MaxStorageMB   int
}

func GetTierLimits(tier SaasTier) (TierLimits, error) {
	switch tier {
	case TierFree:
		return TierLimits{10, 1, 100, 500}, nil
	case TierStarter:
		return TierLimits{100, 3, 1000, 5000}, nil
	case TierPro:
		return TierLimits{-1, 10, -1, 50000}, nil
	case TierBusiness:
		return TierLimits{-1, -1, -1, 500000}, nil
	default:
		return TierLimits{}, errors.New("unknown tier")
	}
}
