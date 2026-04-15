package integrations

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/protobuf/proto"
)

type SlackIntegration struct{}

func (s *SlackIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("slack"),
		Name:        proto.String("Slack"),
		Type:        proto.String(string(IntegrationTypeSlack)),
		Category:    proto.String(string(CategoryChat)),
		Description: proto.String("Send agent-to-human notifications and HITL approval requests via Slack channels."),
		Publisher:   proto.String("Slack Technologies"),
		Icon:        proto.String("https://upload.wikimedia.org/wikipedia/commons/d/d5/Slack_icon_2019.svg"),
		Tags:        []string{"enterprise", "chat", "notifications"}}.Build()
}
func (s *SlackIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure Slack API credentials"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{Key: proto.String("token"), Label: proto.String("Bot Token"), Type: proto.String("password"), Required: proto.Bool(true)}.Build(),
				pb.WizardField_builder{Key: proto.String("channel_id"), Label: proto.String("Channel ID"), Type: proto.String("text"), Required: proto.Bool(true)}.Build(),
			},
		}.Build(),
	}
}

type DiscordIntegration struct{}

func (s *DiscordIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("discord"),
		Name:        proto.String("Discord"),
		Type:        proto.String(string(IntegrationTypeDiscord)),
		Category:    proto.String(string(CategoryChat)),
		Description: proto.String("Post agent messages and meeting summaries to Discord channels or threads."),
		Publisher:   proto.String("Discord Inc."),
		Icon:        proto.String("https://assets-global.website-files.com/6257adef93867e50d84d30e2/636e0a6a49cf127bf92de1e2_icon_clyde_blurple_RGB.png"),
		Tags:        []string{"consumer", "community", "chat"}}.Build()
}
func (s *DiscordIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Webhook Configuration"),
			Description: proto.String("Setup a Discord incoming webhook URL"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{Key: proto.String("webhook_url"), Label: proto.String("Webhook URL"), Type: proto.String("url"), Required: proto.Bool(true)}.Build(),
			},
		}.Build(),
	}
}

type GoogleChatIntegration struct{}

func (s *GoogleChatIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("google-chat"),
		Name:        proto.String("Google Chat"),
		Type:        proto.String(string(IntegrationTypeGoogleChat)),
		Category:    proto.String(string(CategoryChat)),
		Description: proto.String("Deliver agent updates and approval requests via Google Chat spaces."),
		Publisher:   proto.String("Google"),
		Icon:        proto.String("https://upload.wikimedia.org/wikipedia/commons/e/e0/Google_Chat_icon_%20%282020%29.svg"),
		Tags:        []string{"enterprise", "workspace", "chat"}}.Build()
}
func (s *GoogleChatIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title: proto.String("Space Credentials"), Fields: []*pb.WizardField{
				pb.WizardField_builder{Key: proto.String("webhook_url"), Label: proto.String("Webhook URL"), Type: proto.String("url"), Required: proto.Bool(true)}.Build(),
			},
		}.Build(),
	}
}

type TelegramIntegration struct{}

func (s *TelegramIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("telegram"),
		Name:        proto.String("Telegram"),
		Type:        proto.String(string(IntegrationTypeTelegram)),
		Category:    proto.String(string(CategoryChat)),
		Description: proto.String("Send agent notifications and HITL approval requests via Telegram bots and channels."),
		Publisher:   proto.String("Telegram Messenger LLP"),
		Icon:        proto.String("https://upload.wikimedia.org/wikipedia/commons/8/82/Telegram_logo.svg"),
		Tags:        []string{"consumer", "secure", "chat"}}.Build()
}
func (s *TelegramIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title: proto.String("Bot Configuration"), Fields: []*pb.WizardField{
				pb.WizardField_builder{Key: proto.String("bot_token"), Label: proto.String("Bot Token"), Type: proto.String("password"), Required: proto.Bool(true)}.Build(),
				pb.WizardField_builder{Key: proto.String("chat_id"), Label: proto.String("Chat ID"), Type: proto.String("text"), Required: proto.Bool(true)}.Build(),
			},
		}.Build(),
	}
}

type TeamsIntegration struct{}

func (s *TeamsIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("teams"),
		Name:        proto.String("Microsoft Teams"),
		Type:        proto.String(string(IntegrationTypeTeams)),
		Category:    proto.String(string(CategoryChat)),
		Description: proto.String("Deliver agent updates and approval requests to Microsoft Teams channels via webhooks."),
		Publisher:   proto.String("Microsoft"),
		Icon:        proto.String("https://upload.wikimedia.org/wikipedia/commons/c/c9/Microsoft_Office_Teams_%282018%E2%80%93present%29.svg"),
		Tags:        []string{"enterprise", "office365", "chat"}}.Build()
}
func (s *TeamsIntegration) WizardSteps() []*pb.WizardStep { return nil }

type WhatsAppIntegration struct{}

func (s *WhatsAppIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("whatsapp"),
		Name:        proto.String("WhatsApp"),
		Type:        proto.String(string(IntegrationTypeWhatsApp)),
		Category:    proto.String(string(CategoryChat)),
		Description: proto.String("Connect directly with users through WhatsApp messaging for consumer interactions and updates."),
		Publisher:   proto.String("Meta"),
		Icon:        proto.String("https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg"),
		Tags:        []string{"consumer", "messaging", "chat"}}.Build()
}
func (s *WhatsAppIntegration) WizardSteps() []*pb.WizardStep { return nil }

