package builtin

type Role string

const (
	RoleUser Role = "user"
)

type Message struct {
	Role    Role
	Content string
}

type ChatRequest struct {
	System    string
	Messages  []Message
	MaxTokens int
}

type ChatResponse struct {
	Message Message
}
