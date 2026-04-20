package sandbox

import (
	"context"
	"io"
	"net"
	"net/http"
	"net/http/httptest"
	"net/url"
	"os"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestProxyServer(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("OK"))
	}))
	defer ts.Close()

	tsURL, err := url.Parse(ts.URL)
	require.NoError(t, err)

	proxy := NewProxyServer([]string{tsURL.Hostname()})
	addr, err := proxy.Start()
	require.NoError(t, err)
	defer proxy.Stop()

	proxyURL, _ := url.Parse("http://" + addr)
	client := &http.Client{
		Transport: &http.Transport{
			Proxy: http.ProxyURL(proxyURL),
		},
	}

	t.Run("Allowed Domain", func(t *testing.T) {
		req, _ := http.NewRequest("GET", ts.URL, nil)
		resp, err := client.Do(req)
		require.NoError(t, err)
		defer resp.Body.Close()
		assert.Equal(t, http.StatusOK, resp.StatusCode)

		body, _ := io.ReadAll(resp.Body)
		assert.Equal(t, "OK", string(body))
	})

	t.Run("Blocked Domain", func(t *testing.T) {
		req, _ := http.NewRequest("GET", "http://blocked.com", nil)
		resp, err := client.Do(req)
		require.NoError(t, err)
		defer resp.Body.Close()
		assert.Equal(t, http.StatusForbidden, resp.StatusCode)
	})

	t.Run("HTTPS Connect Blocked Domain", func(t *testing.T) {
		conn, err := net.Dial("tcp", addr)
		require.NoError(t, err)
		defer conn.Close()

		_, err = conn.Write([]byte("CONNECT blocked.com:443 HTTP/1.1\r\nHost: blocked.com:443\r\n\r\n"))
		require.NoError(t, err)

		buf := make([]byte, 1024)
		n, _ := conn.Read(buf)
		response := string(buf[:n])
		assert.True(t, strings.HasPrefix(response, "HTTP/1.1 403"))
	})
}

func TestLinuxSandbox(t *testing.T) {
	if _, err := os.Stat("/usr/bin/bwrap"); os.IsNotExist(err) {
		t.Skip("bwrap not installed")
	}

	sandbox := NewLinuxSandbox()
	output, err := sandbox.Run(context.Background(), RunOptions{
		Command: []string{"echo", "hello"},
	})
	require.NoError(t, err)
	assert.Equal(t, "hello\n", output)
}

func TestMacOSSandbox(t *testing.T) {
	if _, err := os.Stat("/usr/bin/sandbox-exec"); os.IsNotExist(err) {
		t.Skip("sandbox-exec not installed")
	}

	sandbox := NewMacOSSandbox()
	output, err := sandbox.Run(context.Background(), RunOptions{
		Command: []string{"echo", "hello"},
	})
	require.NoError(t, err)
	assert.Equal(t, "hello\n", output)
}

func TestProxyServerNoDomains(t *testing.T) {
	proxy := NewProxyServer([]string{})
	addr, err := proxy.Start()
	require.NoError(t, err)
	defer proxy.Stop()

	proxyURL, _ := url.Parse("http://" + addr)
	client := &http.Client{
		Transport: &http.Transport{
			Proxy: http.ProxyURL(proxyURL),
		},
	}

	req, _ := http.NewRequest("GET", "http://example.com", nil)
	resp, err := client.Do(req)
	require.NoError(t, err)
	defer resp.Body.Close()
	assert.Equal(t, http.StatusForbidden, resp.StatusCode)
}

func TestProxyServerHTTPSAllowed(t *testing.T) {
	proxy := NewProxyServer([]string{"example.com"})
	addr, err := proxy.Start()
	require.NoError(t, err)
	defer proxy.Stop()

	conn, err := net.Dial("tcp", addr)
	require.NoError(t, err)
	defer conn.Close()

	_, err = conn.Write([]byte("CONNECT example.com:443 HTTP/1.1\r\nHost: example.com:443\r\n\r\n"))
	require.NoError(t, err)

	buf := make([]byte, 1024)
	n, _ := conn.Read(buf)
	response := string(buf[:n])

	assert.False(t, strings.HasPrefix(response, "HTTP/1.1 403"))
}

func TestSOCKS5Proxy(t *testing.T) {
	proxy := NewProxyServer([]string{"example.com"})
	_, err := proxy.Start()
	require.NoError(t, err)
	defer proxy.Stop()

	// Start SOCKS testing
	socksAddr := proxy.socksListener.Addr().String()
	conn, err := net.Dial("tcp", socksAddr)
	require.NoError(t, err)
	defer conn.Close()

	// Initial handshake
	_, err = conn.Write([]byte{0x05, 0x01, 0x00})
	require.NoError(t, err)

	buf := make([]byte, 2)
	_, err = io.ReadFull(conn, buf)
	require.NoError(t, err)
	assert.Equal(t, []byte{0x05, 0x00}, buf)

	// CONNECT request to blocked domain
	domain := "blocked.com"
	req := []byte{0x05, 0x01, 0x00, 0x03, byte(len(domain))}
	req = append(req, []byte(domain)...)
	req = append(req, []byte{0x01, 0xbb}...) // port 443

	_, err = conn.Write(req)
	require.NoError(t, err)

	resp := make([]byte, 10)
	_, err = io.ReadFull(conn, resp)
	require.NoError(t, err)
	// 0x02 is connection not allowed by ruleset
	assert.Equal(t, byte(0x02), resp[1])
}
