package main

import (
	"log"
	"net"

	"google.golang.org/grpc"
	hub "github.com/onehumancorp/mono/src/proto/hub"
    "github.com/onehumancorp/mono/src/server/orchestration/mesh"
)

func main() {
	lis, err := net.Listen("tcp", ":8082")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	transport := mesh.NewMemoryMeshTransport()
	node := mesh.NewCentrifugeNode(transport)
	server := mesh.NewMeshServer(node)

	s := grpc.NewServer()
	hub.RegisterHubServiceServer(s, server)

	log.Printf("server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
