package network

import (
	"context"
	"fmt"
	"os"
	"os/exec"
)

type NetworkBridge struct {
	Proxy      *NetworkBridgeProxy
	SocketPath string
	socatCmd   *exec.Cmd
}

func NewNetworkBridge(socketPath string, allowedDomains []string) *NetworkBridge {
	return &NetworkBridge{
		Proxy:      NewNetworkBridgeProxy(allowedDomains),
		SocketPath: socketPath,
	}
}

func (b *NetworkBridge) Start() error {
	if err := b.Proxy.Start(); err != nil {
		return err
	}

	os.Remove(b.SocketPath)

	b.socatCmd = exec.Command("socat", fmt.Sprintf("UNIX-LISTEN:%s,fork", b.SocketPath), fmt.Sprintf("TCP:127.0.0.1:%d", b.Proxy.Port))
	if err := b.socatCmd.Start(); err != nil {
		b.Proxy.Stop(context.Background())
		return err
	}

	return nil
}

func (b *NetworkBridge) Stop() error {
	if b.socatCmd != nil && b.socatCmd.Process != nil {
		b.socatCmd.Process.Kill()
	}
	os.Remove(b.SocketPath)
	return b.Proxy.Stop(context.Background())
}
