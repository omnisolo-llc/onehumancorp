package integrations

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/integrations/litefs"
	"github.com/onehumancorp/mono/srcs/server/integrations/pglite"
)

// Integration represents a plugin blueprint defining how to instantiate connections.
type Integration interface {
	Metadata() *pb.IntegrationMetadata
	WizardSteps() []*pb.WizardStep
}

// Catalog holds the global static list of all supported integration providers.
var Catalog = []Integration{
	&SlackIntegration{},
	&DiscordIntegration{},
	&GoogleChatIntegration{},
	&TelegramIntegration{},
	&TeamsIntegration{},
	&WhatsAppIntegration{},
	&IMessageIntegration{},
	&GitHubIntegration{},
	&GitLabIntegration{},
	&GiteaIntegration{},
	&JiraIntegration{},
	&PlaneIntegration{},
	&GitHubIssuesIntegration{},
	&litefs.LiteFSIntegration{},
	&pglite.PGLiteIntegration{},
}

// GetCatalog returns all available integration providers.
func GetCatalog() []Integration {
	return Catalog
}
