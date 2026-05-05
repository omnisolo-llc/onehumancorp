package main

import (
	"log"
	"net/http"

	"ohc/srcs/server/orchestration"
)

type DummyAuthStore struct{}

import_os "os"

func (s *DummyAuthStore) ValidateToken(token string) bool {
	expected := import_os.Getenv("MESH_API_SECRET")
	if expected == "" {
		return false
	}
	return token == expected
}

func main() {
	mesh, err := orchestration.NewTeammateMesh("redis://localhost:6379")
	if err != nil {
		log.Fatalf("Failed to initialize mesh: %v", err)
	}

	mux := http.NewServeMux()
	authStore := &DummyAuthStore{}
	orchestration.RegisterRoutes(mux, mesh, authStore)

	log.Println("Starting Teammate Mesh Go Server on :8081")
	log.Fatal(http.ListenAndServe(":8081", mux))
}
