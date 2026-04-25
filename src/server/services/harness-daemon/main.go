package main

import (
	"flag"
	"log/slog"
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
		slog.Error("Failed to start daemon", "error", err)
		os.Exit(1)
	}

	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, os.Interrupt, syscall.SIGTERM)

	slog.Info("Harness Daemon is running. Press Ctrl+C to stop.")
	<-sigCh

	slog.Info("Shutting down daemon...")
	if err := daemon.Stop(); err != nil {
		slog.Error("Error stopping daemon", "error", err)
	}
}
