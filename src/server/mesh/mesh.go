package mesh

// TeammateMesh defines the realtime coordination layer for agents.
type TeammateMesh interface {
	Publish(channel string, message []byte) error
	Subscribe(channel string) (<-chan []byte, error)
}
