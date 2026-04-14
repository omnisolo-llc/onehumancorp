package chat

import (
	"context"
	"fmt"

	"github.com/slack-go/slack"
)

// SlackClient sends messages using the official slack-go/slack SDK.
type SlackClient struct {
	api *slack.Client
}

// NewSlackClient creates a Slack messenger from a bot token.
func NewSlackClient(token string) (*SlackClient, error) {
	if token == "" {
		return nil, fmt.Errorf("slack token is required")
	}
	return &SlackClient{api: slack.New(token)}, nil
}

// Send posts a plain text message to the given Slack channel and returns its timestamp ID.
func (c *SlackClient) Send(ctx context.Context, channelID, text string) (string, error) {
	if c == nil || c.api == nil {
		return "", fmt.Errorf("slack client is not initialized")
	}
	if channelID == "" {
		return "", fmt.Errorf("channel ID is required")
	}
	_, ts, err := c.api.PostMessageContext(ctx, channelID, slack.MsgOptionText(text, false))
	return ts, err
}
