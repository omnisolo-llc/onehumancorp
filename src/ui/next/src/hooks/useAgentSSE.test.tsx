import { renderHook, act, cleanup } from '@testing-library/react';
import { afterEach, expect, test, vi } from 'vitest';
import { useAgentSSE } from './useAgentSSE';

afterEach(() => { cleanup(); vi.unstubAllGlobals(); });
test('connects to the server stream route and receives named chat events', () => {
  const listeners = new Map<string, (event: {data: string}) => void>();
  const constructor = vi.fn();
  vi.stubGlobal('EventSource', class {
    constructor(url: string) { constructor(url); }
    addEventListener(name: string, callback: (event: {data: string}) => void) { listeners.set(name, callback); }
    close() {}
  });
  const onMessage = vi.fn();
  renderHook(() => useAgentSSE({agentId: 'agent/a', onMessage}));
  expect(constructor).toHaveBeenCalledWith('/api/v1/agents/agent%2Fa/stream');
  act(() => listeners.get('chat')?.({data: '{"content":"hello"}'}));
  expect(onMessage).toHaveBeenCalledWith({content: 'hello'});
});
