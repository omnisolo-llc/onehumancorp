package integrations

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/integrations/etcd"
	"github.com/onehumancorp/mono/srcs/server/integrations/libsql"
	"github.com/onehumancorp/mono/srcs/server/integrations/atlas"
	"github.com/onehumancorp/mono/srcs/server/integrations/litefs"
	"github.com/onehumancorp/mono/srcs/server/integrations/obsidian"
	"github.com/onehumancorp/mono/srcs/server/integrations/ollama"
	"github.com/onehumancorp/mono/srcs/server/integrations/powersync"
	"github.com/onehumancorp/mono/srcs/server/integrations/restic"
	"github.com/onehumancorp/mono/srcs/server/integrations/chromadb"
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
	&atlas.AtlasIntegration{},
	&litefs.LiteFSIntegration{},
	&libsql.LibSQLIntegration{},
	&etcd.EtcdIntegration{},
	&ollama.OllamaIntegration{},
	&powersync.PowerSyncIntegration{},
	&restic.ResticIntegration{},
	&obsidian.ObsidianIntegration{},
	&chromadb.ChromaDBIntegration{},
}

// GetCatalog returns all available integration providers.
func GetCatalog() []IntegrationProvider {
	return Catalog
}
