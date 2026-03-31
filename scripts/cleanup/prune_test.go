package cleanup

import (
	"os"
	"path/filepath"
	"testing"
)

func TestCleanupLogic(t *testing.T) {
	tempDir := t.TempDir()

	// 1. Create a mock .agent-task structure
	agentTaskDir := filepath.Join(tempDir, ".agent-task")
	os.MkdirAll(filepath.Join(agentTaskDir, "missions"), 0755)
	os.MkdirAll(filepath.Join(agentTaskDir, "status"), 0755)
	os.MkdirAll(filepath.Join(agentTaskDir, "memory"), 0755)

	// Mock valid to keep and valid to delete missions
	keepMission := filepath.Join(agentTaskDir, "missions", "keep.yml")
	deleteMission := filepath.Join(agentTaskDir, "missions", "delete.yml")
	os.WriteFile(keepMission, []byte("status: PENDING\n"), 0644)
	os.WriteFile(deleteMission, []byte("status: COMPLETED\n"), 0644)

	// Mock invalid/not yaml files
	notYaml := filepath.Join(agentTaskDir, "missions", "not.txt")
	os.WriteFile(notYaml, []byte("status: COMPLETED\n"), 0644)

	badYaml := filepath.Join(agentTaskDir, "missions", "bad.yml")
	os.WriteFile(badYaml, []byte("status: { foo: "), 0644) // invalid yaml

	// Mock status
	keepStatus := filepath.Join(agentTaskDir, "status", "keep.yml")
	deleteStatus := filepath.Join(agentTaskDir, "status", "delete.yml")
	os.WriteFile(keepStatus, []byte("status: ACTIVE\n"), 0644)
	os.WriteFile(deleteStatus, []byte("status: RUNNING\n"), 0644)

	// Mock memory
	keepMemory := filepath.Join(agentTaskDir, "memory", "keep.yml")
	deleteMemory := filepath.Join(agentTaskDir, "memory", "delete.yml")
	os.WriteFile(keepMemory, []byte("context: User asked for something\n"), 0644)
	os.WriteFile(deleteMemory, []byte("context: Checked queue for missions, none were found. Queue is empty. Heartbeat recorded.\n"), 0644)

	deleteMemory2 := filepath.Join(agentTaskDir, "memory", "delete2.yml")
	os.WriteFile(deleteMemory2, []byte("content: Queue is empty.\n"), 0644)

	// Mock temp files
	tempGo := filepath.Join(tempDir, "temp.pb.go")
	os.WriteFile(tempGo, []byte("package main\n"), 0644)

	tempPy := filepath.Join(tempDir, "temp_pb2.py")
	os.WriteFile(tempPy, []byte("import sys\n"), 0644)

	tempPatch := filepath.Join(tempDir, "temp.patch")
	os.WriteFile(tempPatch, []byte("diff --git\n"), 0644)

	tempDiff := filepath.Join(tempDir, "temp.diff")
	os.WriteFile(tempDiff, []byte("diff --git\n"), 0644)

	tempTs := filepath.Join(tempDir, "temp.pb.ts")
	os.WriteFile(tempTs, []byte("export const\n"), 0644)

	// Create dummy sqlite db to test prune (it shouldn't error)
	dbPath := filepath.Join(agentTaskDir, "swarm.db")
	os.WriteFile(dbPath, []byte("dummy_sql_data_not_valid"), 0644) // just test existence checking doesn't panic

	// Run logic
	if err := RunCleanup(agentTaskDir, tempDir); err != nil {
		t.Fatalf("RunCleanup failed: %v", err)
	}

	// Verify what was kept and what was deleted
	if _, err := os.Stat(keepMission); os.IsNotExist(err) {
		t.Errorf("Expected keepMission to exist")
	}
	if _, err := os.Stat(deleteMission); err == nil {
		t.Errorf("Expected deleteMission to be deleted")
	}

	if _, err := os.Stat(notYaml); os.IsNotExist(err) {
		t.Errorf("Expected notYaml to exist")
	}

	if _, err := os.Stat(badYaml); os.IsNotExist(err) {
		t.Errorf("Expected badYaml to exist")
	}

	if _, err := os.Stat(keepStatus); os.IsNotExist(err) {
		t.Errorf("Expected keepStatus to exist")
	}
	if _, err := os.Stat(deleteStatus); err == nil {
		t.Errorf("Expected deleteStatus to be deleted")
	}

	if _, err := os.Stat(keepMemory); os.IsNotExist(err) {
		t.Errorf("Expected keepMemory to exist")
	}
	if _, err := os.Stat(deleteMemory); err == nil {
		t.Errorf("Expected deleteMemory to be deleted")
	}
	if _, err := os.Stat(deleteMemory2); err == nil {
		t.Errorf("Expected deleteMemory2 to be deleted")
	}

	if _, err := os.Stat(tempGo); err == nil {
		t.Errorf("Expected tempGo to be deleted")
	}
	if _, err := os.Stat(tempPy); err == nil {
		t.Errorf("Expected tempPy to be deleted")
	}
	if _, err := os.Stat(tempPatch); err == nil {
		t.Errorf("Expected tempPatch to be deleted")
	}
	if _, err := os.Stat(tempDiff); err == nil {
		t.Errorf("Expected tempDiff to be deleted")
	}
	if _, err := os.Stat(tempTs); err == nil {
		t.Errorf("Expected tempTs to be deleted")
	}
}

