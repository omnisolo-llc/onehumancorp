package integrations

import (
	pb "github.com/onehumancorp/mono/src/proto"
	"github.com/onehumancorp/mono/src/server/integrations/etcd"
	"github.com/onehumancorp/mono/src/server/integrations/libsql"
	"github.com/onehumancorp/mono/src/server/integrations/litefs"
	"github.com/onehumancorp/mono/src/server/integrations/obsidian"
	"github.com/onehumancorp/mono/src/server/integrations/ollama"
	"github.com/onehumancorp/mono/src/server/integrations/powersync"
	"github.com/onehumancorp/mono/src/server/integrations/restic"
	"github.com/onehumancorp/mono/src/server/integrations/nats"
)

// IntegrationProvider represents a plugin blueprint defining how to instantiate connections.
type IntegrationProvider interface {
	Metadata() *pb.IntegrationMetadata
	WizardSteps() []*pb.WizardStep
}

// Catalog holds the global static list of all supported integration providers.
var Catalog = []IntegrationProvider{
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
	&libsql.LibSQLIntegration{},
	&etcd.EtcdIntegration{},
	&ollama.OllamaIntegration{},
	&powersync.PowerSyncIntegration{},
	&restic.ResticIntegration{},
	&obsidian.ObsidianIntegration{},
	&nats.NatsIntegration{},
}

// GetCatalog returns all available integration providers.
func GetCatalog() []IntegrationProvider {
	return Catalog
}
