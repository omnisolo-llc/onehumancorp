package sandbox

import (
	"bytes"
	"context"
	"fmt"
	"os"
	"os/exec"
	"strings"
)

type MacOSSandbox struct{}

func NewMacOSSandbox() *MacOSSandbox {
	return &MacOSSandbox{}
}

func escapePath(path string) string {
	return strings.ReplaceAll(path, "\"", "\\\"")
}

func (s *MacOSSandbox) Run(ctx context.Context, opts RunOptions) (string, error) {
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

	profile := "(version 1)\n(deny default)\n"
	for _, dir := range opts.ReadOnlyDirs {
		profile += fmt.Sprintf("(allow file-read* (subpath \"%s\"))\n", escapePath(dir))
	}
	for _, dir := range opts.ReadWriteDirs {
		profile += fmt.Sprintf("(allow file-read* file-write* (subpath \"%s\"))\n", escapePath(dir))
	}
	profile += "(allow process-exec*)\n"

	// Network access rules
	if proxyAddr != "" {
		profile += "(deny network*)\n"
		profile += "(allow network-outbound (remote tcp \"localhost:*\" ))\n"
		profile += "(allow network-outbound (remote tcp \"127.0.0.1:*\" ))\n"
	} else if len(opts.AllowedDomains) == 0 {
		profile += "(deny network*)\n"
	}

	profileFile, err := os.CreateTemp("", "sandbox-profile-*.sb")
	if err != nil {
		return "", err
	}
	defer os.Remove(profileFile.Name())

	if _, err := profileFile.WriteString(profile); err != nil {
		return "", err
	}
	profileFile.Close()

	cmd := exec.CommandContext(ctx, "sandbox-exec", "-f", profileFile.Name())
	cmd.Args = append(cmd.Args, opts.Command...)

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
		cmd.Env = append(cmd.Env, fmt.Sprintf("%s=%s", k, v))
	}

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err = cmd.Run()
	if err != nil {
		return stdout.String(), fmt.Errorf("sandbox-exec error: %w, stderr: %s", err, stderr.String())
	}
	return stdout.String(), nil
}