func TestCleanupLogic_EdgeCases(t *testing.T) {
	tempDir := t.TempDir()

	// 1. Missing dirs
	agentTaskDir := filepath.Join(tempDir, ".agent-task")
	if err := RunCleanup(agentTaskDir, tempDir); err != nil {
		t.Fatalf("RunCleanup failed on missing dirs: %v", err)
	}

	// 2. Dir with permission issue or unreadable file
	os.MkdirAll(filepath.Join(agentTaskDir, "missions"), 0755)
	unreadable := filepath.Join(agentTaskDir, "missions", "unreadable.yml")
	os.WriteFile(unreadable, []byte("status: DONE\n"), 0000)

	_ = RunCleanup(agentTaskDir, tempDir)

	// We restore permissions so temp dir can be cleaned up
	os.Chmod(unreadable, 0644)
}

func TestIsObsoleteMission(t *testing.T) {
	data := make(map[string]interface{})
	if isObsoleteMission(data) != false {
		t.Errorf("expected false for missing status")
	}

	data["status"] = "proposed"
	if isObsoleteMission(data) != true {
		t.Errorf("expected true for proposed")
	}

	data["status"] = "PENDING"
	if isObsoleteMission(data) != false {
		t.Errorf("expected false for pending")
	}
}

func TestIsObsoleteStatus(t *testing.T) {
	data := make(map[string]interface{})
	if isObsoleteStatus(data) != false {
		t.Errorf("expected false for missing status")
	}

	data["status"] = "done"
	if isObsoleteStatus(data) != true {
		t.Errorf("expected true for done")
	}
}

func TestIsObsoleteMemory(t *testing.T) {
	data := make(map[string]interface{})
	data["context"] = "Queue is empty"
	if isObsoleteMemory(data) != true {
		t.Errorf("expected true for Queue is empty")
	}

	data2 := make(map[string]interface{})
	data2["content"] = "Queue is empty"
	if isObsoleteMemory(data2) != true {
		t.Errorf("expected true for Queue is empty in content")
	}
}

func TestIsObsoleteMission_More(t *testing.T) {
	data := make(map[string]interface{})
	data["status"] = 123
	if isObsoleteMission(data) != false {
		t.Errorf("expected false for bad status type")
	}
}

