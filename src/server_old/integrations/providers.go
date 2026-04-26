package integrations

import (
	pb "github.com/onehumancorp/mono/src/proto"
)

type SlackIntegration struct{}

func (s *SlackIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "slack",
		Name:        "Slack",
		Type:        string(IntegrationTypeSlack),
		Category:    string(CategoryChat),
		Description: "Send agent-to-human notifications and HITL approval requests via Slack channels.",
		Publisher:   "Slack Technologies",
		Icon:        "https://upload.wikimedia.org/wikipedia/commons/d/d5/Slack_icon_2019.svg",
		Tags:        []string{"enterprise", "chat", "notifications"}}
}
func (s *SlackIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title:       "Connection Data",
			Description: "Configure Slack API credentials",
			Fields: []*pb.WizardField{
				&pb.WizardField{Key: "token", Label: "Bot Token", Type: "password", Required: true},
				&pb.WizardField{Key: "channel_id", Label: "Channel ID", Type: "text", Required: true},
			},
		},
	}
}

type DiscordIntegration struct{}

func (s *DiscordIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "discord",
		Name:        "Discord",
		Type:        string(IntegrationTypeDiscord),
		Category:    string(CategoryChat),
		Description: "Post agent messages and meeting summaries to Discord channels or threads.",
		Publisher:   "Discord Inc.",
		Icon:        "https://assets-global.website-files.com/6257adef93867e50d84d30e2/636e0a6a49cf127bf92de1e2_icon_clyde_blurple_RGB.png",
		Tags:        []string{"consumer", "community", "chat"}}
}
func (s *DiscordIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title:       "Webhook Configuration",
			Description: "Setup a Discord incoming webhook URL",
			Fields: []*pb.WizardField{
				&pb.WizardField{Key: "webhook_url", Label: "Webhook URL", Type: "url", Required: true},
			},
		},
	}
}

type GoogleChatIntegration struct{}

func (s *GoogleChatIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "google-chat",
		Name:        "Google Chat",
		Type:        string(IntegrationTypeGoogleChat),
		Category:    string(CategoryChat),
		Description: "Deliver agent updates and approval requests via Google Chat spaces.",
		Publisher:   "Google",
		Icon:        "https://upload.wikimedia.org/wikipedia/commons/e/e0/Google_Chat_icon_%20%282020%29.svg",
		Tags:        []string{"enterprise", "workspace", "chat"}}
}
func (s *GoogleChatIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title: "Space Credentials", Fields: []*pb.WizardField{
				&pb.WizardField{Key: "webhook_url", Label: "Webhook URL", Type: "url", Required: true},
			},
		},
	}
}

type TelegramIntegration struct{}

func (s *TelegramIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "telegram",
		Name:        "Telegram",
		Type:        string(IntegrationTypeTelegram),
		Category:    string(CategoryChat),
		Description: "Send agent notifications and HITL approval requests via Telegram bots and channels.",
		Publisher:   "Telegram Messenger LLP",
		Icon:        "https://upload.wikimedia.org/wikipedia/commons/8/82/Telegram_logo.svg",
		Tags:        []string{"consumer", "secure", "chat"}}
}
func (s *TelegramIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title: "Bot Configuration", Fields: []*pb.WizardField{
				&pb.WizardField{Key: "bot_token", Label: "Bot Token", Type: "password", Required: true},
				&pb.WizardField{Key: "chat_id", Label: "Chat ID", Type: "text", Required: true},
			},
		},
	}
}

type TeamsIntegration struct{}

func (s *TeamsIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "teams",
		Name:        "Microsoft Teams",
		Type:        string(IntegrationTypeTeams),
		Category:    string(CategoryChat),
		Description: "Deliver agent updates and approval requests to Microsoft Teams channels via webhooks.",
		Publisher:   "Microsoft",
		Icon:        "https://upload.wikimedia.org/wikipedia/commons/c/c9/Microsoft_Office_Teams_%282018%E2%80%93present%29.svg",
		Tags:        []string{"enterprise", "office365", "chat"}}
}
func (s *TeamsIntegration) WizardSteps() []*pb.WizardStep { return nil }

type WhatsAppIntegration struct{}

func (s *WhatsAppIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "whatsapp",
		Name:        "WhatsApp",
		Type:        string(IntegrationTypeWhatsApp),
		Category:    string(CategoryChat),
		Description: "Connect directly with users through WhatsApp messaging for consumer interactions and updates.",
		Publisher:   "Meta",
		Icon:        "https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg",
		Tags:        []string{"consumer", "messaging", "chat"}}
}
func (s *WhatsAppIntegration) WizardSteps() []*pb.WizardStep { return nil }

