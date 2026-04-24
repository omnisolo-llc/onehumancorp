package ollama

import (
	pb "github.com/onehumancorp/mono/src/proto"
	"google.golang.org/protobuf/proto"
)

type OllamaIntegration struct{}

func (s *OllamaIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("ollama"),
		Name:        proto.String("Ollama"),
		Type:        proto.String("ollama"),
		Category:    proto.String("LLM"),
		Description: proto.String("Local LLM Orchestration via Ollama MCP."),
		Publisher:   proto.String("Ollama"),
		Icon:        proto.String("https://ollama.com/public/icon-64x64.png"),
		Tags:        []string{"llm", "local", "mcp"}}.Build()
}

func (s *OllamaIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure Ollama MCP connection"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("url"),
					Label:       proto.String("Ollama URL"),
					Description: proto.String("The URL of the local Ollama endpoint"),
					Type:        proto.String("url"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}
