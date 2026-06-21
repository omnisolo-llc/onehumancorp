import React from 'react';
import { render } from 'ink-testing-library';
import { VisualWorkflowEditor } from './VisualWorkflowEditor.js';
import { vi, describe, it, expect, beforeEach } from 'vitest';
import * as visualWorkflowHook from '../hooks/useVisualWorkflow.js';

vi.mock('../hooks/useVisualWorkflow.js', () => ({
  useVisualWorkflow: vi.fn()
}));

describe('VisualWorkflowEditor Component', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it('renders the predefined workflows and handles keyboard inputs', () => {
    const mockRun = vi.fn();
    const mockBack = vi.fn();
    vi.mocked(visualWorkflowHook.useVisualWorkflow).mockReturnValue({
      result: null,
      loading: false,
      error: null,
      runWorkflow: mockRun
    });

    const { lastFrame, stdin } = render(<VisualWorkflowEditor onBack={mockBack} />);
    expect(lastFrame()!).toContain('Block-Based Visual Workflow Editor');
    expect(lastFrame()!).toContain('Simple Linear Workflow');

    // arrow down
    stdin.write('\x1B[B');
    expect(lastFrame()!).toContain('Condition Workflow');

    // arrow up
    stdin.write('\x1B[A');

    // return key to run
    stdin.write('\r');
    expect(mockRun).toHaveBeenCalled();

    // escape to exit - ink-testing-library might not handle \x1B easily, use q
    stdin.write('q');
    expect(mockBack).toHaveBeenCalled();
  });

  it('renders loading state', () => {
    vi.mocked(visualWorkflowHook.useVisualWorkflow).mockReturnValue({
      result: null,
      loading: true,
      error: null,
      runWorkflow: vi.fn()
    });

    const { lastFrame } = render(<VisualWorkflowEditor onBack={() => {}} />);
    expect(lastFrame()!).toContain('Executing Visual Workflow on Backend...');
  });

  it('renders error state', () => {
    vi.mocked(visualWorkflowHook.useVisualWorkflow).mockReturnValue({
      result: null,
      loading: false,
      error: 'Graph execution failed',
      runWorkflow: vi.fn()
    });

    const { lastFrame } = render(<VisualWorkflowEditor onBack={() => {}} />);
    expect(lastFrame()!).toContain('Error: Graph execution failed');
  });

  it('renders result', () => {
    vi.mocked(visualWorkflowHook.useVisualWorkflow).mockReturnValue({
      result: 'Output successfully generated',
      loading: false,
      error: null,
      runWorkflow: vi.fn()
    });

    const { lastFrame } = render(<VisualWorkflowEditor onBack={() => {}} />);
    expect(lastFrame()!).toContain('Workflow Execution Result:');
    expect(lastFrame()!).toContain('Output successfully generated');
  });
});
