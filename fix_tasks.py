with open("srcs/server/orchestration/tasks.go", "r") as f:
    content = f.read()

content = content.replace("type dummyLLM struct{}\nfunc (d *dummyLLM) Summarize(ctx context.Context, text string) (string, error) { return \"Summary: \" + text, nil }\n\ntype dummyEmbedding struct{}\nfunc (d *dummyEmbedding) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) { return []float32{0.1, 0.2, 0.3}, nil }\n\n", "")

complete_patch = """
	// AutoDream is triggered by sync daemon or another worker.
	// Production logic runs separately via AutoDreamPipeline.
"""
content = content.replace("""	// Wire AutoDream
	go func() {
		// Simple fallback dummy implementations for LLM and Embedding for now
		repo := memory.NewVectorRepository(tm.db)
		service := autodream.NewAutoDreamService(repo, &dummyLLM{}, &dummyEmbedding{})
		// In real usage, fetch logs from eventlog. Here we just use a placeholder text
		_ = service.ConsolidateTaskMemory(context.Background(), claims.OrganizationID, taskID, "Task completed successfully.")
	}()""", complete_patch)

with open("srcs/server/orchestration/tasks.go", "w") as f:
    f.write(content)
