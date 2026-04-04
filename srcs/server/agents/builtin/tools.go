package builtin

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"io/fs"
	"net/http"
	"net/url"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"time"
)

// Tool is a named callable that the agent can invoke.
type Tool interface {
	Definition() ToolDefinition
	Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error)
}

// DefaultTools returns the standard tool set available to a builtin agent.
// It mirrors the ASYNC_AGENT_ALLOWED_TOOLS from CC-Source plus the
// IN_PROCESS_TEAMMATE_ALLOWED_TOOLS for Hub-aware runners.
func DefaultTools() []Tool {
	return []Tool{
		&bashTool{},
		&fileReadTool{},
		&fileWriteTool{},
		&fileEditTool{},
		&grepTool{},
		&globTool{},
		&lsTool{},
		&webFetchTool{hc: &http.Client{Timeout: 30 * time.Second}},
		&webSearchTool{hc: &http.Client{Timeout: 15 * time.Second}},
		&todoTool{},
		&toolSearchTool{},
	}
}

// DefaultToolsWithHub returns the full tool set including Hub-aware tools for
// in-process teammates. These tools require a Hub reference wired after
// construction via WithHubTools.
func DefaultToolsWithHub(hub Hub, agentID string) []Tool {
	base := DefaultTools()
	return append(base,
		&taskCreateTool{hub: hub, agentID: agentID},
		&taskGetTool{hub: hub, agentID: agentID},
		&taskListTool{hub: hub, agentID: agentID},
		&taskUpdateTool{hub: hub, agentID: agentID},
		&sendMessageTool{hub: hub, agentID: agentID},
	)
}

// ─── helpers ──────────────────────────────────────────────────────────────────

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

func boolArg(input map[string]interface{}, key string, def bool) bool {
	if v, ok := input[key]; ok {
		switch b := v.(type) {
		case bool:
			return b
		case string:
			return strings.EqualFold(b, "true") || b == "1"
		}
	}
	return def
}

func resolvePath(workDir, path string) string {
	if filepath.IsAbs(path) {
		return path
	}
	return filepath.Join(workDir, path)
}

// ─── BashTool ─────────────────────────────────────────────────────────────────

type bashTool struct{}

func (t *bashTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "bash",
		Description: `Execute a bash command in a shell. Use for running programs, scripts,
build commands, tests, or any shell operation. Commands run synchronously and
combined stdout+stderr is returned. Prefer specific targeted commands over broad
ones. Long-running commands support an optional timeout.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"command": map[string]interface{}{
					"type":        "string",
					"description": "The bash command to execute.",
				},
				"timeout": map[string]interface{}{
					"type":        "integer",
					"description": "Optional timeout in seconds (default 120, max 600).",
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
	if timeoutSec > 600 {
		timeoutSec = 600
	}
	timeoutDur := time.Duration(timeoutSec) * time.Second

	execCtx, cancel := context.WithTimeout(ctx, timeoutDur)
	defer cancel()

	cmd := exec.CommandContext(execCtx, "bash", "-c", command)
	cmd.Dir = workDir
	out, err := cmd.CombinedOutput()
	if err != nil {
		if errors.Is(execCtx.Err(), context.DeadlineExceeded) {
			return string(out), fmt.Errorf("bash: command timed out after %ds", timeoutSec)
		}
		return string(out), fmt.Errorf("bash: exit status %w", err)
	}
	return string(out), nil
}

// ─── FileReadTool ─────────────────────────────────────────────────────────────

type fileReadTool struct{}

func (t *fileReadTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "file_read",
		Description: `Read the contents of a file. Returns the file content with line numbers
prefixed in the format "N. " (e.g. "1. first line"). Supports offset and limit
for large files. Use offset+limit for targeted reading of specific sections.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"path": map[string]interface{}{
					"type":        "string",
					"description": "Absolute or relative path to the file.",
				},
				"offset": map[string]interface{}{
					"type":        "integer",
					"description": "Line number to start reading from (1-based, optional).",
				},
				"limit": map[string]interface{}{
					"type":        "integer",
					"description": "Maximum number of lines to read (optional, default unlimited).",
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
	path = resolvePath(workDir, path)

	data, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("file_read: %w", err)
	}

	lines := strings.Split(string(data), "\n")
	offset := intArg(input, "offset", 0)
	limit := intArg(input, "limit", 0)

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

	// Prefix each line with its line number.
	lineNum := start + 1
	var sb strings.Builder
	for _, line := range lines {
		sb.WriteString(fmt.Sprintf("%d. %s\n", lineNum, line))
		lineNum++
	}
	return sb.String(), nil
}

// ─── FileWriteTool ────────────────────────────────────────────────────────────

type fileWriteTool struct{}

