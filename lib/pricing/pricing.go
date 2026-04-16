package pricing

type Tier string

const (
	FreeTier       Tier = "FREE"
	ProTier        Tier = "PRO"
	PremiumTier    Tier = "PREMIUM"
	EnterpriseTier Tier = "ENTERPRISE"
)

type Features struct {
	MaxRAGQueries        int
	MaxAgents            int
	HasAutoDream         bool
	TokenBudget          int64
	PromptCachingEnabled bool
}

func GetFeatures(t Tier) Features {
	switch t {
	case FreeTier:
		return Features{MaxRAGQueries: 100, MaxAgents: 1, HasAutoDream: false, TokenBudget: 10000, PromptCachingEnabled: false}
	case ProTier:
		return Features{MaxRAGQueries: 1000, MaxAgents: 5, HasAutoDream: true, TokenBudget: 500000, PromptCachingEnabled: true}
	case PremiumTier:
		return Features{MaxRAGQueries: 5000, MaxAgents: 20, HasAutoDream: true, TokenBudget: 5000000, PromptCachingEnabled: true}
	case EnterpriseTier:
		return Features{MaxRAGQueries: -1, MaxAgents: -1, HasAutoDream: true, TokenBudget: -1, PromptCachingEnabled: true} // -1 means unlimited
	default:
		return Features{MaxRAGQueries: 100, MaxAgents: 1, HasAutoDream: false, TokenBudget: 10000, PromptCachingEnabled: false}
	}
}
