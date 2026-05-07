package harness

import (
	"bufio"
	"context"
	"io"
)

// LocalTransport wraps OS-level stdio mapping
type LocalTransport struct {
	reader *bufio.Reader
	writer io.Writer
}

func NewLocalTransport(stdin io.Reader, stdout io.Writer) *LocalTransport {
	return &LocalTransport{
		reader: bufio.NewReader(stdin),
		writer: stdout,
	}
}

func (t *LocalTransport) Send(ctx context.Context, message []byte) error {
	_, err := t.writer.Write(append(message, '\n'))
	return err
}

func (t *LocalTransport) Receive(ctx context.Context) ([]byte, error) {
	line, err := t.reader.ReadBytes('\n')
	if err != nil {
		return nil, err
	}
	return line, nil
}

func (t *LocalTransport) Close() error {
	return nil
}
