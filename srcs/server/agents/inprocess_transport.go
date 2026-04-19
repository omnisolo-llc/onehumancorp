package agents

import (
	"bufio"
	"encoding/json"
	"io"
	"sync"
)

type InProcessTransport struct {
	reader io.Reader
	writer io.Writer
	mu     sync.Mutex
	done   chan struct{}
}

func NewInProcessTransport(r io.Reader, w io.Writer) *InProcessTransport {
	return &InProcessTransport{
		reader: r,
		writer: w,
		done:   make(chan struct{}),
	}
}

func (t *InProcessTransport) Send(channel string, msg *Message) error {
	t.mu.Lock()
	defer t.mu.Unlock()
	b, err := json.Marshal(msg)
	if err != nil {
		return err
	}
	b = append(b, '\n')
	_, err = t.writer.Write(b)
	return err
}

func (t *InProcessTransport) Receive(channel string) (<-chan *Message, error) {
	ch := make(chan *Message)

	// Create a channel for lines to avoid blocking on scanner.Scan() directly
	// in the main select loop.
	lines := make(chan []byte)
	go func() {
		scanner := bufio.NewScanner(t.reader)
		for scanner.Scan() {
			// Copy the slice since scanner bytes are reused
			b := make([]byte, len(scanner.Bytes()))
			copy(b, scanner.Bytes())

			select {
			case lines <- b:
			case <-t.done:
				return
			}
		}
		close(lines)
	}()

	go func() {
		defer close(ch)
		for {
			select {
			case <-t.done:
				return
			case line, ok := <-lines:
				if !ok {
					return
				}
				var msg Message
				if err := json.Unmarshal(line, &msg); err == nil {
					select {
					case ch <- &msg:
					case <-t.done:
						return
					}
				}
			}
		}
	}()
	return ch, nil
}

func (t *InProcessTransport) Close() error {
	close(t.done)
	return nil
}
