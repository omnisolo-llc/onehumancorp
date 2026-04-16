package local

import (
	"context"
	"errors"
	"fmt"
	"io/fs"
	"net/http"
	"os"
	"github.com/onehumancorp/mono/srcs/server/utils"
	"os/exec"
	"github.com/onehumancorp/mono/srcs/server/agents/harness"
	"path/filepath"
	"regexp"
	"strings"
	"time"
)

// Tool is a named callable that the agent can invoke.
type Tool interface {
	Definition() ToolDefinition
	Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error)
}

// DefaultTools returns the standard tool set available to a local agent.
// It mirrors the ASYNC_AGENT_ALLOWED_TOOLS from CC-Source.
func DefaultTools() []Tool {
	return []Tool{
		&bashTool{},
		&fileReadTool{},
		&fileWriteTool{},
		&fileEditTool{},
		&grepTool{},
		&globTool{},
		&webFetchTool{hc: &http.Client{Timeout: 30 * time.Second}},
		&todoTool{},
	}
}

func strArg(input map[string]interface{}, key string) string {
	if v, ok := input[key]; ok {
		if s, ok := v.(string); ok {
			return s
		}
	}
	return ""
}

func intArg(input map[string]interface{}, key string, def int) int {
	if v, ok := input[key]; ok {
		switch n := v.(type) {
		case int:
			return n
		case float64:
			return int(n)
		}
	}
	return def
}

// ─── BashTool ─────────────────────────────────────────────────────────────────

type bashTool struct{}

func (t *bashTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name:        "bash",
		Description: "Execute a bash command in a shell. Use for running programs, scripts, build commands, or any shell operation. Commands run synchronously and output is returned.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"command": map[string]interface{}{
					"type":        "string",
					"description": "The bash command to execute.",
				},
				"timeout": map[string]interface{}{
					"type":        "integer",
					"description": "Optional timeout in seconds (default 120).",
				},
			},
			"required": []string{"command"},
		},
	}
}

func (t *bashTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	command := strArg(input, "command")
	if command == "" {
		return "", errors.New("bash: command is required")
	}
	timeoutSec := intArg(input, "timeout", 120)
	timeoutDur := time.Duration(timeoutSec) * time.Second

	execCtx, cancel := context.WithTimeout(ctx, timeoutDur)
	defer cancel()

	if err := harness.GlobalInterceptor.Intercept(ctx, command); err != nil {
			return "", err
		}

		cmd := exec.CommandContext(execCtx, "bash", "-c", command)
	cmd.Dir = workDir
	out, err := cmd.CombinedOutput()
	if err != nil {
		if errors.Is(execCtx.Err(), context.DeadlineExceeded) {
			return string(out), fmt.Errorf("bash: command timed out after %ds", timeoutSec)
		}
		// Return output along with error so the model can see what happened.
		return string(out), fmt.Errorf("bash: exit status %w", err)
	}
	return string(out), nil
}

// ─── FileReadTool ─────────────────────────────────────────────────────────────

type fileReadTool struct{}

func (t *fileReadTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name:        "file_read",
		Description: "Read the contents of a file. Returns the file content as a string. Supports offset and limit for large files.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"path": map[string]interface{}{
					"type":        "string",
					"description": "Absolute or relative path to the file.",
				},
				"offset": map[string]interface{}{
					"type":        "integer",
					"description": "Line offset to start reading from (1-based, optional).",
				},
				"limit": map[string]interface{}{
					"type":        "integer",
					"description": "Maximum number of lines to read (optional).",
				},
			},
			"required": []string{"path"},
		},
	}
}

func (t *fileReadTool) Execute(_ context.Context, workDir string, input map[string]interface{}) (string, error) {
	path := strArg(input, "path")
	if path == "" {
		return "", errors.New("file_read: path is required")
	}
	if !filepath.IsAbs(path) {
		path = filepath.Join(workDir, path)
	}
	data, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("file_read: %w", err)
	}

	content := string(data)
	offset := intArg(input, "offset", 0)
	limit := intArg(input, "limit", 0)
	if offset > 0 || limit > 0 {
		lines := strings.Split(content, "\n")
		start := 0
		if offset > 1 {
			start = offset - 1
		}
		if start >= len(lines) {
			return "", nil
		}
		lines = lines[start:]
		if limit > 0 && len(lines) > limit {
			lines = lines[:limit]
		}
		content = strings.Join(lines, "\n")
	}
	return content, nil
}

// ─── FileWriteTool ────────────────────────────────────────────────────────────

