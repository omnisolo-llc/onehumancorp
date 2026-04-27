package grpc
type Provider struct {}
func NewProvider(target string) (*Provider, error) { return &Provider{}, nil }
