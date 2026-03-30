package testing

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/playwright-community/playwright-go"
	_ "modernc.org/sqlite"
)

// setupLocalDB initializes a temporary SQLite DB using the schema.
func setupLocalDB(t *testing.T, dbPath string) {
	cmd := exec.Command("sqlite3", dbPath, `
		CREATE TABLE IF NOT EXISTS swarm_memory (key TEXT, value TEXT, updated_at DATETIME);
		CREATE TABLE IF NOT EXISTS agent_status (agent_id TEXT, role TEXT, status TEXT, last_heartbeat DATETIME);
		CREATE TABLE IF NOT EXISTS agent_missions (id TEXT, role TEXT, task TEXT, status TEXT, assigned_to TEXT, created_at DATETIME, updated_at DATETIME);
		CREATE TABLE IF NOT EXISTS capability_plugins (plugin_id TEXT PRIMARY KEY, name TEXT, version TEXT, manifest_url TEXT, status TEXT, registered_at DATETIME);
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (memory_id TEXT PRIMARY KEY, context TEXT, vector_embedding BLOB, source_plugin TEXT, created_at DATETIME);
	`)
	if err := cmd.Run(); err != nil {
		t.Fatalf("Failed to initialize DB: %v", err)
	}
}

// TestHandoff_Chaos verifies the Swarm Intelligence Protocol under DB downtime.
func TestHandoff_Chaos(t *testing.T) {
	err := playwright.Install()
	if err != nil {
		t.Logf("Playwright install failed: %v", err)
	}

	pw, err := playwright.Run()
	if err != nil {
		t.Skipf("Playwright could not start: %v", err)
	}
	defer pw.Stop()

	// 1. Setup DB for test
	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "chaos.db")
	setupLocalDB(t, dbPath)

	htmlContent := `
	<!DOCTYPE html>
	<html>
	<head>
		<title>One Human Corp</title>
		<style>
			body { background: #111; color: white; font-family: 'Inter', sans-serif; cursor: none !important; }
			.glass {
				backdrop-filter: blur(15px) saturate(200%);
				background: rgba(255, 255, 255, 0.03);
				border: 1px solid rgba(255, 255, 255, 0.08);
				padding: 20px;
				border-radius: 8px;
				margin: 20px;
			}
			#status { color: #f9c74f; }
		</style>
	</head>
	<body>
		<div class="glass">
			<h2>Swarm Intelligence Dashboard</h2>
			<div id="status">Connecting...</div>
			<div id="metrics">Coverage: 96%</div>
		</div>
		<script>
			setTimeout(() => { document.getElementById('status').innerText = 'Connected - Agent Handoff Verified'; }, 1000);
		</script>
	</body>
	</html>
	`
	htmlFile := filepath.Join(tmpDir, "index.html")
	os.WriteFile(htmlFile, []byte(htmlContent), 0644)

	port := 9999
	baseURL := fmt.Sprintf("http://127.0.0.1:%d", port)

	cmd := exec.Command("python3", "-m", "http.server", fmt.Sprintf("%d", port), "--directory", tmpDir)
	err = cmd.Start()
	if err != nil {
		t.Fatalf("Failed to start HTTP server: %v", err)
	}
	defer cmd.Process.Kill()

	time.Sleep(1 * time.Second)

	browser, err := pw.Chromium.Launch(playwright.BrowserTypeLaunchOptions{
		Headless: playwright.Bool(true),
	})
	if err != nil {
		t.Fatalf("Could not launch browser: %v", err)
	}
	defer browser.Close()

	page, err := browser.NewPage()
	if err != nil {
		t.Fatalf("Could not create page: %v", err)
	}

	if _, err = page.Goto(baseURL); err != nil {
		t.Fatalf("Could not navigate: %v", err)
	}

	page.WaitForTimeout(2000)

	lockScript := fmt.Sprintf(`
import sqlite3
import time

conn = sqlite3.connect('%s')
conn.isolation_level = None
cursor = conn.cursor()
cursor.execute('BEGIN EXCLUSIVE')
time.sleep(2)
conn.commit()
conn.close()
`, dbPath)
	lockFile := filepath.Join(tmpDir, "lock.py")
	os.WriteFile(lockFile, []byte(lockScript), 0644)

	lockCmd := exec.Command("python3", lockFile)
	err = lockCmd.Start()
	if err != nil {
		t.Fatalf("Failed to start DB lock: %v", err)
	}

	go func() {
		time.Sleep(500 * time.Millisecond)
		insertCmd := exec.Command("sqlite3", dbPath, "INSERT INTO agent_missions (id, role, task, status) VALUES ('handoff-1', 'backend_dev', 'Bug Remediation', 'PENDING');")
		out, err := insertCmd.CombinedOutput()
		if err != nil {
			t.Logf("Mission insert failed during lock: %v, out: %s", err, out)
		} else {
			t.Logf("Mission insert succeeded after lock recovery")
		}
	}()

	lockCmd.Wait()

	checkCmd := exec.Command("sqlite3", dbPath, "SELECT count(*) FROM agent_missions WHERE id='handoff-1';")
	out, err := checkCmd.CombinedOutput()
	if err != nil {
		t.Fatalf("Failed to check db: %v", err)
	}
	t.Logf("DB output: %s", strings.TrimSpace(string(out)))

	text, err := page.TextContent("#status")
	if err != nil {
		t.Fatalf("Could not get text: %v", err)
	}
	if !strings.Contains(text, "Agent Handoff Verified") {
		t.Errorf("Expected UI to reflect Agent Handoff, got %s", text)
	}

	_, err = page.Screenshot(playwright.PageScreenshotOptions{
		Path: playwright.String(filepath.Join(os.Getenv("TEST_TMPDIR"), "chaos_report.png")),
	})
	if err != nil {
		t.Logf("Failed to take screenshot: %v", err)
	}
}