type fileWriteTool struct{}

func (t *fileWriteTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name:        "file_write",
		Description: "Create or overwrite a file with the provided content. Parent directories are created automatically.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"path": map[string]interface{}{
					"type":        "string",
					"description": "Absolute or relative path to the file.",
				},
				"content": map[string]interface{}{
					"type":        "string",
					"description": "The full content to write to the file.",
				},
			},
			"required": []string{"path", "content"},
		},
	}
}

func (t *fileWriteTool) Execute(_ context.Context, workDir string, input map[string]interface{}) (string, error) {
	path := strArg(input, "path")
	if path == "" {
		return "", errors.New("file_write: path is required")
	}
	if !filepath.IsAbs(path) {
		path = filepath.Join(workDir, path)
	}
	content := strArg(input, "content")

	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return "", fmt.Errorf("file_write: mkdir: %w", err)
	}
	if err := utils.WriteFileAtomic(path, []byte(content), 0o644); err != nil {
		return "", fmt.Errorf("file_write: %w", err)
	}
	return fmt.Sprintf("File written: %s (%d bytes)", path, len(content)), nil
}

// ─── FileEditTool ─────────────────────────────────────────────────────────────

// fileEditTool implements the str_replace_editor pattern used in CC-Source.
type fileEditTool struct{}

func (t *fileEditTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name:        "file_edit",
		Description: "Edit a file by replacing an exact string with new content. The old_str must match exactly (including whitespace). Use file_read first to see the current content.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"path": map[string]interface{}{
					"type":        "string",
					"description": "Absolute or relative path to the file.",
				},
				"old_str": map[string]interface{}{
					"type":        "string",
					"description": "The exact string to replace. Must match exactly once in the file.",
				},
				"new_str": map[string]interface{}{
					"type":        "string",
					"description": "The replacement string.",
				},
			},
			"required": []string{"path", "old_str", "new_str"},
		},
	}
}

func (t *fileEditTool) Execute(_ context.Context, workDir string, input map[string]interface{}) (string, error) {
	path := strArg(input, "path")
	if path == "" {
		return "", errors.New("file_edit: path is required")
	}
	if !filepath.IsAbs(path) {
		path = filepath.Join(workDir, path)
	}
	oldStr := strArg(input, "old_str")
	newStr := strArg(input, "new_str")

	data, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("file_edit: read %s: %w", path, err)
	}
	content := string(data)

	count := strings.Count(content, oldStr)
	if count == 0 {
		return "", fmt.Errorf("file_edit: old_str not found in %s", path)
	}
	if count > 1 {
		return "", fmt.Errorf("file_edit: old_str found %d times in %s; it must match exactly once", count, path)
	}

	updated := strings.Replace(content, oldStr, newStr, 1)
	if err := utils.WriteFileAtomic(path, []byte(updated), 0o644); err != nil {
		return "", fmt.Errorf("file_edit: write %s: %w", path, err)
	}
	return fmt.Sprintf("File edited: %s", path), nil
}

// ─── GrepTool ─────────────────────────────────────────────────────────────────

type grepTool struct{}

func (t *grepTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name:        "grep",
		Description: "Search for a regex pattern in files. Returns matching lines with file and line number. Uses ripgrep when available, otherwise falls back to Go's regexp package.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"pattern": map[string]interface{}{
					"type":        "string",
					"description": "Regular expression pattern to search for.",
				},
				"path": map[string]interface{}{
					"type":        "string",
					"description": "File or directory to search in. Defaults to current working directory.",
				},
				"glob": map[string]interface{}{
					"type":        "string",
					"description": "Glob pattern to filter files (e.g. '*.go', '*.{ts,tsx}').",
				},
				"case_insensitive": map[string]interface{}{
					"type":        "boolean",
					"description": "Case-insensitive search (default false).",
				},
				"max_results": map[string]interface{}{
					"type":        "integer",
					"description": "Maximum number of results (default 100).",
				},
			},
			"required": []string{"pattern"},
		},
	}
}

func (t *grepTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	pattern := strArg(input, "pattern")
	if pattern == "" {
		return "", errors.New("grep: pattern is required")
	}
	searchPath := strArg(input, "path")
	if searchPath == "" {
		searchPath = workDir
	} else if !filepath.IsAbs(searchPath) {
		searchPath = filepath.Join(workDir, searchPath)
	}

	// Try ripgrep first.
	rgPath, err := exec.LookPath("rg")
	if err == nil {
		return t.runRipgrep(ctx, rgPath, pattern, searchPath, input)
	}
	return t.runGoGrep(pattern, searchPath, input)
}

