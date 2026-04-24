	if adConsolidator != nil {
		go func() {
			ticker := time.NewTicker(24 * time.Hour)
			defer ticker.Stop()
			for {
				select {
				case <-ctx.Done():
					return
				case <-ticker.C:
					_ = adConsolidator.PruneStaleMemories(ctx, "system", time.Now().Add(-30*24*time.Hour))
				}
			}
		}()
	}
