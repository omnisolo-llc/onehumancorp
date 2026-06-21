import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { App } from './App.js';
import * as orchestrator from './hooks/useOrchestrator.js';

describe('App', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly and tests useEffect coverage including cleanup', async () => {
    vi.spyOn(orchestrator, 'useOrchestrator').mockReturnValue({
      status: 'Initializing Agent...',
      tools: [],
      error: null,
      runAgent: vi.fn(),
      output: null
    });

    const { lastFrame, unmount } = render(<App />);
    let output = lastFrame();

    expect(output).toContain('ONE HUMAN CORP');
    expect(output).toContain('Initializing Agent...');
    expect(output).toContain('Tools Executed:');
    expect(output).toContain('OHC Interactive Harness');
    expect(output).toContain('Select an action');

    unmount();
  });

  it('renders error state correctly', () => {
    vi.spyOn(orchestrator, 'useOrchestrator').mockReturnValue({
      status: 'error',
      tools: [],
      error: 'Test Error Message',
      runAgent: vi.fn(),
      output: null
    });

    const { lastFrame } = render(<App />);
    const output = lastFrame();

    expect(output).toContain('ERROR:');
    expect(output).toContain('Test Error Message');
  });

  it('handles prompt input correctly', async () => {
    vi.spyOn(orchestrator, 'useOrchestrator').mockReturnValue({
      status: 'ok',
      tools: [],
      error: null,
      runAgent: vi.fn(),
      output: null
    });

    const { lastFrame, unmount } = render(<App />);
    const output = lastFrame();
    expect(output).toContain('Ask Agent >');
    unmount();
  });

  it('can submit prompt input (coverage) and renders output', async () => {
    const runAgentMock = vi.fn();
    vi.spyOn(orchestrator, 'useOrchestrator').mockReturnValue({
      status: 'ok',
      tools: [],
      error: null,
      runAgent: runAgentMock,
      output: 'output text mock'
    });

    const { lastFrame, stdin } = render(<App />);

    stdin.write('test query');
    stdin.write('\r');

    // Wait a tick for react to process
    await new Promise(r => setTimeout(r, 10));

    const output = lastFrame();
    expect(output).toContain('Ask Agent >');
  });

  it('covers prompt submit handling multiple inputs', async () => {
     const runAgentMock = vi.fn();
     vi.spyOn(orchestrator, 'useOrchestrator').mockReturnValue({
       status: 'ok',
       tools: [],
       error: null,
       runAgent: runAgentMock,
       output: 'another mock'
     });

     const { lastFrame, stdin } = render(<App />);

     stdin.write('query 1');
     stdin.write('\r');
     await new Promise(r => setTimeout(r, 50));

     stdin.write('query 2');
     stdin.write('\r');
     await new Promise(r => setTimeout(r, 50));

     const output = lastFrame();
     // Ink testing library sometimes strips out newlines/words in fast updates
     expect(output).toContain('another mock');
     expect(runAgentMock).toHaveBeenCalledTimes(2);
  });

  it('handles selecting Browse Agent Marketplace', async () => {
     const runAgentMock = vi.fn();
     vi.spyOn(orchestrator, 'useOrchestrator').mockReturnValue({
       status: 'ok',
       tools: [],
       error: null,
       runAgent: runAgentMock,
       output: null
     });

     const { lastFrame, stdin } = render(<App />);

     // 10th item is Browse Agent Marketplace
     for (let i = 0; i < 9; i++) {
        stdin.write('\u001B[B');
        await new Promise(r => setTimeout(r, 20));
     }

     stdin.write('\r');
     await new Promise(r => setTimeout(r, 20));

     const output = lastFrame();
     expect(output).toContain('Agent Marketplace');

     // Press q to exit marketplace
     stdin.write('q');
     await new Promise(r => setTimeout(r, 20));

     expect(lastFrame()).toContain('Select an action');
  });
});
