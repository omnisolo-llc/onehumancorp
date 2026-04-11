package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

)

func main() {
	fmt.Println("One Human Corp Server Starting...")

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	sigs := make(chan os.Signal, 1)
	signal.Notify(sigs, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		<-sigs
		fmt.Println("\nShutting down server...")
		cancel()
	}()

	select {
	case <-ctx.Done():
		log.Println("Server exited cleanly.")
	}
}
