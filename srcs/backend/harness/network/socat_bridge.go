package network

import (
    "context"
    "fmt"
    "os"
    "os/exec"
)

type SocatBridge struct {
    HostSocketPath string
    ProxyPort      int
    cmd            *exec.Cmd
}

func NewSocatBridge(proxyPort int) (*SocatBridge, error) {
    tmpFile, err := os.CreateTemp("", "ohc-agent-http-*.sock")
    if err != nil {
        return nil, err
    }
    // We only need the name, so remove the file so socat can create it as a socket
    tmpFile.Close()
    os.Remove(tmpFile.Name())

    return &SocatBridge{
        HostSocketPath: tmpFile.Name(),
        ProxyPort:      proxyPort,
    }, nil
}

func (s *SocatBridge) Start(ctx context.Context) error {
    // Start socat on the host: listening on UNIX socket, forwarding to TCP:127.0.0.1:<proxyPort>
    s.cmd = exec.CommandContext(ctx, "socat",
        fmt.Sprintf("UNIX-LISTEN:%s,fork", s.HostSocketPath),
        fmt.Sprintf("TCP:127.0.0.1:%d", s.ProxyPort),
    )
    err := s.cmd.Start()
    if err == nil {
        go func() {
            _ = s.cmd.Wait()
        }()
    }
    return err
}

func (s *SocatBridge) Stop() error {
    if s.cmd != nil && s.cmd.Process != nil {
        s.cmd.Process.Kill()
    }
    return os.RemoveAll(s.HostSocketPath)
}
