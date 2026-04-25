package main

import (
	"flag"
	"log"
	"os"
	"os/signal"
	"syscall"

	harnessdaemon "github.com/onehumancorp/mono/src/server/services/harness-daemon"
)

func main() {
	port := flag.Int("port", 3000, "Port for the daemon to listen on")
	flag.Parse()

	daemon := harnessdaemon.NewDaemon(*port)
	if err := daemon.Start(); err != nil {
		log.Fatalf("Failed to start daemon: %v", err)
	}

	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, os.Interrupt, syscall.SIGTERM)

	log.Printf("Harness Daemon is running. Press Ctrl+C to stop.")
	<-sigCh

	log.Println("Shutting down daemon...")
	if err := daemon.Stop(); err != nil {
		log.Printf("Error stopping daemon: %v", err)
	}
}
