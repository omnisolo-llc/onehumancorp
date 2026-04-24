package harness

import (
    "bufio"
    "bytes"
    "context"
    "errors"
    "io"
    "sync"
)

type LocalTransport struct {
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

func NewLocalTransport(reader io.Reader, writer io.Writer) *LocalTransport {
    ctx, cancel := context.WithCancel(context.Background())
    t := &LocalTransport{
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

func (t *LocalTransport) readLoop() {
    defer close(t.done)
    bufReader := bufio.NewReader(t.reader)
    for {
        line, err := bufReader.ReadBytes('\n')
        if len(line) > 0 {
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

func (t *LocalTransport) Send(ctx context.Context, message []byte) error {
    if err := ctx.Err(); err != nil {
        return err
    }
    t.sendMu.Lock()
    defer t.sendMu.Unlock()

    if _, err := t.writer.Write(message); err != nil {
        return err
    }
    _, err := t.writer.Write([]byte{'\n'})
    return err
}

func (t *LocalTransport) Receive(ctx context.Context) ([]byte, error) {
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

func (t *LocalTransport) Close() error {
    t.closeOnce.Do(func() { t.cancel() })
    var errs []error
    if closer, ok := t.reader.(io.Closer); ok {
        if err := closer.Close(); err != nil {
            errs = append(errs, err)
        }
    }
    if closer, ok := t.writer.(io.Closer); ok && t.writer != (any)(t.reader) {
        if err := closer.Close(); err != nil {
            errs = append(errs, err)
        }
    }
    if len(errs) > 0 {
        return errors.Join(errs...)
    }
    return nil
}