func (t *grepTool) runRipgrep(ctx context.Context, rgPath, pattern, searchPath string, input map[string]interface{}) (string, error) {
	args := []string{"--line-number", "--with-filename"}
	if v, ok := input["case_insensitive"]; ok {
		if b, ok := v.(bool); ok && b {
			args = append(args, "--ignore-case")
		}
	}
	if g := strArg(input, "glob"); g != "" {
		args = append(args, "--glob", g)
	}
	max := intArg(input, "max_results", 100)
	args = append(args, "--max-count", fmt.Sprint(max))
	args = append(args, pattern, searchPath)

	cmd := exec.CommandContext(ctx, rgPath, args...)
	out, err := cmd.Output()
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok && exitErr.ExitCode() == 1 {
			return "No matches found.", nil
		}
		return "", fmt.Errorf("grep (rg): %w", err)
	}
	return string(out), nil
}

func (t *grepTool) runGoGrep(pattern, searchPath string, input map[string]interface{}) (string, error) {
	flags := 0
	if v, ok := input["case_insensitive"]; ok {
		if b, ok := v.(bool); ok && b {
			flags = 1 // mark as case-insensitive below
		}
	}
	var re *regexp.Regexp
	var err error
	if flags == 1 {
		re, err = regexp.Compile("(?i)" + pattern)
	} else {
		re, err = regexp.Compile(pattern)
	}
	if err != nil {
		return "", fmt.Errorf("grep: invalid pattern: %w", err)
	}

	globPat := strArg(input, "glob")
	max := intArg(input, "max_results", 100)
	var results []string

	err = filepath.WalkDir(searchPath, func(p string, d fs.DirEntry, walkErr error) error {
		if walkErr != nil || d.IsDir() {
			return walkErr
		}
		if len(results) >= max {
			return filepath.SkipAll
		}
		if globPat != "" {
			matched, _ := filepath.Match(globPat, filepath.Base(p))
			if !matched {
				return nil
			}
		}
		data, err := os.ReadFile(p)
		if err != nil {
			return nil
		}
		lines := strings.Split(string(data), "\n")
		for i, line := range lines {
			if re.MatchString(line) {
				results = append(results, fmt.Sprintf("%s:%d:%s", p, i+1, line))
				if len(results) >= max {
					return filepath.SkipAll
				}
			}
		}
		return nil
	})
	if err != nil && !errors.Is(err, filepath.SkipAll) {
		return "", fmt.Errorf("grep: walk: %w", err)
	}
	if len(results) == 0 {
		return "No matches found.", nil
	}
	return strings.Join(results, "\n"), nil
}

// ─── GlobTool ─────────────────────────────────────────────────────────────────

type globTool struct{}

func (t *globTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name:        "glob",
		Description: "Find files matching a glob pattern. Returns a list of matching file paths.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"pattern": map[string]interface{}{
					"type":        "string",
					"description": "Glob pattern to match files (e.g. '**/*.go', 'src/**/*.ts').",
				},
				"path": map[string]interface{}{
					"type":        "string",
					"description": "Directory to search in. Defaults to current working directory.",
				},
			},
			"required": []string{"pattern"},
		},
	}
}

func (t *globTool) Execute(_ context.Context, workDir string, input map[string]interface{}) (string, error) {
	pattern := strArg(input, "pattern")
	if pattern == "" {
		return "", errors.New("glob: pattern is required")
	}
	searchPath := strArg(input, "path")
	if searchPath == "" {
		searchPath = workDir
	} else if !filepath.IsAbs(searchPath) {
		searchPath = filepath.Join(workDir, searchPath)
	}

	// If pattern is a simple glob (no path sep), walk and match basename.
	// If pattern contains path separator, join with searchPath.
	fullPattern := filepath.Join(searchPath, pattern)

	var matches []string
	// Use filepath.Walk for double-star patterns that filepath.Glob doesn't handle.
	if strings.Contains(pattern, "**") {
		base := searchPath
		innerPat := pattern
		_ = filepath.WalkDir(base, func(p string, d fs.DirEntry, err error) error {
			if err != nil || d.IsDir() {
				return err
			}
			rel, _ := filepath.Rel(base, p)
			ok, _ := doubleStarMatch(innerPat, rel)
			if ok {
				matches = append(matches, p)
			}
			if len(matches) >= 500 {
				return filepath.SkipAll
			}
			return nil
		})
	} else {
		ms, err := filepath.Glob(fullPattern)
		if err != nil {
			return "", fmt.Errorf("glob: %w", err)
		}
		matches = ms
	}

	if len(matches) == 0 {
		return "No files found.", nil
	}
	return strings.Join(matches, "\n"), nil
}

