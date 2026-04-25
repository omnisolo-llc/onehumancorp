package orchestration

import (
	"fmt"
	"strings"
)






// SetMeshTransport configures the transport layer to use for cross-node mesh broadcasts

// Start runs the AutoDream background pipelines.

// runCompletedTasksIngestionPipeline processes COMPLETED tasks into autodream_memories.

// ingestCompletedTasks queries shared_tasks for COMPLETED status and adds them to autodream_memories

// runSessionCompressionPipeline periodically compresses context from agent_session_data.

// compressSessionData reads agent_session_data and inserts it into autodream_memories.

// runMemoryIngestionPipeline reads imported runtime memory files and injects them.

// compressSessionContexts periodically compresses context from agent_session_data into autodream_memories.

// ingestAgentMemories processes memory YAML files from the directory
// configured by OHC_MEMORY_DIR.  When that env var is empty the pipeline is
// a no-op — agents write memory directly to the database.

// runPruningPipeline periodically prunes stale agent session data.


// pruneStaleSessions deletes agent_session_data older than 24 hours and compresses it.

// runConflictResolutionPipeline detects contradicting knowledge in the vector database.

// resolveConflicts finds vector embeddings that are similar but have conflicting contexts.

// InjectTruth inserts high-dimensional semantic memory directly into the store.
// embedding expects a valid vector string representation like "[0.1, 0.2, 0.3]" for pgvector, or equivalent array.

// TruthSearchResult represents a semantic search result from pgvector.
type TruthSearchResult struct {
	MemoryID string
	Context  string
	Distance float64
}

// SearchTruth queries the vector database for the closest semantic embeddings.

// ConsolidateEpoch runs a continuous long-term memory consolidation pipeline
// by creating a swarm_dream_epochs record and clustering knowledge.

// runMissionIngestionPipeline reads imported task artifacts from the directory
// configured by OHC_MISSIONS_DIR and injects them into autodream memory.

// ingestMissionArtifacts processes Markdown files from the directory specified
// by OHC_MISSIONS_DIR. GitHub issues are the canonical task tracker, so file
// ingestion is disabled unless the env var is explicitly configured.


// SetLLMClient configures the LLM client to use for embeddings.

// MemoryFile represents the structure of agent memory YAML files.
type MemoryFile struct {
	AgentSessionData string `yaml:"agent_session_data"`
	Content          string `yaml:"content"`
}

// ProcessMemories ingests pending memory YAML files into the autodream_memories table.
//
// When OHC_MEMORY_DIR is set the worker reads YAML files from that directory
// (migration path: legacy agents wrote memory files there).  If the env var is
// empty this pipeline is a no-op — new agents write directly to the DB.

// IngestCompletedTasks fetches COMPLETED tasks from shared_tasks and swarm_tasks, embeds them, and stores them in autodream_memories.


// SearchMemories queries the autodream_memories vector database.



// ConsolidateMemories processes the backlog of episodic memories into embeddings.
// It fetches up to 100 rows where processed_at IS NULL.



func formatFloat32SliceForVector(embedding []float32) string {
	if len(embedding) == 0 {
		return "[]"
	}
	strs := make([]string, len(embedding))
	for i, v := range embedding {
		strs[i] = fmt.Sprintf("%f", v)
	}
	return "[" + strings.Join(strs, ",") + "]"
}
