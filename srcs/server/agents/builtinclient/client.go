package builtinclient

import (
	"context"
	"errors"
	"fmt"
	"io"
	"net"
	"os"
	"strings"
	"time"

	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"google.golang.org/grpc"
	"google.golang.org/protobuf/proto"
)

const (
	DefaultAddress    = "127.0.0.1:50051"
	defaultModel      = "llama3"
	defaultProvider   = "ollama"
	defaultMaxTokens  = 2048
	defaultContextCap = 64
)

type EventHandler func(*agentservicepb.RunTaskEvent)

type RunTaskRequest struct {
	Task               string
	Model              string
	LLMProvider        string
	LLMEndpoint        string
	SystemPrompt       string
	MaxTokens          int32
	Temperature        float32
	MaxIterations      int32
	MaxContextMessages int32
}

type SubAgentRequest struct {
	Task               string
	Model              string
	LLMProvider        string
	LLMEndpoint        string
	SystemPrompt       string
	MaxTokens          int32
	Temperature        float32
	MaxIterations      int32
	MaxContextMessages int32
	SubAgentAddress    string
}

type PingInfo struct {
	AgentID string
	Version string
}

type Client struct {
	address string
	conn    *grpc.ClientConn
	stub    agentservicepb.AgentServiceClient
}

func AddressFromEnv() string {
	if address := os.Getenv("OHC_BUILTIN_AGENT_ADDRESS"); address != "" {
		return address
	}
	return DefaultAddress
}

func IsLocalAddress(address string) bool {
	host, _, err := splitHostPort(address)
	if err != nil {
		return false
	}
	if host == "" || host == "localhost" || host == "0.0.0.0" || host == "::" || host == "[::]" {
		return true
	}
	ip := net.ParseIP(strings.Trim(host, "[]"))
	return ip != nil && ip.IsLoopback()
}

func PortFromAddress(address string) (string, error) {
	_, port, err := splitHostPort(address)
	if err != nil {
		return "", err
	}
	if port == "" {
		return "", fmt.Errorf("builtin agent address %q does not include a port", address)
	}
	return port, nil
}

func Dial(address string) (*Client, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	return DialContext(ctx, address)
}

func DialContext(ctx context.Context, address string) (*Client, error) {
	if address == "" {
		address = DefaultAddress
	}
	conn, err := grpc.DialContext(ctx, address, grpc.WithBlock(), grpc.WithInsecure())
	if err != nil {
		return nil, fmt.Errorf("dial builtin agent %q: %w", address, err)
	}
	return &Client{
		address: address,
		conn:    conn,
		stub:    agentservicepb.NewAgentServiceClient(conn),
	}, nil
}

func WaitForReady(ctx context.Context, address string, interval time.Duration) error {
	if interval <= 0 {
		interval = 250 * time.Millisecond
	}

	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	var lastErr error
	for {
		client, err := DialContext(ctx, address)
		if err == nil {
			_, pingErr := client.Ping(ctx)
			_ = client.Close()
			if pingErr == nil {
				return nil
			}
			lastErr = pingErr
		} else {
			lastErr = err
		}

		select {
		case <-ctx.Done():
			if lastErr != nil {
				return fmt.Errorf("wait for builtin agent %q: %w", address, lastErr)
			}
			return fmt.Errorf("wait for builtin agent %q: %w", address, ctx.Err())
		case <-ticker.C:
		}
	}
}

func (c *Client) Close() error {
	if c == nil || c.conn == nil {
		return nil
	}
	return c.conn.Close()
}

func (c *Client) Ping(ctx context.Context) (PingInfo, error) {
	resp, err := c.stub.Ping(ctx, &agentservicepb.PingRequest{})
	if err != nil {
		return PingInfo{}, fmt.Errorf("ping builtin agent %q: %w", c.address, err)
	}
	return PingInfo{AgentID: resp.GetAgentId(), Version: resp.GetVersion()}, nil
}

