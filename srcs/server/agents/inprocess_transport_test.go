package agents

import (
	"bytes"
	"context"
	"io"
	"os"
	"testing"
	"time"
)

// mockReadCloser is a mock reader that implements io.ReadCloser
type mockReadCloser struct {
	*bytes.Buffer
	closed bool
}

func (m *mockReadCloser) Close() error {
	m.closed = true
	return nil
}

// mockWriteCloser is a mock writer that implements io.WriteCloser
type mockWriteCloser struct {
	*bytes.Buffer
	closed bool
}

func (m *mockWriteCloser) Close() error {
	m.closed = true
	return nil
}

// mockErrorReader always returns an error when reading
type mockErrorReader struct {
	err error
}

func (m *mockErrorReader) Read(p []byte) (n int, err error) {
	return 0, m.err
}

func TestNewInProcessTransport_Defaults(t *testing.T) {
	transport := NewInProcessTransport(nil, nil)
	if transport.reader != os.Stdin {
		t.Errorf("Expected reader to default to os.Stdin, got %v", transport.reader)
	}
	if transport.writer != os.Stdout {
		t.Errorf("Expected writer to default to os.Stdout, got %v", transport.writer)
	}
}

func TestInProcessTransport_Send(t *testing.T) {
	buf := new(bytes.Buffer)
	transport := NewInProcessTransport(nil, buf)

	ctx := context.Background()
	message := []byte(`{"event":"task","data":"do something"}`)

	err := transport.Send(ctx, message)
	if err != nil {
		t.Fatalf("Unexpected error from Send: %v", err)
	}

	expected := string(message) + "\n"
	if buf.String() != expected {
		t.Errorf("Expected buffer to contain %q, got %q", expected, buf.String())
	}
}

func TestInProcessTransport_Send_ContextCancelled(t *testing.T) {
	buf := new(bytes.Buffer)
	transport := NewInProcessTransport(nil, buf)

	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	err := transport.Send(ctx, []byte("test"))
	if err == nil {
		t.Error("Expected error from Send with cancelled context, got nil")
	}
	if err != context.Canceled {
		t.Errorf("Expected context.Canceled, got %v", err)
	}
}

func TestInProcessTransport_Receive(t *testing.T) {
	buf := bytes.NewBufferString("message 1\nmessage 2\r\nmessage 3")
	transport := NewInProcessTransport(buf, nil)

	ctx := context.Background()

	// Read first line
	msg1, err := transport.Receive(ctx)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if string(msg1) != "message 1" {
		t.Errorf("Expected 'message 1', got %q", string(msg1))
	}

	// Read second line (should trim \r as well)
	msg2, err := transport.Receive(ctx)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if string(msg2) != "message 2" {
		t.Errorf("Expected 'message 2', got %q", string(msg2))
	}

	// Read third line (no newline at EOF)
	msg3, err := transport.Receive(ctx)
	if err != nil && err != io.EOF {
		t.Errorf("Expected nil or io.EOF, got %v", err)
	}
	if string(msg3) != "message 3" {
		t.Errorf("Expected 'message 3', got %q", string(msg3))
	}
}

func TestInProcessTransport_Receive_ContextCancelled(t *testing.T) {
	// A reader that never returns
	reader, writer := io.Pipe()
	defer reader.Close()
	defer writer.Close()

	transport := NewInProcessTransport(reader, nil)

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()

	_, err := transport.Receive(ctx)
	if err == nil {
		t.Error("Expected error from Receive with timeout, got nil")
	}
	if err != context.DeadlineExceeded {
		t.Errorf("Expected context.DeadlineExceeded, got %v", err)
	}
}

func TestInProcessTransport_Receive_Error(t *testing.T) {
	expectedErr := io.ErrUnexpectedEOF
	reader := &mockErrorReader{err: expectedErr}
	transport := NewInProcessTransport(reader, nil)

	ctx := context.Background()
	_, err := transport.Receive(ctx)
	if err != expectedErr {
		t.Errorf("Expected %v, got %v", expectedErr, err)
	}
}

