package builtin

type Role string
const (
	RoleUser Role = "user"
	RoleAssistant Role = "assistant"
	RoleSystem Role = "system"
)

type Message struct {
	Role Role
	Content string
}

type ChatRequest struct {
	System string
	Messages []Message
	MaxTokens int
}

type ChatResponse struct {
	Message Message
}
