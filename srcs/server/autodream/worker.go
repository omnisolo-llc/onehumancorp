package autodream

import (
    "context"
    "time"
    "encoding/json"
    "fmt"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type LLMClient interface {
    Chat(ctx context.Context, prompt string) (string, error)
    GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamWorker struct {
    store    VectorStore
    provider db.Provider
    llm      LLMClient
}

func NewAutoDreamWorker(store VectorStore, provider db.Provider, llm LLMClient) *AutoDreamWorker {
    return &AutoDreamWorker{
        store:    store,
        provider: provider,
        llm:      llm,
    }
}

// Start runs the worker loop periodically.
func (w *AutoDreamWorker) Start(ctx context.Context) {
    ticker := time.NewTicker(5 * time.Minute)
    defer ticker.Stop()

    for {
        select {
        case <-ctx.Done():
            return
        case <-ticker.C:
            _ = w.Process(ctx)
            _ = w.ResolveConflicts(ctx)
        }
    }
}

func (w *AutoDreamWorker) Process(ctx context.Context) error {
    var query string
    if w.provider.IsSQLite() {
        query = "SELECT session_id, context_data FROM agent_session_data LIMIT 10"
    } else {
        query = "SELECT session_id, context_data FROM agent_session_data LIMIT 10 FOR UPDATE SKIP LOCKED"
    }

    rows, err := w.provider.Query(ctx, query)
    if err != nil {
        return err
    }

    type taskData struct {
        ID      string
        Context string
    }
    var tasks []taskData
    for rows.Next() {
        var t taskData
        if err := rows.Scan(&t.ID, &t.Context); err == nil {
            tasks = append(tasks, t)
        }
    }
    rows.Close()

    for _, t := range tasks {
        // Use injected LLM client to generate embedding for the actual context
        embedding, err := w.llm.GenerateEmbedding(ctx, t.Context)
        if err != nil {
            continue
        }

        meta := map[string]interface{}{
            "source":  "agent_session",
            "content": t.Context,
        }

        if err := w.store.Store(ctx, t.ID, embedding, meta); err != nil {
            continue
        }

        delQuery := "DELETE FROM agent_session_data WHERE session_id = $1"
        _, _ = w.provider.Exec(ctx, delQuery, t.ID)
    }

    return nil
}

// ResolveConflicts handles the required "Conflict Resolution Logic" using an LLM pipeline.
func (w *AutoDreamWorker) ResolveConflicts(ctx context.Context) error {
    var query string
    if w.provider.IsSQLite() {
        query = "SELECT conflict_id, memory_id_1, memory_id_2 FROM memory_conflicts WHERE resolution_status = 'PENDING' LIMIT 10"
    } else {
        query = "SELECT conflict_id, memory_id_1, memory_id_2 FROM memory_conflicts WHERE resolution_status = 'PENDING' LIMIT 10 FOR UPDATE SKIP LOCKED"
    }

    rows, err := w.provider.Query(ctx, query)
    if err != nil {
        return err
    }

    type conflictData struct {
        ID   string
        Mem1 string
        Mem2 string
    }
    var conflicts []conflictData
    for rows.Next() {
        var c conflictData
        if err := rows.Scan(&c.ID, &c.Mem1, &c.Mem2); err == nil {
            conflicts = append(conflicts, c)
        }
    }
    rows.Close()

    for _, c := range conflicts {
        // Query the knowledge base for mem1 and mem2 contents
        var m1Meta, m2Meta string
        err1 := w.provider.QueryRow(ctx, "SELECT metadata FROM knowledge_base WHERE id = $1", c.Mem1).Scan(&m1Meta)
        err2 := w.provider.QueryRow(ctx, "SELECT metadata FROM knowledge_base WHERE id = $1", c.Mem2).Scan(&m2Meta)

        if err1 != nil || err2 != nil {
            continue // skip if missing
        }

        var meta1, meta2 map[string]interface{}
        _ = json.Unmarshal([]byte(m1Meta), &meta1)
        _ = json.Unmarshal([]byte(m2Meta), &meta2)

        // Use injected LLM to synthesize conflict resolution
        prompt := fmt.Sprintf("Resolve the contradiction between memory 1: %v and memory 2: %v.", meta1["content"], meta2["content"])
        synthesizedMemory, err := w.llm.Chat(ctx, prompt)
        if err != nil {
            continue
        }

        embedding, err := w.llm.GenerateEmbedding(ctx, synthesizedMemory)
        if err != nil {
            continue
        }

        synthMeta := map[string]interface{}{
            "source":   "conflict_resolution",
            "resolved": true,
            "content":  synthesizedMemory,
        }

        // Store resolved memory
        w.store.Store(ctx, "resolved_"+c.ID, embedding, synthMeta)

        // Delete old
        w.provider.Exec(ctx, "DELETE FROM knowledge_base WHERE id IN ($1, $2)", c.Mem1, c.Mem2)

        // Update status
        w.provider.Exec(ctx, "UPDATE memory_conflicts SET resolution_status = 'RESOLVED', resolved_memory_id = $1 WHERE conflict_id = $2", "resolved_"+c.ID, c.ID)
    }

    return nil
}
