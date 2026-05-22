package local_stateful_execution_proxy

import (
	"context"
	"errors"
	"io"
	"testing"

	pb "github.com/onehumancorp/mono/src/proto/mcp_proxy"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func TestProxyClientExecuteShellCommand(t *testing.T) {
	client := NewProxyClient("localhost:9090", "spiffe://test", []string{"shell"})

	out, err := client.ExecuteShellCommand("echo 'hello proxy'")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if out != "hello proxy\n" {
		t.Fatalf("expected 'hello proxy\n', got %q", out)
	}
}

func TestProxyClientExecuteError(t *testing.T) {
	client := NewProxyClient("localhost:9090", "spiffe://test", []string{"shell"})

	out, err := client.ExecuteShellCommand("non_existent_command_123")
	if err == nil {
		t.Fatalf("expected error, got none. Output: %s", out)
	}
}

func TestEmitMetrics(t *testing.T) {
	emitMetrics(context.Background(), "test_metric", 1)
}

type mockStream struct {
	grpc.ClientStream
	recvCh chan *pb.ServerToProxy
	sendCh chan *pb.ProxyToServer
	sendErr error
	recvErr error
}

func (m *mockStream) Send(msg *pb.ProxyToServer) error {
	if m.sendErr != nil {
		return m.sendErr
	}
	if m.sendCh != nil {
		m.sendCh <- msg
	}
	return nil
}

func (m *mockStream) Recv() (*pb.ServerToProxy, error) {
	if m.recvErr != nil {
		return nil, m.recvErr
	}
	if m.recvCh != nil {
		msg, ok := <-m.recvCh
		if !ok {
			return nil, io.EOF
		}
		return msg, nil
	}
	return nil, io.EOF
}

type mockClient struct {
	stream pb.McpReverseTunnelService_EstablishTunnelClient
	establishErr error
}

func (m *mockClient) EstablishTunnel(ctx context.Context, opts ...grpc.CallOption) (pb.McpReverseTunnelService_EstablishTunnelClient, error) {
	if m.establishErr != nil {
		return nil, m.establishErr
	}
	return m.stream, nil
}

func TestServeStream_EstablishError(t *testing.T) {
	c := NewProxyClient("localhost", "spiffe", []string{})
	mc := &mockClient{establishErr: errors.New("establish error")}
	err := c.ServeStream(context.Background(), mc)
	if err == nil || err.Error() != "failed to establish tunnel: establish error" {
		t.Fatalf("unexpected err: %v", err)
	}
}

func TestServeStream_RegisterError(t *testing.T) {
	c := NewProxyClient("localhost", "spiffe", []string{})
	ms := &mockStream{sendErr: errors.New("send error")}
	mc := &mockClient{stream: ms}
	err := c.ServeStream(context.Background(), mc)
	if err == nil || err.Error() != "failed to send registration: send error" {
		t.Fatalf("unexpected err: %v", err)
	}
}

func TestServeStream_RecvError(t *testing.T) {
	c := NewProxyClient("localhost", "spiffe", []string{})
	ms := &mockStream{recvErr: errors.New("recv error")}
	mc := &mockClient{stream: ms}
	err := c.ServeStream(context.Background(), mc)
	if err == nil || err.Error() != "error receiving from stream: recv error" {
		t.Fatalf("unexpected err: %v", err)
	}
}

func TestServeStream_SuccessShell(t *testing.T) {
	c := NewProxyClient("localhost", "spiffe", []string{})
	recvCh := make(chan *pb.ServerToProxy, 1)
	sendCh := make(chan *pb.ProxyToServer, 2)

	recvCh <- &pb.ServerToProxy{
		RequestId: "req1",
		Payload: &pb.ServerToProxy_InvokeRequest{
			InvokeRequest: &pb.InvokeCommandRequest{
				ToolId: "shell",
				Params: "echo ok",
			},
		},
	}
	close(recvCh)

	ms := &mockStream{recvCh: recvCh, sendCh: sendCh}
	mc := &mockClient{stream: ms}

	err := c.ServeStream(context.Background(), mc)
	if err != nil {
		t.Fatalf("unexpected err: %v", err)
	}

	// Read registration
	<-sendCh

	// Read response
	resp := <-sendCh
	if resp.RequestId != "req1" || !resp.GetInvokeResponse().Success || resp.GetInvokeResponse().Result != "ok\n" {
		t.Fatalf("unexpected response: %v", resp)
	}
}

func TestServeStream_ErrorShell(t *testing.T) {
	c := NewProxyClient("localhost", "spiffe", []string{})
	recvCh := make(chan *pb.ServerToProxy, 1)
	sendCh := make(chan *pb.ProxyToServer, 2)

	recvCh <- &pb.ServerToProxy{
		RequestId: "req2",
		Payload: &pb.ServerToProxy_InvokeRequest{
			InvokeRequest: &pb.InvokeCommandRequest{
				ToolId: "shell",
				Params: "non_existent_command_xyz",
			},
		},
	}
	close(recvCh)

	ms := &mockStream{recvCh: recvCh, sendCh: sendCh}
	mc := &mockClient{stream: ms}

	err := c.ServeStream(context.Background(), mc)
	if err != nil {
		t.Fatalf("unexpected err: %v", err)
	}

	// Read registration
	<-sendCh

	// Read response
	resp := <-sendCh
	if resp.RequestId != "req2" || resp.GetInvokeResponse().Success || resp.GetInvokeResponse().ErrorDetails == "" {
		t.Fatalf("unexpected response: %v", resp)
	}
}

func TestServeStream_NonShellCommand(t *testing.T) {
	c := NewProxyClient("localhost", "spiffe", []string{})
	recvCh := make(chan *pb.ServerToProxy, 1)
	sendCh := make(chan *pb.ProxyToServer, 2)

	recvCh <- &pb.ServerToProxy{
		RequestId: "req4",
		Payload: &pb.ServerToProxy_InvokeRequest{
			InvokeRequest: &pb.InvokeCommandRequest{
				ToolId: "fs_read",
				Params: "file.txt",
			},
		},
	}
	close(recvCh)

	ms := &mockStream{recvCh: recvCh, sendCh: sendCh}
	mc := &mockClient{stream: ms}

	err := c.ServeStream(context.Background(), mc)
	if err != nil {
		t.Fatalf("unexpected err: %v", err)
	}
}

type customSendStream struct {
	grpc.ClientStream
	recvCh chan *pb.ServerToProxy
	sendFunc func(*pb.ProxyToServer) error
}

func (m *customSendStream) Send(msg *pb.ProxyToServer) error {
	return m.sendFunc(msg)
}

func (m *customSendStream) Recv() (*pb.ServerToProxy, error) {
	msg, ok := <-m.recvCh
	if !ok {
		return nil, io.EOF
	}
	return msg, nil
}

func TestServeStream_SendResponseError(t *testing.T) {
	c := NewProxyClient("localhost", "spiffe", []string{})
	recvCh := make(chan *pb.ServerToProxy, 1)

	recvCh <- &pb.ServerToProxy{
		RequestId: "req3",
		Payload: &pb.ServerToProxy_InvokeRequest{
			InvokeRequest: &pb.InvokeCommandRequest{
				ToolId: "shell",
				Params: "echo ok",
			},
		},
	}
	close(recvCh)

	callCount := 0
	sendFunc := func(msg *pb.ProxyToServer) error {
		callCount++
		if callCount > 1 {
			return errors.New("send error on response")
		}
		return nil
	}

	ms2 := &customSendStream{
		recvCh: recvCh,
		sendFunc: sendFunc,
	}

	mc := &mockClient{stream: ms2}

	err := c.ServeStream(context.Background(), mc)
	if err == nil || err.Error() != "failed to send response: send error on response" {
		t.Fatalf("unexpected err: %v", err)
	}
}

func TestServeStream_NilInvokeRequest(t *testing.T) {
	c := NewProxyClient("localhost", "spiffe", []string{})
	recvCh := make(chan *pb.ServerToProxy, 1)
	sendCh := make(chan *pb.ProxyToServer, 2)

	recvCh <- &pb.ServerToProxy{
		RequestId: "req5",
		// Payload is nil
	}
	close(recvCh)

	ms := &mockStream{recvCh: recvCh, sendCh: sendCh}
	mc := &mockClient{stream: ms}

	err := c.ServeStream(context.Background(), mc)
	if err != nil {
		t.Fatalf("unexpected err: %v", err)
	}
}

func TestConnectAndServeWrapper(t *testing.T) {
	// A dummy test calling ConnectAndServe with a real dummy connection to reach the client creation code
	c := NewProxyClient("localhost", "spiffe", []string{})
	conn, err := grpc.Dial("localhost:9090", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		t.Fatalf("failed to dial: %v", err)
	}
	// It will error immediately on connection attempt
	err = c.ConnectAndServe(context.Background(), conn)
	if err == nil {
		t.Fatalf("expected error from connecting to invalid server")
	}
}
