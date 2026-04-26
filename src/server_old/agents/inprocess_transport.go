package agents

import (
	"bufio"
	"bytes"
	"context"
	"errors"
	"io"
	"os"
	"sync"
)

// InProcessTransport wraps io.Reader and io.Writer for local, zero-latency execution.
// It implements the Transport interface.
type InProcessTransport struct {
	reader io.Reader
	writer io.Writer

	msgChan chan []byte
	errChan chan error

	closeOnce sync.Once
	sendMu    sync.Mutex
	ctx       context.Context
	cancel    context.CancelFunc
	done      chan struct{}
}

// NewInProcessTransport creates a new InProcessTransport.
// If reader or writer is nil, it defaults to os.Stdin and os.Stdout respectively.
func NewInProcessTransport(reader io.Reader, writer io.Writer) *InProcessTransport {
	if reader == nil {
		reader = os.Stdin
	}
	if writer == nil {
		writer = os.Stdout
	}

	ctx, cancel := context.WithCancel(context.Background())
	t := &InProcessTransport{
		reader:  reader,
		writer:  writer,
		msgChan: make(chan []byte),
		errChan: make(chan error, 1),
		ctx:     ctx,
		cancel:  cancel,
		done:    make(chan struct{}),
	}

	go t.readLoop()
	return t
}

func (t *InProcessTransport) readLoop() {
	defer close(t.done)
	bufReader := bufio.NewReader(t.reader)

	for {
		// ReadBytes blocks until it finds a newline or an error occurs.
		line, err := bufReader.ReadBytes('\n')

		if len(line) > 0 {
			// Trim the newline
			trimmed := bytes.TrimRight(line, "\r\n")

			select {
			case <-t.ctx.Done():
				return
			case t.msgChan <- trimmed:
			}
		}

		if err != nil {
			select {
			case <-t.ctx.Done():
				return
			case t.errChan <- err:
			}
			return
		}
	}
}

// Send writes the message to the writer followed by a newline.
func (t *InProcessTransport) Send(ctx context.Context, message []byte) error {
	// Check context
	if err := ctx.Err(); err != nil {
		return err
	}

	t.sendMu.Lock()
	defer t.sendMu.Unlock()

	// We must not mutate the caller's slice by appending to it directly,
	// as it might overwrite data if the slice has extra capacity.
	_, err := t.writer.Write(message)
	if err != nil {
		return err
	}
	_, err = t.writer.Write([]byte{'\n'})
	return err
}

// Receive reads a line from the reader.
func (t *InProcessTransport) Receive(ctx context.Context) ([]byte, error) {
	select {
	case <-ctx.Done():
		return nil, ctx.Err()
	case <-t.ctx.Done():
		return nil, t.ctx.Err()
	case err := <-t.errChan:
		return nil, err
	case msg := <-t.msgChan:
		return msg, nil
	}
}

// Close closes the underlying reader and writer if they implement io.Closer.
func (t *InProcessTransport) Close() error {
	t.closeOnce.Do(func() {
		t.cancel()
	})

	var errs []error

	if closer, ok := t.reader.(io.Closer); ok {
		if err := closer.Close(); err != nil {
			errs = append(errs, err)
		}
	}

	if closer, ok := t.writer.(io.Closer); ok && t.writer != (any)(t.reader) { // Avoid closing same fd twice
		if err := closer.Close(); err != nil {
			errs = append(errs, err)
		}
	}

	// Wait for the read loop to exit gracefully (or not if the reader is blocked, but we won't block forever here)
	// We can't actually wait on t.done because if the reader is blocked on reading, it will never exit.
	// But closing the reader above should unblock it if it's a pipe or socket.
	// We just continue.

	if len(errs) > 0 {
		return errors.Join(errs...)
	}

	return nil
}
