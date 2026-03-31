package cleanup

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"gopkg.in/yaml.v3"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// RunCleanup prunes obsolete missions, status, and memory from the agent-task dir,
// cleans up the swarm.db, and deletes temporary/generated files from the given root dir.
func RunCleanup(agentTaskDir, rootDir string) error {
	if err := pruneDir(filepath.Join(agentTaskDir, "missions"), isObsoleteMission); err != nil {
		fmt.Printf("Error pruning missions: %v\n", err)
	}
	if err := pruneDir(filepath.Join(agentTaskDir, "status"), isObsoleteStatus); err != nil {
		fmt.Printf("Error pruning status: %v\n", err)
	}
	if err := pruneDir(filepath.Join(agentTaskDir, "memory"), isObsoleteMemory); err != nil {
		fmt.Printf("Error pruning memory: %v\n", err)
	}

	// Clean up swarm.db
	dbPath := filepath.Join(agentTaskDir, "swarm.db")
	if _, err := os.Stat(dbPath); err == nil {
		if sipdb, err := orchestration.NewSIPDB(dbPath); err == nil {
			defer sipdb.Close()
			ctx := context.Background()
			// Prune stale missions in db older than 0 seconds (so basically all completed)
			if err := sipdb.PruneStaleMissions(ctx, 0); err != nil {
				fmt.Printf("Error pruning swarm.db: %v\n", err)
			}
		}
	}

	cleanupTempFiles(rootDir)
	return nil
}

func pruneDir(dir string, isObsolete func(map[string]interface{}) bool) error {
	if _, err := os.Stat(dir); os.IsNotExist(err) {
		return nil
	}
	return filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			return nil
		}
		if !strings.HasSuffix(d.Name(), ".yml") && !strings.HasSuffix(d.Name(), ".yaml") {
			return nil
		}
		content, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		var data map[string]interface{}
		if err := yaml.Unmarshal(content, &data); err != nil {
			return nil
		}

		if isObsolete(data) {
			fmt.Printf("Deleting obsolete file: %s\n", path)
			return os.Remove(path)
		}
		return nil
	})
}

func isObsoleteMission(data map[string]interface{}) bool {
	status, ok := data["status"].(string)
	if !ok {
		return false
	}
	status = strings.ToUpper(status)
	return status == "DONE" || status == "COMPLETED" || status == "PROPOSED"
}

func isObsoleteStatus(data map[string]interface{}) bool {
	status, ok := data["status"].(string)
	if !ok {
		return false
	}
	status = strings.ToUpper(status)
	if status == "DONE" || status == "COMPLETED" || status == "RUNNING" {
		return true
	}
	return false
}

func isObsoleteMemory(data map[string]interface{}) bool {
	ctx, ok := data["context"].(string)
	if ok && strings.Contains(ctx, "Queue is empty") {
		return true
	}
	content, ok := data["content"].(string)
	if ok && strings.Contains(content, "Queue is empty") {
		return true
	}
	return false
}

func cleanupTempFiles(root string) {
	filepath.WalkDir(root, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if d.IsDir() {
			if d.Name() == ".git" || d.Name() == ".agent-task" || strings.HasPrefix(d.Name(), "bazel-") {
				return filepath.SkipDir
			}
			return nil
		}
		name := d.Name()
		// Only delete generated pb files and patch/diff
		if strings.HasSuffix(name, ".pb.go") || strings.HasSuffix(name, ".pb.ts") || strings.HasSuffix(name, "_pb2.py") || strings.HasSuffix(name, ".diff") || strings.HasSuffix(name, ".patch") {
			fmt.Printf("Deleting generated/temp file: %s\n", path)
			os.Remove(path)
		}
		return nil
	})
}