func (t *fileWriteTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "file_write",
		Description: `Create or overwrite a file with the provided content. Parent directories
are created automatically. Prefer file_edit for targeted edits of existing files
to avoid accidentally overwriting unrelated content.`,
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
	content := strArg(input, "content")
	path = resolvePath(workDir, path)

	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return "", fmt.Errorf("file_write: mkdir: %w", err)
	}
	if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
		return "", fmt.Errorf("file_write: %w", err)
	}
	return fmt.Sprintf("Written %d bytes to %s", len(content), path), nil
}

// ─── FileEditTool ─────────────────────────────────────────────────────────────

type fileEditTool struct{}

func (t *fileEditTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "file_edit",
		Description: `Edit a file by replacing exactly one occurrence of old_str with new_str.
old_str must match the file content exactly (including indentation and whitespace).
Always read the file first to confirm the exact content before editing. Returns an
error if old_str is not found or if it matches more than once.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"path": map[string]interface{}{
					"type":        "string",
					"description": "Absolute or relative path to the file.",
				},
				"old_str": map[string]interface{}{
					"type":        "string",
					"description": "The exact string to replace (must match exactly once).",
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
	oldStr := strArg(input, "old_str")
	newStr := strArg(input, "new_str")
	path = resolvePath(workDir, path)

	data, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("file_edit: read: %w", err)
	}
	content := string(data)

	count := strings.Count(content, oldStr)
	if count == 0 {
		return "", fmt.Errorf("file_edit: old_str not found in %s", path)
	}
	if count > 1 {
		return "", fmt.Errorf("file_edit: old_str found %d times in %s — must be unique; add more context to make it unique", count, path)
	}

	updated := strings.Replace(content, oldStr, newStr, 1)
	if err := os.WriteFile(path, []byte(updated), 0o644); err != nil {
		return "", fmt.Errorf("file_edit: write: %w", err)
	}

	oldLines := len(strings.Split(oldStr, "\n"))
	newLines := len(strings.Split(newStr, "\n"))
	return fmt.Sprintf("Edited %s: replaced %d line(s) with %d line(s)", path, oldLines, newLines), nil
}

// ─── GrepTool ─────────────────────────────────────────────────────────────────

type grepTool struct{}

func (t *grepTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "grep",
		Description: `Search for a regular expression pattern in file contents using ripgrep (rg)
when available, falling back to Go's regexp package. Supports multiple output
modes and context lines. Use output_mode "files_with_matches" (default) for a
quick list of matching files, "content" for matching lines with optional context,
and "count" for match counts per file.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"pattern": map[string]interface{}{
					"type":        "string",
					"description": "The regular expression pattern to search for in file contents.",
				},
				"path": map[string]interface{}{
					"type":        "string",
					"description": "File or directory to search in. Defaults to working directory.",
				},
				"glob": map[string]interface{}{
					"type":        "string",
					"description": `Glob pattern to filter files (e.g. "*.go", "*.{ts,tsx}").`,
				},
				"output_mode": map[string]interface{}{
					"type":        "string",
					"enum":        []string{"content", "files_with_matches", "count"},
					"description": `Output mode: "content" shows matching lines, "files_with_matches" shows file paths (default), "count" shows match counts per file.`,
				},
				"-i": map[string]interface{}{
					"type":        "boolean",
					"description": "Case insensitive search.",
				},
				"-n": map[string]interface{}{
					"type":        "boolean",
					"description": "Show line numbers. Only used with output_mode \"content\".",
				},
				"-A": map[string]interface{}{
					"type":        "integer",
					"description": "Lines of context after match. Only used with output_mode \"content\".",
				},
				"-B": map[string]interface{}{
					"type":        "integer",
					"description": "Lines of context before match. Only used with output_mode \"content\".",
				},
				"-C": map[string]interface{}{
					"type":        "integer",
					"description": "Lines of context before and after match. Only used with output_mode \"content\".",
				},
				"head_limit": map[string]interface{}{
					"type":        "integer",
					"description": "Limit output to first N results.",
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
	} else {
		searchPath = resolvePath(workDir, searchPath)
	}

	rgPath, rgErr := exec.LookPath("rg")
	if rgErr == nil {
		return t.runRipgrep(ctx, rgPath, pattern, searchPath, input)
	}
	return t.runGoGrep(pattern, searchPath, input)
}

