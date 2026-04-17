package mcp_inference_router

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"
)

type InferenceRequest struct {
	Prompt      string `json:"prompt"`
	TokenCount  int    `json:"token_count"`
	IsSensitive bool   `json:"is_sensitive"`
}

type InferenceResponse struct {
	Result    string `json:"result"`
	Source    string `json:"source"`
	RoutedVia string `json:"routed_via"`
}

type InferenceRouter struct {
	LocalEndpoint string
	CloudEndpoint string
	HTTPClient    *http.Client
}

func NewInferenceRouter(local, cloud string) *InferenceRouter {
	return &InferenceRouter{
		LocalEndpoint: local,
		CloudEndpoint: cloud,
		HTTPClient: &http.Client{
			Timeout: 30 * time.Second,
		},
	}
}

func (r *InferenceRouter) RouteInference(ctx context.Context, req InferenceRequest) (InferenceResponse, error) {
	// 1. Check privacy constraints first
	if req.IsSensitive {
		return r.executeLocal(ctx, req)
	}

	// 2. Resource check: Evaluate prompt token size
	// If the prompt is too large for local execution (e.g., > 1024 tokens), offload to cloud
	if req.TokenCount > 1024 {
		resp, err := r.executeCloud(ctx, req)
		if err == nil {
			return resp, nil
		}
		// Fallback to local if cloud fails
		return r.executeLocal(ctx, req)
	}

	// 3. Default to local execution for smaller prompts
	resp, err := r.executeLocal(ctx, req)
	if err != nil {
		// If local fails, try cloud offloading as fallback (if not sensitive)
		return r.executeCloud(ctx, req)
	}
	return resp, nil
}

func (r *InferenceRouter) executeLocal(ctx context.Context, req InferenceRequest) (InferenceResponse, error) {
	resp, err := r.doRequest(ctx, r.LocalEndpoint, req)
	if err != nil {
		return InferenceResponse{}, fmt.Errorf("local inference failed: %w", err)
	}
	resp.Source = "Local"
	return resp, nil
}

func (r *InferenceRouter) executeCloud(ctx context.Context, req InferenceRequest) (InferenceResponse, error) {
	resp, err := r.doRequest(ctx, r.CloudEndpoint, req)
	if err != nil {
		return InferenceResponse{}, fmt.Errorf("cloud inference failed: %w", err)
	}
	resp.Source = "Cloud"
	return resp, nil
}

func (r *InferenceRouter) doRequest(ctx context.Context, url string, req InferenceRequest) (InferenceResponse, error) {
	reqBytes, err := json.Marshal(req)
	if err != nil {
		return InferenceResponse{}, fmt.Errorf("failed to marshal request: %w", err)
	}

	httpReq, err := http.NewRequestWithContext(ctx, "POST", url, bytes.NewBuffer(reqBytes))
	if err != nil {
		return InferenceResponse{}, fmt.Errorf("failed to create http request: %w", err)
	}
	httpReq.Header.Set("Content-Type", "application/json")

	httpResp, err := r.HTTPClient.Do(httpReq)
	if err != nil {
		return InferenceResponse{}, fmt.Errorf("http request failed: %w", err)
	}
	defer httpResp.Body.Close()

	if httpResp.StatusCode < 200 || httpResp.StatusCode >= 300 {
		return InferenceResponse{}, fmt.Errorf("server returned status: %d", httpResp.StatusCode)
	}

	bodyBytes, err := io.ReadAll(httpResp.Body)
	if err != nil {
		return InferenceResponse{}, fmt.Errorf("failed to read response body: %w", err)
	}

	var infResp InferenceResponse
	if err := json.Unmarshal(bodyBytes, &infResp); err != nil {
		// Attempt to read as raw string if JSON parsing fails
		infResp.Result = string(bodyBytes)
	}

	infResp.RoutedVia = "mcp_inference_router"
	return infResp, nil
}