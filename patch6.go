<<<<<<< SEARCH
type AutoDreamEngine struct {
	db        db.Provider
	llmClient orchestration.MinimaxClient
	ticker    *time.Ticker
	quit      chan struct{}
}

// NewAutoDreamEngine initializes the autoDream engine.
func NewAutoDreamEngine(db db.Provider, llmClient orchestration.MinimaxClient) *AutoDreamEngine {
	return &AutoDreamEngine{
		db:        db,
		llmClient: llmClient,
		quit:      make(chan struct{}),
	}
}
=======
type AutoDreamEngine struct {
	db        db.Provider
	llmClient *orchestration.MinimaxClient
	ticker    *time.Ticker
	quit      chan struct{}
}

// NewAutoDreamEngine initializes the autoDream engine.
func NewAutoDreamEngine(db db.Provider, llmClient *orchestration.MinimaxClient) *AutoDreamEngine {
	return &AutoDreamEngine{
		db:        db,
		llmClient: llmClient,
		quit:      make(chan struct{}),
	}
}
>>>>>>> REPLACE