type IMessageIntegration struct{}

func (s *IMessageIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "imessage",
		Name:        "iMessage",
		Type:        string(IntegrationTypeIMessage),
		Category:    string(CategoryChat),
		Description: "Interact with users natively via Apple's iMessage interface.",
		Publisher:   "Apple",
		Icon:        "https://upload.wikimedia.org/wikipedia/commons/5/51/IMessage_logo.svg",
		Tags:        []string{"consumer", "apple", "chat"}}
}
func (s *IMessageIntegration) WizardSteps() []*pb.WizardStep { return nil }

type GitHubIntegration struct{}

func (s *GitHubIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "github",
		Name:        "GitHub",
		Type:        string(IntegrationTypeGitHub),
		Category:    string(CategoryGit),
		BaseUrl:     "https://github.com",
		Description: "Open pull requests, review code, and manage branches on GitHub.",
		Publisher:   "GitHub, Inc.",
		Icon:        "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png",
		Tags:        []string{"developer", "version-control", "git"}}
}
func (s *GitHubIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title: "Personal Access Token", Fields: []*pb.WizardField{
				&pb.WizardField{Key: "pat", Label: "Token", Type: "password", Required: true},
			},
		},
	}
}

type GitLabIntegration struct{}

func (s *GitLabIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "gitlab",
		Name:        "GitLab",
		Type:        string(IntegrationTypeGitLab),
		Category:    string(CategoryGit),
		BaseUrl:     "https://gitlab.com",
		Description: "Create merge requests and manage repositories on GitLab or self-hosted instances.",
		Publisher:   "GitLab Inc.",
		Icon:        "https://about.gitlab.com/images/press/logo/png/gitlab-icon-rgb.png",
		Tags:        []string{"developer", "version-control", "git", "self-hosted"}}
}
func (s *GitLabIntegration) WizardSteps() []*pb.WizardStep { return nil }

type GiteaIntegration struct{}

func (s *GiteaIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "gitea",
		Name:        "Gitea",
		Type:        string(IntegrationTypeGitea),
		Category:    string(CategoryGit),
		Description: "Open PRs on a self-hosted Gitea instance — the zero-lock-in OSS git option.",
		Publisher:   "The Gitea Authors",
		Icon:        "https://about.gitea.com/gitea.svg",
		Tags:        []string{"developer", "open-source", "git"}}
}
func (s *GiteaIntegration) WizardSteps() []*pb.WizardStep { return nil }

type JiraIntegration struct{}

func (s *JiraIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "jira",
		Name:        "Jira",
		Type:        string(IntegrationTypeJIRA),
		Category:    string(CategoryIssues),
		Description: "Create and manage issues, epics, and sprints in Atlassian Jira.",
		Publisher:   "Atlassian",
		Icon:        "https://wac-cdn.atlassian.com/dam/jcr:e3372c08-4107-4286-ab72-1b1e6aeb637a/logos-jira-icon-blue.svg",
		Tags:        []string{"enterprise", "issues", "agile"}}
}
func (s *JiraIntegration) WizardSteps() []*pb.WizardStep { return nil }

type PlaneIntegration struct{}

func (s *PlaneIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "plane",
		Name:        "Plane",
		Type:        string(IntegrationTypePlane),
		Category:    string(CategoryIssues),
		Description: "Manage issues and cycles with Plane — the open-source Jira alternative.",
		Publisher:   "MakePlane",
		Icon:        "https://plane.so/favicon.ico",
		Tags:        []string{"open-source", "issues", "project-management"}}
}
func (s *PlaneIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title: "Credentials", Fields: []*pb.WizardField{
				&pb.WizardField{Key: "api_key", Label: "API Key", Type: "password", Required: true},
				&pb.WizardField{Key: "workspace", Label: "Workspace Slug", Type: "text", Required: true},
			},
		},
	}
}

type GitHubIssuesIntegration struct{}

func (s *GitHubIssuesIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "github-issues",
		Name:        "GitHub Issues",
		Type:        string(IntegrationTypeGitHubIssues),
		Category:    string(CategoryIssues),
		Description: "Track tasks and bugs directly in GitHub Issues alongside your repositories.",
		Publisher:   "GitHub, Inc.",
		Icon:        "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png",
		Tags:        []string{"developer", "issues", "integrated"}}
}
func (s *GitHubIssuesIntegration) WizardSteps() []*pb.WizardStep { return nil }