func TestIsObsoleteStatus_More(t *testing.T) {
	data := make(map[string]interface{})
	data["status"] = 123
	if isObsoleteStatus(data) != false {
		t.Errorf("expected false for bad status type")
	}

	data["status"] = "PENDING"
	if isObsoleteStatus(data) != false {
		t.Errorf("expected false for PENDING")
	}
}

func TestIsObsoleteMemory_More(t *testing.T) {
	data := make(map[string]interface{})
	if isObsoleteMemory(data) != false {
		t.Errorf("expected false for empty")
	}
}

func TestCleanupTempFiles_SkipDirs(t *testing.T) {
	tempDir := t.TempDir()
	os.MkdirAll(filepath.Join(tempDir, ".git"), 0755)
	os.WriteFile(filepath.Join(tempDir, ".git", "temp.sh"), []byte("echo hi\n"), 0644)

	os.MkdirAll(filepath.Join(tempDir, "bazel-out"), 0755)
	os.WriteFile(filepath.Join(tempDir, "bazel-out", "temp.pb.go"), []byte("package main\n"), 0644)

	os.MkdirAll(filepath.Join(tempDir, "bazel-testlogs"), 0755)
	os.WriteFile(filepath.Join(tempDir, "bazel-testlogs", "temp.diff"), []byte("diff\n"), 0644)

	cleanupTempFiles(tempDir)

	if _, err := os.Stat(filepath.Join(tempDir, ".git", "temp.sh")); os.IsNotExist(err) {
		t.Errorf("Expected temp file in .git to be ignored")
	}
	if _, err := os.Stat(filepath.Join(tempDir, "bazel-out", "temp.pb.go")); os.IsNotExist(err) {
		t.Errorf("Expected temp file in bazel-out to be ignored")
	}
	if _, err := os.Stat(filepath.Join(tempDir, "bazel-testlogs", "temp.diff")); os.IsNotExist(err) {
		t.Errorf("Expected temp file in bazel-testlogs to be ignored")
	}
}

func TestRunCleanupErrors(t *testing.T) {
	tempDir := t.TempDir()

	// 1. Missing dirs should trigger err logs in RunCleanup
	agentTaskDir := filepath.Join(tempDir, ".agent-task")
	// RunCleanup with missing dir

	// Create unreadable dirs for missions, status, memory
	os.MkdirAll(filepath.Join(agentTaskDir, "missions"), 0000)
	os.MkdirAll(filepath.Join(agentTaskDir, "status"), 0000)
	os.MkdirAll(filepath.Join(agentTaskDir, "memory"), 0000)

	_ = RunCleanup(agentTaskDir, tempDir)

	os.Chmod(filepath.Join(agentTaskDir, "missions"), 0755)
	os.Chmod(filepath.Join(agentTaskDir, "status"), 0755)
	os.Chmod(filepath.Join(agentTaskDir, "memory"), 0755)

	// test cleanupTempFiles err
	unreadableRoot := filepath.Join(tempDir, "unreadable_root")
	os.MkdirAll(unreadableRoot, 0000)
	cleanupTempFiles(unreadableRoot)
	os.Chmod(unreadableRoot, 0755)
}

func TestErrors(t *testing.T) {
	tempDir := t.TempDir()

	// 1. Missing dirs
	agentTaskDir := filepath.Join(tempDir, ".agent-task")
	// RunCleanup with no dir
	_ = RunCleanup(agentTaskDir, tempDir)

	// create bad yaml
	os.MkdirAll(filepath.Join(agentTaskDir, "missions"), 0755)
	os.WriteFile(filepath.Join(agentTaskDir, "missions", "test.yml"), []byte("bad: ["), 0644)

	_ = pruneDir(filepath.Join(agentTaskDir, "missions"), isObsoleteMission)

	// test unreadable dir
	unreadableDir := filepath.Join(tempDir, "unreadable_dir")
	os.MkdirAll(unreadableDir, 0000)
	_ = pruneDir(unreadableDir, isObsoleteMission)
	os.Chmod(unreadableDir, 0755)
}
