package mcp

import (
	"errors"
	"fmt"
	"os"
)

// Orchestrator represents the KAIROS Orchestrator interface.
type Orchestrator interface {
	RegisterTool(toolName, endpoint string) error
}

// TelemetryMCPBridge handles the registration of the telemetry MCP tool.
type TelemetryMCPBridge struct {
	orchestrator Orchestrator
}

// NewTelemetryMCPBridge creates a new TelemetryMCPBridge.
func NewTelemetryMCPBridge(orchestrator Orchestrator) *TelemetryMCPBridge {
	return &TelemetryMCPBridge{
		orchestrator: orchestrator,
	}
}

// Register connects the telemetry-mcp-bridge service to the KAIROS Orchestrator.
// It gracefully degrades if Cloud dependencies are absent (Standalone Desktop Mode fallback).
func (b *TelemetryMCPBridge) Register() error {
	if b.orchestrator == nil {
		return errors.New("orchestrator cannot be nil")
	}

	standalone := os.Getenv("OHC_STANDALONE")

	// Define the tool name and endpoint based on the environment.
	toolName := "telemetry-mcp-bridge"
	var endpoint string

	if standalone == "true" {
		// Standalone Desktop Mode fallback
		endpoint = "local://telemetry-bridge"
		fmt.Println("Registering Telemetry MCP Bridge in Standalone Desktop Mode")
	} else {
		// Cloud mode
		endpoint = "http://telemetry-mcp-bridge:9090"
		fmt.Println("Registering Telemetry MCP Bridge in Cloud Mode")
	}

	err := b.orchestrator.RegisterTool(toolName, endpoint)
	if err != nil {
		return fmt.Errorf("failed to register tool with KAIROS orchestrator: %w", err)
	}

	return nil
}
