package chromadb

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// ChromaDBTool provides methods to interact with local ChromaDB daemon/HTTP port.
type ChromaDBTool struct {
	dbProvider db.Provider
	httpClient *http.Client
}

// NewChromaDBTool initializes the ChromaDB tool handler.
func NewChromaDBTool(dbProvider db.Provider) *ChromaDBTool {
	return &ChromaDBTool{
		dbProvider: dbProvider,
		httpClient: &http.Client{Timeout: 10 * time.Second},
	}
}

func (t *ChromaDBTool) CreateCollectionTool() builtin.Tool {
	return builtin.Tool{
		Name:        "chromadb_create_collection",
		Description: "Create a new ChromaDB collection.",
		Parameters:  json.RawMessage(`{"type":"object","properties":{"url":{"type":"string","description":"ChromaDB URL (default: http://localhost:8000)"},"collection_name":{"type":"string"}},"required":["collection_name"]}`),
		Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
			var input struct {
				URL            string `json:"url"`
				CollectionName string `json:"collection_name"`
			}
			if err := json.Unmarshal(args, &input); err != nil {
				return "", fmt.Errorf("invalid arguments: %w", err)
			}

			chromaURL := input.URL
			if chromaURL == "" {
				chromaURL = "http://localhost:8000"
			}
			chromaURL = strings.TrimRight(chromaURL, "/")

			headless := os.Getenv("OHC_HEADLESS") == "true"
			standalone := os.Getenv("OHC_STANDALONE") == "true"

			if !headless && !standalone {
				return `{"status": "mocked", "message": "ChromaDB tool 'create_collection' mocked in Cloud mode"}`, nil
			}

			if input.CollectionName == "" {
				return "", fmt.Errorf("collection_name is required")
			}

			endpoint := fmt.Sprintf("%s/api/v1/collections", chromaURL)
			reqBody := map[string]interface{}{"name": input.CollectionName}
			jsonData, _ := json.Marshal(reqBody)

			req, err := http.NewRequestWithContext(ctx, http.MethodPost, endpoint, strings.NewReader(string(jsonData)))
			if err != nil {
				return "", err
			}
			req.Header.Set("Content-Type", "application/json")

			resp, err := t.httpClient.Do(req)
			if err != nil {
				return "", err
			}
			defer resp.Body.Close()

			if resp.StatusCode >= 400 {
				body, _ := io.ReadAll(resp.Body)
				return "", fmt.Errorf("chromadb returned status %d: %s", resp.StatusCode, string(body))
			}

			var result map[string]interface{}
			if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
				return "", err
			}
			resStr, _ := json.Marshal(result)
			return string(resStr), nil
		},
	}
}

func (t *ChromaDBTool) AddDocumentsTool() builtin.Tool {
	return builtin.Tool{
		Name:        "chromadb_add_documents",
		Description: "Add documents to a ChromaDB collection.",
		Parameters:  json.RawMessage(`{"type":"object","properties":{"url":{"type":"string","description":"ChromaDB URL"},"collection_id":{"type":"string"},"documents":{"type":"array","items":{"type":"string"}},"ids":{"type":"array","items":{"type":"string"}}},"required":["collection_id","documents","ids"]}`),
		Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
			var input struct {
				URL          string   `json:"url"`
				CollectionID string   `json:"collection_id"`
				Documents    []string `json:"documents"`
				IDs          []string `json:"ids"`
			}
			if err := json.Unmarshal(args, &input); err != nil {
				return "", fmt.Errorf("invalid arguments: %w", err)
			}

			chromaURL := input.URL
			if chromaURL == "" {
				chromaURL = "http://localhost:8000"
			}
			chromaURL = strings.TrimRight(chromaURL, "/")

			headless := os.Getenv("OHC_HEADLESS") == "true"
			standalone := os.Getenv("OHC_STANDALONE") == "true"

			if !headless && !standalone {
				return `{"status": "mocked", "message": "ChromaDB tool 'add_documents' mocked in Cloud mode"}`, nil
			}

			if input.CollectionID == "" {
				return "", fmt.Errorf("collection_id is required")
			}

			endpoint := fmt.Sprintf("%s/api/v1/collections/%s/add", chromaURL, input.CollectionID)
			reqBody := map[string]interface{}{"documents": input.Documents, "ids": input.IDs}
			jsonData, _ := json.Marshal(reqBody)

			req, err := http.NewRequestWithContext(ctx, http.MethodPost, endpoint, strings.NewReader(string(jsonData)))
			if err != nil {
				return "", err
			}
			req.Header.Set("Content-Type", "application/json")

			resp, err := t.httpClient.Do(req)
			if err != nil {
				return "", err
			}
			defer resp.Body.Close()

			if resp.StatusCode >= 400 {
				body, _ := io.ReadAll(resp.Body)
				return "", fmt.Errorf("chromadb returned status %d: %s", resp.StatusCode, string(body))
			}

			var result map[string]interface{}
			if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
				return "", err
			}
			resStr, _ := json.Marshal(result)
			return string(resStr), nil
		},
	}
}