type IMessageIntegration struct{}

func (s *IMessageIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("imessage"),
		Name:        proto.String("iMessage"),
		Type:        proto.String(string(IntegrationTypeIMessage)),
		Category:    proto.String(string(CategoryChat)),
		Description: proto.String("Interact with users natively via Apple's iMessage interface."),
		Publisher:   proto.String("Apple"),
		Icon:        proto.String("https://upload.wikimedia.org/wikipedia/commons/5/51/IMessage_logo.svg"),
		Tags:        []string{"consumer", "apple", "chat"}}.Build()
}
func (s *IMessageIntegration) WizardSteps() []*pb.WizardStep { return nil }

type GitHubIntegration struct{}

func (s *GitHubIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("github"),
		Name:        proto.String("GitHub"),
		Type:        proto.String(string(IntegrationTypeGitHub)),
		Category:    proto.String(string(CategoryGit)),
		BaseUrl:     proto.String("https://github.com"),
		Description: proto.String("Open pull requests, review code, and manage branches on GitHub."),
		Publisher:   proto.String("GitHub, Inc."),
		Icon:        proto.String("https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"),
		Tags:        []string{"developer", "version-control", "git"}}.Build()
}
func (s *GitHubIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title: proto.String("Personal Access Token"), Fields: []*pb.WizardField{
				pb.WizardField_builder{Key: proto.String("pat"), Label: proto.String("Token"), Type: proto.String("password"), Required: proto.Bool(true)}.Build(),
			},
		}.Build(),
	}
}

type GitLabIntegration struct{}

func (s *GitLabIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("gitlab"),
		Name:        proto.String("GitLab"),
		Type:        proto.String(string(IntegrationTypeGitLab)),
		Category:    proto.String(string(CategoryGit)),
		BaseUrl:     proto.String("https://gitlab.com"),
		Description: proto.String("Create merge requests and manage repositories on GitLab or self-hosted instances."),
		Publisher:   proto.String("GitLab Inc."),
		Icon:        proto.String("https://about.gitlab.com/images/press/logo/png/gitlab-icon-rgb.png"),
		Tags:        []string{"developer", "version-control", "git", "self-hosted"}}.Build()
}
func (s *GitLabIntegration) WizardSteps() []*pb.WizardStep { return nil }

type GiteaIntegration struct{}

func (s *GiteaIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("gitea"),
		Name:        proto.String("Gitea"),
		Type:        proto.String(string(IntegrationTypeGitea)),
		Category:    proto.String(string(CategoryGit)),
		Description: proto.String("Open PRs on a self-hosted Gitea instance — the zero-lock-in OSS git option."),
		Publisher:   proto.String("The Gitea Authors"),
		Icon:        proto.String("https://about.gitea.com/gitea.svg"),
		Tags:        []string{"developer", "open-source", "git"}}.Build()
}
func (s *GiteaIntegration) WizardSteps() []*pb.WizardStep { return nil }

type JiraIntegration struct{}

func (s *JiraIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("jira"),
		Name:        proto.String("Jira"),
		Type:        proto.String(string(IntegrationTypeJIRA)),
		Category:    proto.String(string(CategoryIssues)),
		Description: proto.String("Create and manage issues, epics, and sprints in Atlassian Jira."),
		Publisher:   proto.String("Atlassian"),
		Icon:        proto.String("https://wac-cdn.atlassian.com/dam/jcr:e3372c08-4107-4286-ab72-1b1e6aeb637a/logos-jira-icon-blue.svg"),
		Tags:        []string{"enterprise", "issues", "agile"}}.Build()
}
func (s *JiraIntegration) WizardSteps() []*pb.WizardStep { return nil }

type PlaneIntegration struct{}

func (s *PlaneIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("plane"),
		Name:        proto.String("Plane"),
		Type:        proto.String(string(IntegrationTypePlane)),
		Category:    proto.String(string(CategoryIssues)),
		Description: proto.String("Manage issues and cycles with Plane — the open-source Jira alternative."),
		Publisher:   proto.String("MakePlane"),
		Icon:        proto.String("https://plane.so/favicon.ico"),
		Tags:        []string{"open-source", "issues", "project-management"}}.Build()
}
func (s *PlaneIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title: proto.String("Credentials"), Fields: []*pb.WizardField{
				pb.WizardField_builder{Key: proto.String("api_key"), Label: proto.String("API Key"), Type: proto.String("password"), Required: proto.Bool(true)}.Build(),
				pb.WizardField_builder{Key: proto.String("workspace"), Label: proto.String("Workspace Slug"), Type: proto.String("text"), Required: proto.Bool(true)}.Build(),
			},
		}.Build(),
	}
}

type GitHubIssuesIntegration struct{}

func (s *GitHubIssuesIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("github-issues"),
		Name:        proto.String("GitHub Issues"),
		Type:        proto.String(string(IntegrationTypeGitHubIssues)),
		Category:    proto.String(string(CategoryIssues)),
		Description: proto.String("Track tasks and bugs directly in GitHub Issues alongside your repositories."),
		Publisher:   proto.String("GitHub, Inc."),
		Icon:        proto.String("https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"),
		Tags:        []string{"developer", "issues", "integrated"}}.Build()
}
func (s *GitHubIssuesIntegration) WizardSteps() []*pb.WizardStep { return nil }

