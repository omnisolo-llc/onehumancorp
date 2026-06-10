import React from 'react';
import { render } from 'ink-testing-library';
import { AgentProtocol } from './AgentProtocol';
import { describe, it, expect, vi } from 'vitest';

describe('AgentProtocol', () => {
  it('renders correctly and handles fetch', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ result: [{ task_id: '123', input: 'test' }] })
    });
    const { lastFrame } = render(<AgentProtocol />);
    await new Promise((r) => setTimeout(r, 20));
    expect(lastFrame()).toContain('Task: 123');
  });

  it('handles fetch error', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('crash'));
    const { lastFrame } = render(<AgentProtocol />);
    await new Promise((r) => setTimeout(r, 20));
    expect(lastFrame()).toContain('Network Error');
  });

  it('handles fetch loading and empty error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ result: null })
    });
    const { lastFrame } = render(<AgentProtocol />);
    await new Promise((r) => setTimeout(r, 20));
    expect(lastFrame()).toContain('Error loading tasks');
  });
});
