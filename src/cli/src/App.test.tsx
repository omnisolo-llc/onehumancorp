import React from 'react';
import { render } from 'ink-testing-library';
import { App } from './App.js';
import { describe, it, expect, vi } from 'vitest';
import { useOrchestrator } from './hooks/useOrchestrator.js';

// Mock the hook
vi.mock('./hooks/useOrchestrator.js', () => ({
  useOrchestrator: vi.fn(),
}));

const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

describe('App', () => {
  it('renders the header and main sections', async () => {
    (useOrchestrator as any).mockReturnValue({
      status: 'idle',
      tools: [],
      error: null,
      runAgent: vi.fn(),
      output: null,
    });

    const { lastFrame } = render(<App />);
    await delay(10);
    const frame = lastFrame()!;
    expect(frame).toContain('OHC');
    expect(frame).toContain('Select an action');
    expect(frame).toContain('Ask Agent >');
  });

  it('handles submitting a prompt', async () => {
    let callCount = 0;
    const runAgentMock = vi.fn().mockImplementation(async () => {
        callCount++;
    });

    (useOrchestrator as any).mockImplementation(() => {
        return {
            status: 'idle',
            tools: [],
            error: null,
            runAgent: runAgentMock,
            output: callCount > 0 ? 'Mock agent response' : null,
        }
    });

    const { stdin, lastFrame } = render(<App />);
    await delay(10);
    stdin.write('h');
    await delay(10);
    stdin.write('i');
    await delay(10);
    stdin.write('\r');

    await delay(50);

    const frame = lastFrame()!;
    expect(frame).toContain('User:');
    expect(frame).toContain('hi');
    expect(runAgentMock).toHaveBeenCalledWith('hi');
  });

  it('renders error state when error is present', async () => {
    (useOrchestrator as any).mockReturnValue({
      status: 'error',
      tools: [],
      error: 'Test error message',
      runAgent: vi.fn(),
      output: null,
    });

    const { lastFrame } = render(<App />);
    await delay(10);
    const frame = lastFrame()!;
    expect(frame).toContain('ERROR');
    expect(frame).toContain('Test error message');
    expect(frame).not.toContain('Ask Agent >');
  });

  it('handles selecting Browse Agent Marketplace', async () => {
    (useOrchestrator as any).mockReturnValue({
      status: 'idle',
      tools: [],
      error: null,
      runAgent: vi.fn(),
      output: null,
    });

    const { stdin, lastFrame } = render(<App />);

    await delay(10);
    // Navigate to Browse Agent Marketplace
    for(let i=0; i<9; i++) {
        stdin.write('\x1B[B');
        await delay(10);
    }

    // Press enter
    stdin.write('\r');
    await delay(50);

    const frame = lastFrame()!;
    expect(frame).toContain('Agent Marketplace');
  });

  it('handles selecting Visual Workflow Builder', async () => {
    (useOrchestrator as any).mockReturnValue({
      status: 'idle',
      tools: [],
      error: null,
      runAgent: vi.fn(),
      output: null,
    });

    const { stdin, lastFrame } = render(<App />);

    await delay(10);
    // Navigate to Visual Workflow Builder
    for(let i=0; i<10; i++) {
        stdin.write('\x1B[B');
        await delay(10);
    }

    // Press enter
    stdin.write('\r');
    await delay(50);

    const frame = lastFrame()!;
    expect(frame).toContain('Visual Workflow Builder');
  });

  it('handles rendering agent output correctly', async () => {
    let callCount = 0;
    const runAgentMock = vi.fn().mockImplementation(async () => {
        callCount++;
    });

    (useOrchestrator as any).mockImplementation(() => {
        return {
            status: 'idle',
            tools: [],
            error: null,
            runAgent: runAgentMock,
            output: callCount > 0 ? 'Mock markdown response' : null,
        }
    });

    const { stdin, lastFrame } = render(<App />);
    await delay(10);
    stdin.write('x');
    await delay(10);
    stdin.write('\r');

    await delay(50);

    const frame = lastFrame()!;
    expect(frame).toContain('Agent:');
    expect(frame).toContain('Mock');
  });
});
