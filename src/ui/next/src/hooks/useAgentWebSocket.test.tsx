import { renderHook, act, cleanup, waitFor } from '@testing-library/react';
import { afterEach, expect, test, vi } from 'vitest';
import { Blob } from 'node:buffer';
import { gzipSync } from 'node:zlib';
import { DecompressionStream } from 'node:stream/web';
import { useAgentWebSocket } from './useAgentWebSocket';
let socket: any;
const constructor = vi.fn();
function installSocket() {
  vi.stubGlobal('WebSocket', class {
    onmessage: any;
    onclose: any;
    close() {}
    constructor(...args: any[]) { constructor(...args); socket = this; }
  });
}
afterEach(() => { cleanup(); vi.unstubAllGlobals(); vi.restoreAllMocks(); constructor.mockClear(); });
test('negotiates gzip and delivers compressed and text frames in wire order', async () => {
  installSocket();
  vi.stubGlobal('DecompressionStream', DecompressionStream);
  vi.stubGlobal('Blob', Blob);
  const onMessage = vi.fn();
  renderHook(() => useAgentWebSocket({url: 'ws://localhost/ws', onMessage}));
  expect(constructor).toHaveBeenCalledWith('ws://localhost/ws', ['omnisolo.gzip.v1']);
  expect(socket.binaryType).toBe('arraybuffer');
  const bytes = gzipSync(JSON.stringify({type: 'batch', items: [{seq: 1}, {seq: 2}]}));
  act(() => {
    socket.onmessage({data: new Uint8Array(bytes).buffer});
    socket.onmessage({data: '{"seq":3}'});
  });
  await waitFor(() => expect(onMessage.mock.calls).toEqual([[{seq: 1}], [{seq: 2}], [{seq: 3}]]));
});
test('legacy browsers use text without advertising gzip', async () => {
  installSocket();
  vi.stubGlobal('DecompressionStream', undefined);
  const onMessage = vi.fn();
  renderHook(() => useAgentWebSocket({url: 'ws://localhost/ws', onMessage}));
  expect(constructor).toHaveBeenCalledWith('ws://localhost/ws');
  act(() => socket.onmessage({data: '{"seq":1}'}));
  await waitFor(() => expect(onMessage).toHaveBeenCalledWith({seq: 1}));
});
test('malformed compressed frames do not block later messages', async () => {
  installSocket();
  vi.stubGlobal('DecompressionStream', DecompressionStream);
  vi.stubGlobal('Blob', Blob);
  vi.spyOn(console, 'error').mockImplementation(() => {});
  const onMessage = vi.fn();
  renderHook(() => useAgentWebSocket({url: 'ws://localhost/ws', onMessage}));
  act(() => {
    socket.onmessage({data: new Uint8Array([1, 2, 3]).buffer});
    socket.onmessage({data: '{"seq":2}'});
  });
  await waitFor(() => expect(onMessage).toHaveBeenCalledWith({seq: 2}));
});

test('drops queued frames after the connection is disposed', async () => {
  installSocket();
  vi.stubGlobal('DecompressionStream', undefined);
  const onMessage = vi.fn();
  const {unmount} = renderHook(() => useAgentWebSocket({url: 'ws://localhost/ws', onMessage}));
  act(() => {
    socket.onmessage({data: '{"seq":1}'});
    unmount();
  });
  await act(async () => { await Promise.resolve(); });
  expect(onMessage).not.toHaveBeenCalled();
});

test('delivers already received frames when the socket closes during decompression', async () => {
  installSocket();
  vi.stubGlobal('DecompressionStream', DecompressionStream);
  vi.stubGlobal('Blob', Blob);
  const onMessage = vi.fn();
  renderHook(() => useAgentWebSocket({url: 'ws://localhost/ws', onMessage, reconnectInterval: 1}));
  const bytes = gzipSync(JSON.stringify({seq: 1}));
  act(() => {
    socket.onmessage({data: new Uint8Array(bytes).buffer});
    socket.onmessage({data: '{"seq":2}'});
    socket.onclose();
  });
  await waitFor(() => expect(constructor).toHaveBeenCalledTimes(2));
  act(() => socket.onmessage({data: '{"seq":3}'}));
  await waitFor(() => expect(onMessage.mock.calls).toEqual([[{seq: 1}], [{seq: 2}], [{seq: 3}]]));
});

test('discards queued frames when changing subscriptions, while delivering the new subscription', async () => {
  installSocket();
  vi.stubGlobal('DecompressionStream', undefined);
  const onMessage = vi.fn();
  const {rerender} = renderHook(({url}) => useAgentWebSocket({url, onMessage}), {initialProps: {url: 'ws://localhost/first'}});
  act(() => {
    socket.onmessage({data: '{"seq":1}'});
    rerender({url: 'ws://localhost/second'});
  });
  act(() => socket.onmessage({data: '{"seq":2}'}));
  await waitFor(() => expect(onMessage.mock.calls).toEqual([[{seq: 2}]]));
});