func (t *grepTool) runRipgrep(ctx context.Context, rgPath, pattern, searchPath string, input map[string]interface{}) (string, error) {
	args := []string{"--no-heading"}

	outputMode := strArg(input, "output_mode")
	if outputMode == "" {
		outputMode = "files_with_matches"
	}
	switch outputMode {
	case "files_with_matches":
		args = append(args, "-l")
	case "count":
		args = append(args, "--count")
	default: // "content"
		// Default content mode — nothing special.
		if boolArg(input, "-n", false) {
			args = append(args, "-n")
		}
		contextAfter := intArg(input, "-A", 0)
		contextBefore := intArg(input, "-B", 0)
		context := intArg(input, "-C", 0)
		if context > 0 {
			args = append(args, fmt.Sprintf("-C%d", context))
		} else {
			if contextAfter > 0 {
				args = append(args, fmt.Sprintf("-A%d", contextAfter))
			}
			if contextBefore > 0 {
				args = append(args, fmt.Sprintf("-B%d", contextBefore))
			}
		}
	}

	if boolArg(input, "-i", false) {
		args = append(args, "-i")
	}
	if glob := strArg(input, "glob"); glob != "" {
		args = append(args, "--glob", glob)
	}

	args = append(args, pattern, searchPath)
	cmd := exec.CommandContext(ctx, rgPath, args...)
	out, _ := cmd.CombinedOutput()
	result := string(out)

	if headLimit := intArg(input, "head_limit", 0); headLimit > 0 {
		lines := strings.Split(result, "\n")
		if len(lines) > headLimit {
			lines = lines[:headLimit]
			result = strings.Join(lines, "\n") + "\n(truncated)"
		}
	}

	return result, nil
}

func (t *grepTool) runGoGrep(pattern, searchPath string, input map[string]interface{}) (string, error) {
	caseInsensitive := boolArg(input, "-i", false)

	reStr := pattern
	if caseInsensitive {
		reStr = "(?i)" + pattern
	}
	re, err := regexp.Compile(reStr)
	if err != nil {
		return "", fmt.Errorf("grep: invalid pattern: %w", err)
	}

	glob := strArg(input, "glob")
	outputMode := strArg(input, "output_mode")
	if outputMode == "" {
		outputMode = "files_with_matches"
	}
	headLimit := intArg(input, "head_limit", 0)

	var results []string
	err = filepath.WalkDir(searchPath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if d.IsDir() {
			if d.Name() == ".git" || d.Name() == "vendor" || d.Name() == "node_modules" {
				return filepath.SkipDir
			}
			return nil
		}
		if glob != "" {
			matched, _ := filepath.Match(glob, d.Name())
			if !matched {
				return nil
			}
		}
		data, err := os.ReadFile(path)
		if err != nil {
			return nil
		}
		lines := strings.Split(string(data), "\n")
		matched := false
		var fileResults []string
		for i, line := range lines {
			if re.MatchString(line) {
				matched = true
				switch outputMode {
				case "files_with_matches":
					// We'll add path once.
				case "count":
					// We'll count.
				default: // content
					if boolArg(input, "-n", false) {
						fileResults = append(fileResults, fmt.Sprintf("%s:%d:%s", path, i+1, line))
					} else {
						fileResults = append(fileResults, fmt.Sprintf("%s:%s", path, line))
					}
				}
			}
		}
		if matched {
			switch outputMode {
			case "files_with_matches":
				results = append(results, path)
			case "count":
				cnt := len(re.FindAllString(string(data), -1))
				results = append(results, fmt.Sprintf("%s:%d", path, cnt))
			default:
				results = append(results, fileResults...)
			}
		}
		if headLimit > 0 && len(results) >= headLimit {
			return filepath.SkipAll
		}
		return nil
	})
	if err != nil {
		return "", fmt.Errorf("grep: walk: %w", err)
	}

	if len(results) == 0 {
		return "(no matches)", nil
	}
	return strings.Join(results, "\n"), nil
}

// ─── GlobTool ─────────────────────────────────────────────────────────────────

type globTool struct{}