func (t *ChromaDBTool) QueryTool() builtin.Tool {
	return builtin.Tool{
		Name:        "chromadb_query",
		Description: "Query documents from a ChromaDB collection.",
		Parameters:  json.RawMessage(`{"type":"object","properties":{"url":{"type":"string","description":"ChromaDB URL"},"collection_id":{"type":"string"},"query_texts":{"type":"array","items":{"type":"string"}},"n_results":{"type":"number"}},"required":["collection_id","query_texts"]}`),
		Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
			var input struct {
				URL          string   `json:"url"`
				CollectionID string   `json:"collection_id"`
				QueryTexts   []string `json:"query_texts"`
				NResults     int      `json:"n_results"`
			}
			if err := json.Unmarshal(args, &input); err != nil {
				return "", fmt.Errorf("invalid arguments: %w", err)
			}

			chromaURL := input.URL
			if chromaURL == "" {
				chromaURL = "http://localhost:8000"
			}
			chromaURL = strings.TrimRight(chromaURL, "/")

			headless := os.Getenv("OHC_HEADLESS") == "true"
			standalone := os.Getenv("OHC_STANDALONE") == "true"

			if !headless && !standalone {
				return `{"status": "mocked", "message": "ChromaDB tool 'query' mocked in Cloud mode"}`, nil
			}

			if input.CollectionID == "" {
				return "", fmt.Errorf("collection_id is required")
			}

			if input.NResults == 0 {
				input.NResults = 10
			}

			endpoint := fmt.Sprintf("%s/api/v1/collections/%s/query", chromaURL, input.CollectionID)
			reqBody := map[string]interface{}{"query_texts": input.QueryTexts, "n_results": input.NResults}
			jsonData, _ := json.Marshal(reqBody)

			req, err := http.NewRequestWithContext(ctx, http.MethodPost, endpoint, strings.NewReader(string(jsonData)))
			if err != nil {
				return "", err
			}
			req.Header.Set("Content-Type", "application/json")

			resp, err := t.httpClient.Do(req)
			if err != nil {
				return "", err
			}
			defer resp.Body.Close()

			if resp.StatusCode >= 400 {
				body, _ := io.ReadAll(resp.Body)
				return "", fmt.Errorf("chromadb returned status %d: %s", resp.StatusCode, string(body))
			}

			var result map[string]interface{}
			if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
				return "", err
			}
			resStr, _ := json.Marshal(result)
			return string(resStr), nil
		},
	}
}

// ListTools returns all builtin tools for ChromaDB.
func (t *ChromaDBTool) ListTools() []builtin.Tool {
	return []builtin.Tool{
		t.CreateCollectionTool(),
		t.AddDocumentsTool(),
		t.QueryTool(),
	}
}
