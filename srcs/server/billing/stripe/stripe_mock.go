package stripe

import (
	"context"
	"fmt"
	"time"
)

type PlanTier string

const (
	TierFree     PlanTier = "free"
	TierStarter  PlanTier = "starter"
	TierPro      PlanTier = "pro"
	TierBusiness PlanTier = "business"
)

type PlanDetail struct {
	ID              string   `json:"id"`
	Name            string   `json:"name"`
	PriceUSD        float64  `json:"priceUsd"`
	Interval        string   `json:"interval"`
	ProductsLimit   int      `json:"productsLimit"`
	AIDepartments   int      `json:"aiDepartments"`
	AIActionsLimit  int      `json:"aiActionsLimit"`
	StorageLimitMB  int      `json:"storageLimitMb"`
	HasCustomDomain bool     `json:"hasCustomDomain"`
	Features        []string `json:"features"`
}

var AvailablePlans = []PlanDetail{
	{
		ID:              "plan_free",
		Name:            "Free",
		PriceUSD:        0,
		Interval:        "month",
		ProductsLimit:   10,
		AIDepartments:   1,
		AIActionsLimit:  100,
		StorageLimitMB:  500,
		HasCustomDomain: false,
		Features:        []string{"Basic Storefront", "1 AI Agent", "Standard Support"},
	},
	{
		ID:              "plan_starter",
		Name:            "Starter",
		PriceUSD:        9.00,
		Interval:        "month",
		ProductsLimit:   100,
		AIDepartments:   3,
		AIActionsLimit:  1000,
		StorageLimitMB:  5120,
		HasCustomDomain: true,
		Features:        []string{"Custom Domain", "3 AI Agents", "Priority Support"},
	},
	{
		ID:              "plan_pro",
		Name:            "Pro",
		PriceUSD:        29.00,
		Interval:        "month",
		ProductsLimit:   -1,
		AIDepartments:   10,
		AIActionsLimit:  -1,
		StorageLimitMB:  51200,
		HasCustomDomain: true,
		Features:        []string{"Unlimited Products", "10 AI Agents", "Advanced Analytics"},
	},
	{
		ID:              "plan_business",
		Name:            "Business",
		PriceUSD:        79.00,
		Interval:        "month",
		ProductsLimit:   -1,
		AIDepartments:   -1,
		AIActionsLimit:  -1,
		StorageLimitMB:  512000,
		HasCustomDomain: true,
		Features:        []string{"Unlimited Agents", "Dedicated Support", "API Access"},
	},
}

type Subscription struct {
	ID             string    `json:"id"`
	OrganizationID string    `json:"organizationId"`
	Plan           PlanTier  `json:"plan"`
	Status         string    `json:"status"`
	CurrentPeriodEnd time.Time `json:"currentPeriodEnd"`
}

type Service struct {
	subscriptions map[string]*Subscription
}

func NewService() *Service {
	return &Service{
		subscriptions: make(map[string]*Subscription),
	}
}

func (s *Service) CreateCheckoutSession(ctx context.Context, orgID string, planID string) (string, error) {
	return fmt.Sprintf("https://checkout.stripe.com/pay/cs_test_%s_%s", orgID, planID), nil
}

func (s *Service) CreateCustomerPortalSession(ctx context.Context, orgID string) (string, error) {
	return fmt.Sprintf("https://billing.stripe.com/p/session/test_%s", orgID), nil
}

func (s *Service) GetSubscription(ctx context.Context, orgID string) (*Subscription, error) {
	if sub, ok := s.subscriptions[orgID]; ok {
		return sub, nil
	}
	return &Subscription{
		ID:             "sub_default_free",
		OrganizationID: orgID,
		Plan:           TierFree,
		Status:         "active",
		CurrentPeriodEnd: time.Now().AddDate(1, 0, 0),
	}, nil
}