func (t *globTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "glob",
		Description: `Find files by name pattern using glob syntax. Supports standard wildcards:
  * matches any characters within a path segment
  ** matches any characters across multiple path segments
  ? matches a single character
  {a,b} matches either a or b
Returns matching file paths sorted alphabetically, truncated at 200 results.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"pattern": map[string]interface{}{
					"type":        "string",
					"description": "The glob pattern to match files against (e.g. \"**/*.go\", \"src/**/*.ts\").",
				},
				"path": map[string]interface{}{
					"type":        "string",
					"description": "Directory to search in. Defaults to working directory.",
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
	basePath := strArg(input, "path")
	if basePath == "" {
		basePath = workDir
	} else {
		basePath = resolvePath(workDir, basePath)
	}

	var matches []string
	const maxResults = 200

	err := filepath.WalkDir(basePath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if d.IsDir() {
			if d.Name() == ".git" || d.Name() == "vendor" || d.Name() == "node_modules" {
				return filepath.SkipDir
			}
			return nil
		}
		rel, err := filepath.Rel(basePath, path)
		if err != nil {
			rel = path
		}
		ok, err := doubleStarMatch(pattern, rel)
		if err == nil && ok {
			matches = append(matches, path)
		}
		if len(matches) >= maxResults {
			return filepath.SkipAll
		}
		return nil
	})
	if err != nil {
		return "", fmt.Errorf("glob: %w", err)
	}

	if len(matches) == 0 {
		return "(no matches)", nil
	}
	sort.Strings(matches)
	truncated := false
	if len(matches) >= maxResults {
		truncated = true
	}
	result := strings.Join(matches, "\n")
	if truncated {
		result += fmt.Sprintf("\n(truncated at %d results)", maxResults)
	}
	return result, nil
}

// doubleStarMatch supports ** patterns by normalising them into filepath.Match calls.
func doubleStarMatch(pattern, name string) (bool, error) {
	// Normalise separators.
	pattern = filepath.ToSlash(pattern)
	name = filepath.ToSlash(name)

	if !strings.Contains(pattern, "**") {
		return filepath.Match(pattern, name)
	}

	patternParts := strings.Split(pattern, "/")
	nameParts := strings.Split(name, "/")
	return matchParts(patternParts, nameParts), nil
}

func matchParts(pattern, name []string) bool {
	for len(pattern) > 0 && len(name) > 0 {
		if pattern[0] == "**" {
			if len(pattern) == 1 {
				return true
			}
			// Try matching ** with 0 to N name segments.
			for i := 0; i <= len(name); i++ {
				if matchParts(pattern[1:], name[i:]) {
					return true
				}
			}
			return false
		}
		ok, err := filepath.Match(pattern[0], name[0])
		if err != nil || !ok {
			return false
		}
		pattern = pattern[1:]
		name = name[1:]
	}
	// Both exhausted → match.
	if len(pattern) == 0 && len(name) == 0 {
		return true
	}
	// Remaining pattern is all "**" → match.
	for _, p := range pattern {
		if p != "**" {
			return false
		}
	}
	return len(name) == 0
}

// ─── LSTool ───────────────────────────────────────────────────────────────────

type lsTool struct{}

func (t *lsTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "ls",
		Description: `List files and directories at a path. Returns a formatted directory listing
showing names, sizes, and types. Use this to explore the structure of a directory
before reading files within it.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"path": map[string]interface{}{
					"type":        "string",
					"description": "Directory path to list. Defaults to working directory.",
				},
				"all": map[string]interface{}{
					"type":        "boolean",
					"description": "Include hidden files and directories (starting with '.').",
				},
			},
		},
	}
}

func (t *lsTool) Execute(_ context.Context, workDir string, input map[string]interface{}) (string, error) {
	path := strArg(input, "path")
	if path == "" {
		path = workDir
	} else {
		path = resolvePath(workDir, path)
	}
	showAll := boolArg(input, "all", false)

	entries, err := os.ReadDir(path)
	if err != nil {
		return "", fmt.Errorf("ls: %w", err)
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("%s:\n", path))
	for _, e := range entries {
		if !showAll && strings.HasPrefix(e.Name(), ".") {
			continue
		}
		info, err := e.Info()
		if err != nil {
			continue
		}
		typ := "-"
		if e.IsDir() {
			typ = "d"
		} else if e.Type()&fs.ModeSymlink != 0 {
			typ = "l"
		}
		sb.WriteString(fmt.Sprintf("%s  %8d  %s\n", typ, info.Size(), e.Name()))
	}
	return sb.String(), nil
}

// ─── WebFetchTool ─────────────────────────────────────────────────────────────

type webFetchTool struct {
	hc *http.Client
}

func (t *webFetchTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "web_fetch",
		Description: `Fetch content from a URL and return the response body. Supports GET, POST, and
other HTTP methods. Response body is truncated at 500 KB. Use this to retrieve
documentation, API responses, or any web content.`,
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
				"body": map[string]interface{}{
					"type":        "string",
					"description": "Optional request body for POST/PUT requests.",
				},
				"headers": map[string]interface{}{
					"type":                 "object",
					"additionalProperties": map[string]interface{}{"type": "string"},
					"description":          "Optional HTTP headers to include in the request.",
				},
			},
			"required": []string{"url"},
		},
	}
}

func (t *webFetchTool) Execute(ctx context.Context, _ string, input map[string]interface{}) (string, error) {
	rawURL := strArg(input, "url")
	if rawURL == "" {
		return "", errors.New("web_fetch: url is required")
	}
	method := strArg(input, "method")
	if method == "" {
		method = http.MethodGet
	}

	var bodyReader io.Reader
	if bodyStr := strArg(input, "body"); bodyStr != "" {
		bodyReader = strings.NewReader(bodyStr)
	}

	req, err := http.NewRequestWithContext(ctx, method, rawURL, bodyReader)
	if err != nil {
		return "", fmt.Errorf("web_fetch: build request: %w", err)
	}
	req.Header.Set("User-Agent", "OHC-BuiltinAgent/1.0")

	// Apply optional headers.
	if hdrs, ok := input["headers"].(map[string]interface{}); ok {
		for k, v := range hdrs {
			if s, ok := v.(string); ok {
				req.Header.Set(k, s)
			}
		}
	}

	resp, err := t.hc.Do(req)
	if err != nil {
		return "", fmt.Errorf("web_fetch: http: %w", err)
	}
	defer resp.Body.Close()

	const maxBody = 500 * 1024
	limited := io.LimitReader(resp.Body, maxBody)
	body, err := io.ReadAll(limited)
	if err != nil {
		return "", fmt.Errorf("web_fetch: read body: %w", err)
	}

	result := fmt.Sprintf("HTTP %d %s\n\n%s", resp.StatusCode, resp.Status, string(body))
	if int64(len(body)) >= maxBody {
		result += "\n\n(response truncated at 500 KB)"
	}
	return result, nil
}

