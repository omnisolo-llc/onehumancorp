package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/onehumancorp/mono/services/billing"
)

func main() {
	fmt.Println("Starting Billing Application")

	svc := billing.NewBillingService()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Simulate processing usage
	err := svc.ProcessUsage(ctx, 2000, 1000)
	if err != nil {
		log.Fatalf("Error processing usage: %v", err)
	}

	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)

	fmt.Println("Billing Application running. Press Ctrl+C to exit.")
	<-sigCh
	fmt.Println("Shutting down Billing Application")
}
