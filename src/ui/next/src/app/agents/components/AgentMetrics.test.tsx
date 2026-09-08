import React from 'react';
import { act, cleanup, render, screen } from '@testing-library/react';
import { afterEach, expect, test, vi } from 'vitest';
import { AgentMetrics } from './AgentMetrics';

let listener: (event: { data: string }) => void;
const close = vi.fn();
afterEach(() => { cleanup(); vi.unstubAllGlobals(); vi.clearAllMocks(); });
function mockStream() {
  vi.stubGlobal('EventSource', class {
    constructor(public url: string) { expect(url).toBe('/api/v1/agents/metrics/stream'); }
    addEventListener(name: string, callback: typeof listener) { if (name === 'metrics') listener = callback; }
    close = close;
  });
}
test('renders measured data and labels unavailable measurements', () => {
  mockStream();
  render(<AgentMetrics />);
  act(() => listener({ data: JSON.stringify([{ agent_id: 'analyst', messages_processed: 2, avg_response_time_ms: 125, active_connections: 1, memory_usage_bytes: null, last_active_at: null, error_rate: 0.5, cost_accumulated: 0.12 }]) }));
  expect(screen.getByText('analyst')).toBeTruthy();
  expect(screen.getByText('125 ms')).toBeTruthy();
  expect(screen.getByText('50%')).toBeTruthy();
  expect(screen.getAllByText('Unavailable').length).toBe(2);
});
test('closes the stream on unmount and displays empty data truthfully', () => {
  mockStream();
  const result = render(<AgentMetrics />);
  act(() => listener({ data: '[]' }));
  expect(screen.getByText('No registered agents yet.')).toBeTruthy();
  result.unmount();
  expect(close).toHaveBeenCalledOnce();
});