// ─── WebSearchTool ────────────────────────────────────────────────────────────

// webSearchTool implements web search using the DuckDuckGo Instant Answer API
// (free, no key required). For richer results, set OHC_SEARCH_API_KEY and
// OHC_SEARCH_API_URL to point at a Tavily/Brave/Serper compatible endpoint.
type webSearchTool struct {
	hc *http.Client
}

func (t *webSearchTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "web_search",
		Description: `Search the web for information. Returns a list of relevant results with
titles, URLs, and short snippets. Use for finding documentation, API references,
recent news, or any information not available in the local codebase.
Set OHC_SEARCH_API_URL + OHC_SEARCH_API_KEY env vars for a full-text search
provider (Tavily/Brave/Serper compatible). Falls back to DuckDuckGo Instant
Answer API (no key required) which works best for factual lookups.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"query": map[string]interface{}{
					"type":        "string",
					"description": "The search query.",
				},
				"max_results": map[string]interface{}{
					"type":        "integer",
					"description": "Maximum number of results to return (default 10).",
				},
			},
			"required": []string{"query"},
		},
	}
}

// searchResult holds a single search result item for JSON encoding.
type searchResult struct {
	Title   string `json:"title"`
	URL     string `json:"url"`
	Snippet string `json:"snippet,omitempty"`
}

func (t *webSearchTool) Execute(ctx context.Context, _ string, input map[string]interface{}) (string, error) {
	query := strArg(input, "query")
	if query == "" {
		return "", errors.New("web_search: query is required")
	}
	maxResults := intArg(input, "max_results", 10)
	if maxResults <= 0 {
		maxResults = 10
	}

	// Prefer a configured full-text search provider.
	if apiURL := os.Getenv("OHC_SEARCH_API_URL"); apiURL != "" {
		return t.searchProvider(ctx, apiURL, os.Getenv("OHC_SEARCH_API_KEY"), query, maxResults)
	}
	return t.searchDuckDuckGo(ctx, query, maxResults)
}

// searchProvider calls a Tavily/Brave/Serper-compatible JSON search API.
func (t *webSearchTool) searchProvider(ctx context.Context, apiURL, apiKey, query string, maxResults int) (string, error) {
	payload := map[string]interface{}{
		"query":   query,
		"num":     maxResults,
		"api_key": apiKey,
	}
	data, err := json.Marshal(payload)
	if err != nil {
		return "", fmt.Errorf("web_search: marshal: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, apiURL, strings.NewReader(string(data)))
	if err != nil {
		return "", fmt.Errorf("web_search: build request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+apiKey)

	resp, err := t.hc.Do(req)
	if err != nil {
		return "", fmt.Errorf("web_search: request: %w", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(io.LimitReader(resp.Body, 256*1024))
	if err != nil {
		return "", fmt.Errorf("web_search: read: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("web_search: provider returned %d: %s", resp.StatusCode, string(body))
	}

	// Attempt to parse a generic results array.
	var generic struct {
		Results []struct {
			Title   string `json:"title"`
			URL     string `json:"url"`
			Content string `json:"content"`
			Snippet string `json:"snippet"`
		} `json:"results"`
	}
	if err := json.Unmarshal(body, &generic); err == nil && len(generic.Results) > 0 {
		var sb strings.Builder
		for i, r := range generic.Results {
			if i >= maxResults {
				break
			}
			snippet := r.Snippet
			if snippet == "" {
				snippet = r.Content
			}
			if len(snippet) > 200 {
				snippet = snippet[:200] + "…"
			}
			sb.WriteString(fmt.Sprintf("%d. %s\n   %s\n   %s\n\n", i+1, r.Title, r.URL, snippet))
		}
		return sb.String(), nil
	}

	return string(body), nil
}

// searchDuckDuckGo queries the DuckDuckGo Instant Answer JSON API.
func (t *webSearchTool) searchDuckDuckGo(ctx context.Context, query string, maxResults int) (string, error) {
	ddgURL := "https://api.duckduckgo.com/?format=json&no_html=1&skip_disambig=1&q=" + url.QueryEscape(query)
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, ddgURL, nil)
	if err != nil {
		return "", fmt.Errorf("web_search: build request: %w", err)
	}
	req.Header.Set("User-Agent", "OHC-BuiltinAgent/1.0")

	resp, err := t.hc.Do(req)
	if err != nil {
		return "", fmt.Errorf("web_search: request: %w", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(io.LimitReader(resp.Body, 256*1024))
	if err != nil {
		return "", fmt.Errorf("web_search: read: %w", err)
	}

	var ddgResp struct {
		Abstract       string `json:"Abstract"`
		AbstractURL    string `json:"AbstractURL"`
		AbstractSource string `json:"AbstractSource"`
		RelatedTopics  []struct {
			Text     string `json:"Text"`
			FirstURL string `json:"FirstURL"`
			Topics   []struct {
				Text     string `json:"Text"`
				FirstURL string `json:"FirstURL"`
			} `json:"Topics"`
		} `json:"RelatedTopics"`
	}
	if err := json.Unmarshal(body, &ddgResp); err != nil {
		return string(body), nil
	}

	var results []searchResult

	if ddgResp.Abstract != "" {
		snippet := ddgResp.Abstract
		if len(snippet) > 300 {
			snippet = snippet[:300] + "…"
		}
		results = append(results, searchResult{
			Title:   ddgResp.AbstractSource,
			URL:     ddgResp.AbstractURL,
			Snippet: snippet,
		})
	}

	for _, rt := range ddgResp.RelatedTopics {
		if rt.FirstURL != "" {
			snippet := rt.Text
			if len(snippet) > 200 {
				snippet = snippet[:200] + "…"
			}
			results = append(results, searchResult{
				Title:   rt.FirstURL,
				URL:     rt.FirstURL,
				Snippet: snippet,
			})
		}
		for _, st := range rt.Topics {
			if st.FirstURL != "" {
				snippet := st.Text
				if len(snippet) > 200 {
					snippet = snippet[:200] + "…"
				}
				results = append(results, searchResult{
					Title:   st.FirstURL,
					URL:     st.FirstURL,
					Snippet: snippet,
				})
			}
		}
		if len(results) >= maxResults {
			break
		}
	}

	if len(results) == 0 {
		return fmt.Sprintf("No results found for %q.\nTip: if you need fuller search results, set OHC_SEARCH_API_URL and OHC_SEARCH_API_KEY environment variables.", query), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Search results for %q:\n\n", query))
	for i, r := range results {
		if i >= maxResults {
			break
		}
		sb.WriteString(fmt.Sprintf("%d. %s\n   %s\n   %s\n\n", i+1, r.Title, r.URL, r.Snippet))
	}
	return sb.String(), nil
}

// ─── TodoTool ─────────────────────────────────────────────────────────────────

// todoTool manages a simple in-memory todo list shared across the agent's lifetime.
type todoTool struct{}

func (t *todoTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "todo_write",
		Description: `Write or update the session task todo list. The entire list is replaced with
the provided items. Use this to track multi-step tasks, mark items as done, and
keep yourself organised across a complex task. Items have a status of "pending",
"in_progress", or "completed", and a priority of "high", "medium", or "low".`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"todos": map[string]interface{}{
					"type": "array",
					"items": map[string]interface{}{
						"type": "object",
						"properties": map[string]interface{}{
							"id":       map[string]interface{}{"type": "string"},
							"content":  map[string]interface{}{"type": "string"},
							"status":   map[string]interface{}{"type": "string", "enum": []string{"pending", "in_progress", "completed"}},
							"priority": map[string]interface{}{"type": "string", "enum": []string{"high", "medium", "low"}},
						},
						"required": []string{"id", "content", "status"},
					},
					"description": "The complete todo list (replaces any previous list).",
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
	switch v := todos.(type) {
	case []interface{}:
		var sb strings.Builder
		pending, inProgress, done := 0, 0, 0
		for _, item := range v {
			if m, ok := item.(map[string]interface{}); ok {
				status := fmt.Sprint(m["status"])
				content := fmt.Sprint(m["content"])
				priority := fmt.Sprint(m["priority"])
				switch status {
				case "pending":
					pending++
					sb.WriteString(fmt.Sprintf("[ ] %s (%s)\n", content, priority))
				case "in_progress":
					inProgress++
					sb.WriteString(fmt.Sprintf("[~] %s (%s)\n", content, priority))
				case "completed":
					done++
					sb.WriteString(fmt.Sprintf("[x] %s\n", content))
				}
			}
		}
		return fmt.Sprintf("Todo list updated: %d pending, %d in progress, %d completed\n\n%s",
			pending, inProgress, done, sb.String()), nil
	default:
		return "Todo list updated.", nil
	}
}

// ─── ToolSearchTool ───────────────────────────────────────────────────────────

// toolSearchTool lists all available tools or filters them by a search term.
type toolSearchTool struct {
	tools []Tool // populated lazily at first use via the agent
}

func (t *toolSearchTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "tool_search",
		Description: `List all tools available to this agent, optionally filtered by a keyword.
Returns tool names and brief descriptions to help you discover what operations
are available.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"query": map[string]interface{}{
					"type":        "string",
					"description": "Optional keyword filter (searches tool names and descriptions).",
				},
			},
		},
	}
}

