package main

import (
	"fmt"
	"log"

	"github.com/onehumancorp/mono/srcs/agents"
)

func main() {
	// Initialize the registry with the default built-in providers
	registry := agents.DefaultRegistry()

	// Get the built-in provider
	provider, ok := registry.Get(agents.ProviderTypeBuiltin)
	if !ok {
		log.Fatal("Built-in provider not found")
	}

	fmt.Printf("Successfully loaded provider: %s\n", provider.Type())
	fmt.Printf("Description: %s\n", provider.Description())
	fmt.Printf("Is Authenticated: %t\n", provider.IsAuthenticated())

	// Create an agent or do something with the built in provider
	fmt.Println("\n✨ \033[1;32mHello World!\033[0m The agent provider is ready to use with zero configuration.")
	fmt.Println("   \033[3mOne Human Corp - Orchestrating Swarms with Zero Friction\033[0m\n")
}