func TestInProcessTransport_Close(t *testing.T) {
	r := &mockReadCloser{Buffer: new(bytes.Buffer)}
	w := &mockWriteCloser{Buffer: new(bytes.Buffer)}

	transport := NewInProcessTransport(r, w)

	err := transport.Close()
	if err != nil {
		t.Fatalf("Unexpected error from Close: %v", err)
	}

	if !r.closed {
		t.Error("Expected reader to be closed")
	}
	if !w.closed {
		t.Error("Expected writer to be closed")
	}
}

func TestInProcessTransport_Close_SameUnderlying(t *testing.T) {
	// Close should only be called once if they are the exact same object
	m := &mockReadCloser{Buffer: new(bytes.Buffer)}

	transport := NewInProcessTransport(m, m)

	err := transport.Close()
	if err != nil {
		t.Fatalf("Unexpected error from Close: %v", err)
	}

	if !m.closed {
		t.Error("Expected closer to be called")
	}
}

func TestInProcessTransport_Send_WriteError(t *testing.T) {
	expectedErr := io.ErrClosedPipe
	errWriter := &mockErrorWriter{err: expectedErr}
	transport := NewInProcessTransport(nil, errWriter)

	err := transport.Send(context.Background(), []byte("test"))
	if err != expectedErr {
		t.Errorf("Expected %v, got %v", expectedErr, err)
	}
}

type mockErrorWriter struct {
	err error
}

func (m *mockErrorWriter) Write(p []byte) (n int, err error) {
	return 0, m.err
}

func TestInProcessTransport_Receive_TransportContextCancelled(t *testing.T) {
	transport := NewInProcessTransport(nil, nil)
	transport.cancel() // cancel transport context

	_, err := transport.Receive(context.Background())
	if err != context.Canceled {
		t.Errorf("Expected context.Canceled, got %v", err)
	}
}

func TestInProcessTransport_Close_WithErrors(t *testing.T) {
	expectedErr := io.ErrClosedPipe
	r := &mockErrorCloser{err: expectedErr}
	w := &mockErrorCloser{err: expectedErr}

	transport := NewInProcessTransport(r, w)

	err := transport.Close()
	if err == nil {
		t.Errorf("Expected error, got nil")
	}
}

type mockErrorCloser struct {
	err error
}

func (m *mockErrorCloser) Read(p []byte) (n int, err error) {
	return 0, io.EOF
}

func (m *mockErrorCloser) Write(p []byte) (n int, err error) {
	return len(p), nil
}

func (m *mockErrorCloser) Close() error {
	return m.err
}

func TestInProcessTransport_ReadLoop_ContextCancelledWhenMsgRead(t *testing.T) {
	// Let's pass a message to readLoop but cancel the context at the same time
	buf := bytes.NewBufferString("msg\n")
	transport := NewInProcessTransport(buf, nil)

	// We want to cancel context so that the select hits `case <-t.ctx.Done():`
	// Since readLoop is a goroutine, it's hard to precisely hit the race condition.
	// But simply cancelling it right away and not reading from msgChan might do it
	// or we can test by sending a lot of data and not reading.
	transport.cancel()

	// Sleep a bit to allow readLoop to process
	time.Sleep(10 * time.Millisecond)
}

func TestInProcessTransport_Send_NewlineWriteError(t *testing.T) {
	expectedErr := io.ErrClosedPipe
	errWriter := &mockFailOnNewlineWriter{err: expectedErr}
	transport := NewInProcessTransport(nil, errWriter)

	err := transport.Send(context.Background(), []byte("test"))
	if err != expectedErr {
		t.Errorf("Expected %v, got %v", expectedErr, err)
	}
}

type mockFailOnNewlineWriter struct {
	err error
	writes int
}

func (m *mockFailOnNewlineWriter) Write(p []byte) (n int, err error) {
	if m.writes == 1 {
		return 0, m.err
	}
	m.writes++
	return len(p), nil
}

func TestInProcessTransport_ReadLoop_ReadError_ContextCancelled(t *testing.T) {
	reader := &mockErrorReader{err: io.ErrUnexpectedEOF}
	transport := NewInProcessTransport(reader, nil)

	transport.cancel() // cancel context before readLoop hits the error select

	time.Sleep(10 * time.Millisecond)
}
