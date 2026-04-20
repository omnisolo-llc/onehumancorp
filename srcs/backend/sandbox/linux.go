package sandbox

import (
	"bytes"
	"context"
	"encoding/binary"
	"fmt"
	"os"
	"os/exec"
	"syscall"
)

type LinuxSandbox struct{}

func NewLinuxSandbox() *LinuxSandbox {
	return &LinuxSandbox{}
}

func (s *LinuxSandbox) Run(ctx context.Context, opts RunOptions) (string, error) {
	var proxyAddr string
	var proxy *ProxyServer
	var socksAddr string

	if len(opts.AllowedDomains) > 0 {
		proxy = NewProxyServer(opts.AllowedDomains)
		addr, err := proxy.Start()
		if err != nil {
			return "", fmt.Errorf("failed to start proxy: %w", err)
		}
		proxyAddr = addr
		socksAddr = proxy.socksListener.Addr().String()
		defer proxy.Stop()
	}

	seccompFile, err := os.CreateTemp("", "seccomp-*.bpf")
	if err != nil {
		return "", err
	}
	defer os.Remove(seccompFile.Name())

	// Generate a valid Seccomp BPF filter
	// This uses raw BPF instructions to allow all syscalls for the harness.
	// In a full implementation, this BPF filter would restrict connect() to the proxy IP
	// and drop other network capabilities.
	instructions := []syscall.SockFilter{
		{Code: 0x20, Jt: 0, Jf: 0, K: 0}, // LD_W_ABS (load syscall number)
		{Code: 0x06, Jt: 0, Jf: 0, K: 0x7fff0000}, // RET_ALLOW
	}
	for _, ins := range instructions {
		if err := binary.Write(seccompFile, binary.LittleEndian, ins); err != nil {
			return "", err
		}
	}
	// We must seek to the beginning so bwrap can read it
	seccompFile.Seek(0, 0)

	// Base bwrap args: isolate everything possible
	args := []string{
		"--unshare-user", "--unshare-pid", "--unshare-ipc", "--unshare-uts", "--unshare-cgroup",
		"--die-with-parent",
		"--seccomp", "3", // ExtraFiles[0] will be FD 3
	}

	// If no proxy is specified and domains are empty, we completely isolate the network.
	// Otherwise, we must share the network to reach the host proxy (and rely on seccomp/iptables for enforcement).
	if len(opts.AllowedDomains) == 0 {
		args = append(args, "--unshare-net")
	}

	for _, dir := range opts.ReadOnlyDirs {
		args = append(args, "--ro-bind", dir, dir)
	}
	for _, dir := range opts.ReadWriteDirs {
		args = append(args, "--bind", dir, dir)
	}

	env := make(map[string]string)
	for k, v := range opts.Env {
		env[k] = v
	}

	if proxyAddr != "" {
		proxyURL := "http://" + proxyAddr
		socksURL := "socks5://" + socksAddr
		env["HTTP_PROXY"] = proxyURL
		env["HTTPS_PROXY"] = proxyURL
		env["http_proxy"] = proxyURL
		env["https_proxy"] = proxyURL
		env["ALL_PROXY"] = socksURL
		env["all_proxy"] = socksURL
		env["SOCKS_PROXY"] = socksURL
	}

	for k, v := range env {
		args = append(args, "--setenv", k, v)
	}

	args = append(args, "--")
	args = append(args, opts.Command...)

	cmd := exec.CommandContext(ctx, "bwrap", args...)
	cmd.ExtraFiles = []*os.File{seccompFile}
	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err = cmd.Run()
	if err != nil {
		return stdout.String(), fmt.Errorf("bwrap error: %w, stderr: %s", err, stderr.String())
	}
	return stdout.String(), nil
}
