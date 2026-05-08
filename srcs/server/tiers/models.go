package tiers

// TierType defines the subscription tiers available
type TierType string

const (
	TierFree     TierType = "free"
	TierStarter  TierType = "starter"
	TierPro      TierType = "pro"
	TierBusiness TierType = "business"
)

// TierLimits defines the constraints for a specific tier
type TierLimits struct {
	MaxProducts     int
	MaxAIDepartments int
	MaxAIActions    int
	MaxStorageBytes int64
}

// LimitsByTier defines the hardcoded limits based on the design doc
var LimitsByTier = map[TierType]TierLimits{
	TierFree: {
		MaxProducts:     10,
		MaxAIDepartments: 1,
		MaxAIActions:    100,
		MaxStorageBytes: 500 * 1024 * 1024, // 500MB
	},
	TierStarter: {
		MaxProducts:     100,
		MaxAIDepartments: 3,
		MaxAIActions:    1000,
		MaxStorageBytes: 5 * 1024 * 1024 * 1024, // 5GB
	},
	TierPro: {
		MaxProducts:     -1, // Unlimited
		MaxAIDepartments: 10,
		MaxAIActions:    -1, // Unlimited
		MaxStorageBytes: 50 * 1024 * 1024 * 1024, // 50GB
	},
	TierBusiness: {
		MaxProducts:     -1, // Unlimited
		MaxAIDepartments: -1, // Unlimited
		MaxAIActions:    -1, // Unlimited
		MaxStorageBytes: 500 * 1024 * 1024 * 1024, // 500GB
	},
}

// UsageMetrics holds current usage for a tenant
type UsageMetrics struct {
	ProductCount    int
	AIActionsMonth  int
	StorageBytes    int64
}