func (t *toolSearchTool) Execute(_ context.Context, _ string, input map[string]interface{}) (string, error) {
	query := strings.ToLower(strArg(input, "query"))

	if len(t.tools) == 0 {
		return "No tools registered. This is a configuration error.", nil
	}

	var sb strings.Builder
	count := 0
	for _, tool := range t.tools {
		def := tool.Definition()
		if query != "" {
			if !strings.Contains(strings.ToLower(def.Name), query) &&
				!strings.Contains(strings.ToLower(def.Description), query) {
				continue
			}
		}
		// Print first line of description only.
		desc := def.Description
		if idx := strings.IndexByte(desc, '\n'); idx >= 0 {
			desc = desc[:idx]
		}
		sb.WriteString(fmt.Sprintf("%-20s  %s\n", def.Name, desc))
		count++
	}
	if count == 0 {
		return fmt.Sprintf("No tools match %q.", query), nil
	}
	return fmt.Sprintf("Available tools (%d):\n\n%s", count, sb.String()), nil
}

// ─── Hub Task & Message Tools ─────────────────────────────────────────────────

// These tools require access to the Hub and are only included when the agent
// is wired up as an in-process teammate (DefaultToolsWithHub).

// taskCreateTool creates a new task in the Hub task registry.
type taskCreateTool struct {
	hub     Hub
	agentID string
}

