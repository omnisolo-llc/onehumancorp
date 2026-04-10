package hub

// Hub struct contains dependencies for the hub package.
type Hub struct {
	RAGSyncService RAGSyncService
}

// New initializes a Hub
func New(ragSync RAGSyncService) *Hub {
	return &Hub{
		RAGSyncService: ragSync,
	}
}