func (c *Client) RunTask(ctx context.Context, req RunTaskRequest, onEvent EventHandler) (string, error) {
	if strings.TrimSpace(req.Task) == "" {
		return "", errors.New("builtin RunTask request requires a task")
	}

	runtimeConfig := buildRuntimeConfig(
		req.Model,
		req.LLMProvider,
		req.LLMEndpoint,
		req.SystemPrompt,
		req.MaxTokens,
		req.Temperature,
		req.MaxIterations,
		req.MaxContextMessages,
	)

	stream, err := c.stub.RunTask(ctx, agentservicepb.RunTaskRequest_builder{
		Task:               proto.String(req.Task),
		Model:              proto.String(chooseString(req.Model, defaultModel)),
		LlmProvider:        proto.String(chooseString(req.LLMProvider, defaultProvider)),
		LlmEndpoint:        proto.String(chooseString(req.LLMEndpoint, os.Getenv("OHC_LOCAL_LLM_ENDPOINT"))),
		SystemPrompt:       proto.String(req.SystemPrompt),
		MaxTokens:          proto.Int32(chooseInt32(req.MaxTokens, defaultMaxTokens)),
		Temperature:        proto.Float32(chooseTemperature(req.Temperature)),
		MaxContextMessages: proto.Int32(chooseInt32(req.MaxContextMessages, defaultContextCap)),
		RuntimeConfig:      runtimeConfig,
	}.Build())
	if err != nil {
		return "", fmt.Errorf("start builtin RunTask stream: %w", err)
	}

	var final string
	for {
		event, err := stream.Recv()
		if errors.Is(err, io.EOF) {
			return final, nil
		}
		if err != nil {
			return final, fmt.Errorf("receive builtin RunTask event: %w", err)
		}
		if onEvent != nil {
			onEvent(event)
		}
		switch event.GetType() {
		case agentservicepb.EventType_TASK_COMPLETE:
			final = event.GetContent()
		case agentservicepb.EventType_TASK_ERROR:
			return final, errors.New(event.GetError())
		}
	}
}

func (c *Client) DispatchToSubAgent(ctx context.Context, req SubAgentRequest) (string, error) {
	if strings.TrimSpace(req.Task) == "" {
		return "", errors.New("builtin sub-agent request requires a task")
	}
	runtimeConfig := buildRuntimeConfig(
		req.Model,
		req.LLMProvider,
		req.LLMEndpoint,
		req.SystemPrompt,
		req.MaxTokens,
		req.Temperature,
		req.MaxIterations,
		req.MaxContextMessages,
	)
	resp, err := c.stub.DispatchToSubAgent(ctx, agentservicepb.SubAgentRequest_builder{
		Task:            proto.String(req.Task),
		Model:           proto.String(chooseString(req.Model, defaultModel)),
		LlmProvider:     proto.String(chooseString(req.LLMProvider, defaultProvider)),
		LlmEndpoint:     proto.String(chooseString(req.LLMEndpoint, os.Getenv("OHC_LOCAL_LLM_ENDPOINT"))),
		SystemPrompt:    proto.String(req.SystemPrompt),
		MaxTokens:       proto.Int32(chooseInt32(req.MaxTokens, defaultMaxTokens)),
		Temperature:     proto.Float32(chooseTemperature(req.Temperature)),
		RuntimeConfig:   runtimeConfig,
		SubAgentAddress: proto.String(req.SubAgentAddress),
	}.Build())
	if err != nil {
		return "", fmt.Errorf("dispatch to builtin sub-agent: %w", err)
	}
	if resp.GetError() != "" {
		return "", errors.New(resp.GetError())
	}
	return resp.GetResult(), nil
}

func splitHostPort(address string) (string, string, error) {
	if strings.HasPrefix(address, ":") {
		return "", strings.TrimPrefix(address, ":"), nil
	}
	host, port, err := net.SplitHostPort(address)
	if err == nil {
		return host, port, nil
	}
	if strings.Count(address, ":") == 1 && !strings.Contains(address, "]") {
		parts := strings.SplitN(address, ":", 2)
		return parts[0], parts[1], nil
	}
	return "", "", fmt.Errorf("parse builtin agent address %q: %w", address, err)
}

func chooseString(value, fallback string) string {
	if value != "" {
		return value
	}
	return fallback
}

func chooseInt32(value, fallback int32) int32 {
	if value > 0 {
		return value
	}
	return fallback
}

func chooseTemperature(value float32) float32 {
	if value > 0 {
		return value
	}
	return 0.7
}

func buildRuntimeConfig(model, provider, endpoint, systemPrompt string, maxTokens int32, temperature float32, maxIterations int32, maxContextMessages int32) *agentservicepb.AgentRuntimeConfig {
	return agentservicepb.AgentRuntimeConfig_builder{
		Model:              proto.String(chooseString(model, defaultModel)),
		LlmProvider:        proto.String(chooseString(provider, defaultProvider)),
		LlmEndpoint:        proto.String(chooseString(endpoint, os.Getenv("OHC_LOCAL_LLM_ENDPOINT"))),
		SystemPrompt:       proto.String(systemPrompt),
		MaxTokens:          proto.Int32(chooseInt32(maxTokens, defaultMaxTokens)),
		Temperature:        proto.Float32(chooseTemperature(temperature)),
		MaxIterations:      proto.Int32(chooseInt32(maxIterations, 50)),
		MaxContextMessages: proto.Int32(chooseInt32(maxContextMessages, defaultContextCap)),
	}.Build()
}