func (t *taskCreateTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "task_create",
		Description: `Create a new task in the shared task registry and publish it to the Hub.
Use this to delegate sub-tasks to other agents or track work items that need to
be completed asynchronously. Returns the task ID.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"subject": map[string]interface{}{
					"type":        "string",
					"description": "A brief title for the task.",
				},
				"description": map[string]interface{}{
					"type":        "string",
					"description": "What needs to be done.",
				},
				"assignee": map[string]interface{}{
					"type":        "string",
					"description": "Optional agent ID to assign the task to.",
				},
			},
			"required": []string{"subject", "description"},
		},
	}
}

func (t *taskCreateTool) Execute(_ context.Context, _ string, input map[string]interface{}) (string, error) {
	subject := strArg(input, "subject")
	if subject == "" {
		return "", errors.New("task_create: subject is required")
	}
	description := strArg(input, "description")
	assignee := strArg(input, "assignee")

	taskID, err := generateTaskID()
	if err != nil {
		return "", fmt.Errorf("task_create: %w", err)
	}

	payload, _ := json.Marshal(map[string]string{
		"task_id":     taskID,
		"subject":     subject,
		"description": description,
		"status":      "pending",
		"created_by":  t.agentID,
	})

	toAgent := assignee
	if toAgent == "" {
		toAgent = "orchestrator"
	}

	msg := HubMessage{
		ID:        taskID,
		FromAgent: t.agentID,
		ToAgent:   toAgent,
		Type:      "TaskCreated",
		Content:   string(payload),
	}
	if err := t.hub.Publish(msg); err != nil {
		return "", fmt.Errorf("task_create: publish: %w", err)
	}

	return fmt.Sprintf(`{"task_id":%q,"subject":%q,"status":"pending"}`, taskID, subject), nil
}

// taskGetTool retrieves status of a Hub-tracked task.
type taskGetTool struct {
	hub     Hub
	agentID string
}

func (t *taskGetTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "task_get",
		Description: `Get the current status and details of a task by its ID. Use this to check
whether a delegated task has completed and retrieve its result.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"task_id": map[string]interface{}{
					"type":        "string",
					"description": "The task ID to retrieve.",
				},
			},
			"required": []string{"task_id"},
		},
	}
}

func (t *taskGetTool) Execute(_ context.Context, _ string, input map[string]interface{}) (string, error) {
	taskID := strArg(input, "task_id")
	if taskID == "" {
		return "", errors.New("task_get: task_id is required")
	}

	// Query via Hub message to the orchestrator.
	payload, _ := json.Marshal(map[string]string{"task_id": taskID})
	msg := HubMessage{
		ID:        "query-" + taskID,
		FromAgent: t.agentID,
		ToAgent:   "orchestrator",
		Type:      "TaskStatusQuery",
		Content:   string(payload),
	}
	if err := t.hub.Publish(msg); err != nil {
		return "", fmt.Errorf("task_get: publish: %w", err)
	}

	return fmt.Sprintf(`{"task_id":%q,"queried":true,"note":"Status will be delivered via the inbox."}`, taskID), nil
}

