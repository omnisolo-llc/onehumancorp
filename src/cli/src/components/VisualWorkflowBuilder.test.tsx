import React from 'react';
import { render } from 'ink-testing-library';
import { VisualWorkflowBuilder } from './VisualWorkflowBuilder.js';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { useVisualWorkflow } from '../hooks/useVisualWorkflow.js';

vi.mock('../hooks/useVisualWorkflow.js', () => ({
  useVisualWorkflow: vi.fn(),
}));

const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

describe('VisualWorkflowBuilder', () => {
  const runWorkflowMock = vi.fn();

  beforeEach(() => {
    (useVisualWorkflow as any).mockReturnValue({
      status: 'idle',
      result: null,
      error: null,
      runWorkflow: runWorkflowMock,
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders the initial state correctly', async () => {
    const onBack = vi.fn();
    const { lastFrame } = render(<VisualWorkflowBuilder onBack={onBack} />);
    await delay(10);
    expect(lastFrame()!).toContain('Visual Workflow Builder');
    expect(lastFrame()!).toContain('No nodes in the workflow');
  });

  it('allows adding nodes', async () => {
    const onBack = vi.fn();
    const { stdin, lastFrame } = render(<VisualWorkflowBuilder onBack={onBack} />);

    await delay(10);
    stdin.write('a');
    await delay(10);
    expect(lastFrame()!).toContain('Select Node Type to Add:');

    stdin.write('1');
    await delay(10);
    expect(lastFrame()!).toContain('[Input] node_1');

    stdin.write('a');
    await delay(10);
    stdin.write('2');
    await delay(10);
    expect(lastFrame()!).toContain('[Llm] node_2');

    stdin.write('a');
    await delay(10);
    stdin.write('3');
    await delay(10);
    expect(lastFrame()!).toContain('[Output] node_3');
  });

  it('allows selecting and deleting nodes', async () => {
    const onBack = vi.fn();
    const { stdin, lastFrame } = render(<VisualWorkflowBuilder onBack={onBack} />);

    await delay(10);
    stdin.write('a');
    await delay(10);
    stdin.write('1');
    await delay(10);
    stdin.write('a');
    await delay(10);
    stdin.write('2');
    await delay(10);

    stdin.write('k');
    await delay(10);
    expect(lastFrame()!).toContain('> [Input] node_1');

    stdin.write('j');
    await delay(10);

    stdin.write('d');
    await delay(10);
    expect(lastFrame()!).not.toContain('[Llm] node_2');
    expect(lastFrame()!).toContain('[Input] node_1');

    stdin.write('d');
    await delay(10);
    expect(lastFrame()!).toContain('No nodes in the workflow');
  });

  it('calls onBack when escape is pressed in view mode', async () => {
    const onBack = vi.fn();
    const { stdin } = render(<VisualWorkflowBuilder onBack={onBack} />);
    await delay(10);
    stdin.write('\x1B');
    await delay(50);
    expect(onBack).toHaveBeenCalled();
  });

  it('returns to view mode when escape is pressed in add mode', async () => {
    const onBack = vi.fn();
    const { stdin, lastFrame } = render(<VisualWorkflowBuilder onBack={onBack} />);
    await delay(10);
    stdin.write('a');
    await delay(10);
    expect(lastFrame()!).toContain('Select Node Type to Add:');
    stdin.write('\x1B');
    await delay(50);
    expect(lastFrame()!).toContain('No nodes in the workflow');
    expect(onBack).not.toHaveBeenCalled();
  });

  it('disables input when status is running', async () => {
    (useVisualWorkflow as any).mockReturnValue({
      status: 'running',
      result: null,
      error: null,
      runWorkflow: runWorkflowMock,
    });

    const onBack = vi.fn();
    const { stdin, lastFrame } = render(<VisualWorkflowBuilder onBack={onBack} />);
    await delay(10);

    expect(lastFrame()!).toContain('Running workflow on backend...');

    stdin.write('a');
    await delay(10);
    expect(lastFrame()!).not.toContain('Select Node Type to Add:');
  });

  it('displays error state correctly', async () => {
    (useVisualWorkflow as any).mockReturnValue({
      status: 'error',
      result: null,
      error: 'Backend connection failed',
      runWorkflow: runWorkflowMock,
    });

    const onBack = vi.fn();
    const { lastFrame } = render(<VisualWorkflowBuilder onBack={onBack} />);
    await delay(10);

    expect(lastFrame()!).toContain('Backend connection failed');
  });

  it('displays result state correctly', async () => {
    (useVisualWorkflow as any).mockReturnValue({
      status: 'complete',
      result: 'Success!',
      error: null,
      runWorkflow: runWorkflowMock,
    });

    const onBack = vi.fn();
    const { lastFrame } = render(<VisualWorkflowBuilder onBack={onBack} />);
    await delay(10);

    expect(lastFrame()!).toContain('Workflow Result:');
    expect(lastFrame()!).toContain('Success!');
  });

  it('executes workflow when r is pressed', async () => {
    const onBack = vi.fn();
    const { stdin, lastFrame } = render(<VisualWorkflowBuilder onBack={onBack} />);

    await delay(10);
    // Add nodes: Input -> Llm -> Output
    stdin.write('a'); await delay(10); stdin.write('1'); await delay(10);
    stdin.write('a'); await delay(10); stdin.write('2'); await delay(10);
    stdin.write('a'); await delay(10); stdin.write('3'); await delay(10);

    // Press r to run
    stdin.write('r');
    await delay(50);

    expect(runWorkflowMock).toHaveBeenCalledWith(
      {
        nodes: [
          { id: 'node_1', type: { Input: { name: 'in' } } },
          { id: 'node_2', type: { Llm: { prompt_template: 'Process: {{in}}' } } },
          { id: 'node_3', type: { Output: null } }
        ],
        edges: [
          { source: 'node_1', target: 'node_2' },
          { source: 'node_2', target: 'node_3' }
        ]
      },
      { in: 'test data' }
    );
  });
});
