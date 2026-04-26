package ollama

import (
	pb "github.com/onehumancorp/mono/src/proto"
)

type OllamaIntegration struct{}

func (s *OllamaIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "ollama",
		Name:        "Ollama",
		Type:        "ollama",
		Category:    "LLM",
		Description: "Local LLM Orchestration via Ollama MCP.",
		Publisher:   "Ollama",
		Icon:        "https://ollama.com/public/icon-64x64.png",
		Tags:        []string{"llm", "local", "mcp"}}
}

func (s *OllamaIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title:       "Connection Data",
			Description: "Configure Ollama MCP connection",
			Fields: []*pb.WizardField{
				&pb.WizardField{
					Key:         "url",
					Label:       "Ollama URL",
					Description: "The URL of the local Ollama endpoint",
					Type:        "url",
					Required:    true,
				},
			},
		},
	}
}
