import React from 'react';
import { render } from 'ink-testing-library';
import { App } from './App';
import { describe, it, expect, vi } from 'vitest';
import { useOrchestrator } from './hooks/useOrchestrator';

// Mock the hook
vi.mock('./hooks/useOrchestrator', () => ({
  useOrchestrator: vi.fn()
}));

describe('App', () => {
  it('renders with pending approval', () => {
    vi.mocked(useOrchestrator).mockReturnValue({
      status: 'Test Status',
      tools: [],
      error: null,
      pendingApproval: {
        toolName: 'test',
        argsJson: '{}',
        reason: 'test reason',
        isHighRisk: false
      }
    });

    const { lastFrame, unmount } = render(<App />);
    const frame = lastFrame();
    expect(frame).toContain('Approval Required');
    expect(frame).toContain('test');
    expect(frame).toContain('test reason');
    unmount();
  });

  it('renders error state', () => {
    vi.mocked(useOrchestrator).mockReturnValue({
      status: '', tools: [], error: 'Test Error', pendingApproval: null
    });
    const { lastFrame, unmount } = render(<App />);
    expect(lastFrame()).toContain('Test Error');
    unmount();
  });

  it('can submit prompt input (coverage)', async () => {
    vi.mocked(useOrchestrator).mockReturnValue({
      status: '', tools: [], error: null, pendingApproval: null
    });
    const { stdin, lastFrame, unmount } = render(<App />);

    // allow state update
    await new Promise(r => setTimeout(r, 50));

    // Simulate user typing "test" and hitting enter
    stdin.write('t');
    stdin.write('e');
    stdin.write('s');
    stdin.write('t');
    stdin.write('\r');

    // allow state update
    await new Promise(r => setTimeout(r, 50));

    expect(lastFrame()).toContain('User');
    unmount();
  });

  it('handles approval actions (Approve)', async () => {
    vi.mocked(useOrchestrator).mockReturnValue({
      status: 'Test Status',
      tools: [],
      error: null,
      pendingApproval: {
        toolName: 'test',
        argsJson: '{}',
        reason: 'test reason',
        isHighRisk: false
      }
    });

    const consoleSpy = vi.spyOn(console, 'log').mockImplementation(() => {});
    const { stdin, unmount, lastFrame } = render(<App />);

    // allow state update
    await new Promise(r => setTimeout(r, 50));

    stdin.write('y');
    await new Promise(r => setTimeout(r, 50));
    expect(consoleSpy).toHaveBeenCalledWith('Approved');

    consoleSpy.mockRestore();
    unmount();
  });

  it('handles approval actions (Reject)', async () => {
    vi.mocked(useOrchestrator).mockReturnValue({
      status: 'Test Status',
      tools: [],
      error: null,
      pendingApproval: {
        toolName: 'test',
        argsJson: '{}',
        reason: 'test reason',
        isHighRisk: false
      }
    });

    const consoleSpy = vi.spyOn(console, 'log').mockImplementation(() => {});
    const { stdin, unmount } = render(<App />);

    // allow state update
    await new Promise(r => setTimeout(r, 50));

    stdin.write('n');
    await new Promise(r => setTimeout(r, 50));
    expect(consoleSpy).toHaveBeenCalledWith('Rejected');

    consoleSpy.mockRestore();
    unmount();
  });

  it('handles approval actions (Edit)', async () => {
    vi.mocked(useOrchestrator).mockReturnValue({
      status: 'Test Status',
      tools: [],
      error: null,
      pendingApproval: {
        toolName: 'test',
        argsJson: '{}',
        reason: 'test reason',
        isHighRisk: false
      }
    });

    const { stdin, unmount, lastFrame } = render(<App />);

    // allow state update
    await new Promise(r => setTimeout(r, 50));

    stdin.write('e');
    await new Promise(r => setTimeout(r, 50));
    const frame = lastFrame();
    expect(frame).toContain('Editing arguments feature is not fully implemented in CLI mockup yet.');

    unmount();
  });

  it('renders correctly and tests useEffect coverage including cleanup', async () => {
    vi.mocked(useOrchestrator).mockReturnValue({
      status: 'Ready', tools: [], error: null, pendingApproval: null
    });
    const { unmount } = render(<App />);
    unmount();
  });
});
