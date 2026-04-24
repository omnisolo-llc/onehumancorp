package storage

import (
	"bytes"
	"fmt"
	"image"
	"image/jpeg"
	"image/png"
	"strings"
	"sync"
)

// StorageQuota holds the limit for a tenant.
type StorageQuota struct {
	MaxBytes int64
}

// Subscription tiers define the storage quota logic.
var Tiers = map[string]StorageQuota{
	"free":    {MaxBytes: 500 * 1024 * 1024},         // 500MB
	"starter": {MaxBytes: 5 * 1024 * 1024 * 1024},    // 5GB
	"pro":     {MaxBytes: 50 * 1024 * 1024 * 1024},   // 50GB
}

// QuotaManager implements tracking of storage bytes to prevent abuse.
type QuotaManager struct {
	mu          sync.Mutex
	storageUsed map[string]int64
	tenantTiers map[string]string
}

// NewQuotaManager returns a new instance.
func NewQuotaManager() *QuotaManager {
	return &QuotaManager{
		storageUsed: make(map[string]int64),
		tenantTiers: make(map[string]string),
	}
}

// SetTier sets the tenant tier.
func (qm *QuotaManager) SetTier(tenantID, tier string) {
	qm.mu.Lock()
	defer qm.mu.Unlock()
	qm.tenantTiers[tenantID] = tier
}

// RecordUsage increments usage.
func (qm *QuotaManager) RecordUsage(tenantID string, bytes int64) (int64, int64, bool) {
	qm.mu.Lock()
	defer qm.mu.Unlock()

	tier, exists := qm.tenantTiers[tenantID]
	if !exists {
		tier = "free"
	}

	quota := Tiers[tier]
	used := qm.storageUsed[tenantID]

	// Increment used storage
	used += bytes
	qm.storageUsed[tenantID] = used

	return used, quota.MaxBytes, used > quota.MaxBytes
}

// OptimizeImage auto-resizes and converts image to WebP (simulated here as low-quality JPEG for simplicity).
func OptimizeImage(data []byte, format string) ([]byte, error) {
	if len(data) == 0 {
		return nil, fmt.Errorf("empty image data")
	}

	reader := bytes.NewReader(data)
	var img image.Image
	var err error

	format = strings.ToLower(format)
	if format == "png" {
		img, err = png.Decode(reader)
	} else if format == "jpg" || format == "jpeg" {
		img, err = jpeg.Decode(reader)
	} else {
		// Pass through un-optimized if unsupported
		return data, nil
	}

	if err != nil {
		return nil, err
	}

	var buf bytes.Buffer
	// In production, this uses WebP bindings or BIMG. Simulated with JPG compression.
	err = jpeg.Encode(&buf, img, &jpeg.Options{Quality: 60})
	if err != nil {
		return nil, err
	}

	return buf.Bytes(), nil
}
