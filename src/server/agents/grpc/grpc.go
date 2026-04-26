package agentgrpc

type Client interface {}
func NewClient(addr string) Client { return nil }
func ClientOptionsFromEnv() []interface{} { return nil }