// taskListTool lists tasks visible to the current agent.
type taskListTool struct {
	hub     Hub
	agentID string
}

func (t *taskListTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "task_list",
		Description: `List tasks in the shared task registry. Optionally filter by status.
Returns a JSON array of task summaries.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"status": map[string]interface{}{
					"type":        "string",
					"enum":        []string{"pending", "in_progress", "completed", "failed", "all"},
					"description": `Filter by status (default "all").`,
				},
			},
		},
	}
}

func (t *taskListTool) Execute(_ context.Context, _ string, input map[string]interface{}) (string, error) {
	status := strArg(input, "status")
	if status == "" {
		status = "all"
	}

	payload, _ := json.Marshal(map[string]string{
		"requester": t.agentID,
		"filter":    status,
	})
	msg := HubMessage{
		ID:        "tasklist-" + t.agentID,
		FromAgent: t.agentID,
		ToAgent:   "orchestrator",
		Type:      "TaskListQuery",
		Content:   string(payload),
	}
	if err := t.hub.Publish(msg); err != nil {
		return "", fmt.Errorf("task_list: publish: %w", err)
	}

	return `{"queried":true,"note":"Task list will be delivered via the inbox."}`, nil
}

// taskUpdateTool updates the status of a task.
type taskUpdateTool struct {
	hub     Hub
	agentID string
}

func (t *taskUpdateTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "task_update",
		Description: `Update the status or description of an existing task. Use this to mark tasks
as in_progress when you start working on them, and completed when done.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"task_id": map[string]interface{}{
					"type":        "string",
					"description": "The task ID to update.",
				},
				"status": map[string]interface{}{
					"type":        "string",
					"enum":        []string{"pending", "in_progress", "completed", "failed"},
					"description": "New status for the task.",
				},
				"result": map[string]interface{}{
					"type":        "string",
					"description": "Optional result or notes to attach to the task.",
				},
			},
			"required": []string{"task_id", "status"},
		},
	}
}

func (t *taskUpdateTool) Execute(_ context.Context, _ string, input map[string]interface{}) (string, error) {
	taskID := strArg(input, "task_id")
	if taskID == "" {
		return "", errors.New("task_update: task_id is required")
	}
	status := strArg(input, "status")
	if status == "" {
		return "", errors.New("task_update: status is required")
	}
	result := strArg(input, "result")

	payload, _ := json.Marshal(map[string]string{
		"task_id":    taskID,
		"status":     status,
		"result":     result,
		"updated_by": t.agentID,
	})
	msg := HubMessage{
		ID:        "update-" + taskID,
		FromAgent: t.agentID,
		ToAgent:   "orchestrator",
		Type:      "TaskUpdate",
		Content:   string(payload),
	}
	if err := t.hub.Publish(msg); err != nil {
		return "", fmt.Errorf("task_update: publish: %w", err)
	}

	return fmt.Sprintf(`{"task_id":%q,"status":%q,"updated":true}`, taskID, status), nil
}

// sendMessageTool sends a direct message to another agent via the Hub.
type sendMessageTool struct {
	hub     Hub
	agentID string
}

func (t *sendMessageTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name: "send_message",
		Description: `Send a direct message to another agent via the Hub. Use this to coordinate
with other agents, delegate tasks, request information, or report results.
The recipient will receive the message in their inbox.`,
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"to": map[string]interface{}{
					"type":        "string",
					"description": "The recipient agent ID.",
				},
				"type": map[string]interface{}{
					"type":        "string",
					"description": `Message type, e.g. "TaskAssignment", "Question", "Result" (default "Message").`,
				},
				"content": map[string]interface{}{
					"type":        "string",
					"description": "The message body (plain text or JSON).",
				},
			},
			"required": []string{"to", "content"},
		},
	}
}

func (t *sendMessageTool) Execute(_ context.Context, _ string, input map[string]interface{}) (string, error) {
	to := strArg(input, "to")
	if to == "" {
		return "", errors.New("send_message: to is required")
	}
	content := strArg(input, "content")
	if content == "" {
		return "", errors.New("send_message: content is required")
	}
	msgType := strArg(input, "type")
	if msgType == "" {
		msgType = "Message"
	}

	msgID, err := generateTaskID()
	if err != nil {
		return "", fmt.Errorf("send_message: %w", err)
	}

	msg := HubMessage{
		ID:        msgID,
		FromAgent: t.agentID,
		ToAgent:   to,
		Type:      msgType,
		Content:   content,
	}
	if err := t.hub.Publish(msg); err != nil {
		return "", fmt.Errorf("send_message: publish: %w", err)
	}

	return fmt.Sprintf("Message %q sent to %q (type: %s)", msgID, to, msgType), nil
}
