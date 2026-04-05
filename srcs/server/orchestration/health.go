package orchestration

import (
	"time"
)

// HybridHealthProbe contains health guardianship diagnostics for hybrid environments.
type HybridHealthProbe struct {
	Mode        string        `json:"mode"`
	Status      string        `json:"status"`
	DBPing      time.Duration `json:"db_ping"`
	SyncBacklog int           `json:"sync_backlog"`
	MeshActive  bool          `json:"mesh_active"`
}