// doubleStarMatch is a simplified double-star glob matcher.
func doubleStarMatch(pattern, name string) (bool, error) {
	// Convert ** to match any path segment by replacing with a temporary marker.
	// We use filepath.Match segment by segment.
	patParts := strings.Split(pattern, "/")
	nameParts := strings.Split(name, string(filepath.Separator))
	return matchParts(patParts, nameParts), nil
}

func matchParts(pattern, name []string) bool {
	if len(pattern) == 0 {
		return len(name) == 0
	}
	if pattern[0] == "**" {
		// ** matches zero or more path components
		for i := 0; i <= len(name); i++ {
			if matchParts(pattern[1:], name[i:]) {
				return true
			}
		}
		return false
	}
	if len(name) == 0 {
		return false
	}
	ok, _ := filepath.Match(pattern[0], name[0])
	return ok && matchParts(pattern[1:], name[1:])
}

// ─── WebFetchTool ─────────────────────────────────────────────────────────────

type webFetchTool struct {
	hc *http.Client
}

func (t *webFetchTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name:        "web_fetch",
		Description: "Fetch the content of a URL. Returns the response body as a string (up to 500 KB).",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"url": map[string]interface{}{
					"type":        "string",
					"description": "The URL to fetch.",
				},
				"method": map[string]interface{}{
					"type":        "string",
					"description": "HTTP method (default GET).",
				},
			},
			"required": []string{"url"},
		},
	}
}

func (t *webFetchTool) Execute(ctx context.Context, _ string, input map[string]interface{}) (string, error) {
	url := strArg(input, "url")
	if url == "" {
		return "", errors.New("web_fetch: url is required")
	}
	method := strArg(input, "method")
	if method == "" {
		method = http.MethodGet
	}

	req, err := http.NewRequestWithContext(ctx, method, url, nil)
	if err != nil {
		return "", fmt.Errorf("web_fetch: build request: %w", err)
	}
	req.Header.Set("User-Agent", "OHC-LocalAgent/1.0")

	resp, err := t.hc.Do(req)
	if err != nil {
		return "", fmt.Errorf("web_fetch: http: %w", err)
	}
	defer resp.Body.Close()

	const maxBody = 500 * 1024
	buf := make([]byte, maxBody)
	n, _ := resp.Body.Read(buf)
	body := string(buf[:n])

	return fmt.Sprintf("HTTP %d\n\n%s", resp.StatusCode, body), nil
}

// ─── TodoTool ─────────────────────────────────────────────────────────────────

// todoTool manages a simple in-memory todo list shared across the agent's lifetime.
type todoTool struct{}

func (t *todoTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name:        "todo_write",
		Description: "Write or update the task todo list. The list is replaced entirely with the provided items.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"todos": map[string]interface{}{
					"type": "array",
					"items": map[string]interface{}{
						"type": "object",
						"properties": map[string]interface{}{
							"id":      map[string]interface{}{"type": "string"},
							"content": map[string]interface{}{"type": "string"},
							"status":  map[string]interface{}{"type": "string", "enum": []string{"pending", "in_progress", "completed"}},
							"priority": map[string]interface{}{"type": "string", "enum": []string{"high", "medium", "low"}},
						},
						"required": []string{"id", "content", "status"},
					},
					"description": "The full list of todo items.",
				},
			},
			"required": []string{"todos"},
		},
	}
}

func (t *todoTool) Execute(_ context.Context, _ string, input map[string]interface{}) (string, error) {
	todos, ok := input["todos"]
	if !ok {
		return "", errors.New("todo_write: todos is required")
	}
	// Accept whatever was passed; format as string summary.
	switch v := todos.(type) {
	case []interface{}:
		lines := make([]string, 0, len(v))
		for _, item := range v {
			if m, ok := item.(map[string]interface{}); ok {
				status := fmt.Sprintf("[%s]", m["status"])
				content := fmt.Sprint(m["content"])
				lines = append(lines, status+" "+content)
			}
		}
		return fmt.Sprintf("Todo list updated (%d items):\n%s", len(lines), strings.Join(lines, "\n")), nil
	default:
		return "Todo list updated.", nil
	}
}
