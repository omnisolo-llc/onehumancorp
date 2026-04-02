<<<<<<< SEARCH
		minimaxKey := os.Getenv("MINIMAX_API_KEY")
		resolvedContext := ""
		if minimaxKey != "" {
			client := NewMinimaxClient(minimaxKey)
			ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
			response, err := client.Reason(ctxTimeout, prompt)
			cancel()
			if err != nil {
				slog.Warn("AutoDream: LLM reasoning failed, fallback to concatenation", "error", err)
				resolvedContext = "Consolidated memory: " + c.Context1 + " & " + c.Context2
			} else {
				resolvedContext = response
			}
		} else {
			slog.Warn("AutoDream: MINIMAX_API_KEY not set, using placeholder consolidation")
			resolvedContext = "Consolidated memory: " + c.Context1 + " & " + c.Context2
		}
=======
		minimaxKey := os.Getenv("MINIMAX_API_KEY")
		resolvedContext := ""
		if minimaxKey != "" {
			client := NewMinimaxClient(minimaxKey, "https://api.minimax.chat")
			ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
			response, err := client.Reason(ctxTimeout, prompt)
			cancel()
			if err != nil {
				slog.Warn("AutoDream: LLM reasoning failed, fallback to concatenation", "error", err)
				resolvedContext = "Consolidated memory: " + c.Context1 + " & " + c.Context2
			} else {
				resolvedContext = response
			}
		} else {
			slog.Warn("AutoDream: MINIMAX_API_KEY not set, using placeholder consolidation")
			resolvedContext = "Consolidated memory: " + c.Context1 + " & " + c.Context2
		}
>>>>>>> REPLACE
