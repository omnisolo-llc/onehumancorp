import React from 'react';
import { render } from 'ink-testing-library';
import { WorkflowBuilder } from './WorkflowBuilder';
import { expect, test, describe, vi } from 'vitest';

describe('WorkflowBuilder', () => {
  test('renders palette and initial empty canvas', () => {
    const { lastFrame } = render(<WorkflowBuilder onBack={() => {}} onRun={() => {}} />);
    const output = lastFrame();
    expect(output).toContain('Visual Workflow Builder (CLI Edition)');
    expect(output).toContain('Palette:');
    expect(output).toContain('Inbound Message (Trigger)');
    expect(output).toContain('Canvas:');
    expect(output).toContain('Canvas is empty.');
  });

  test('can navigate and add blocks, then run workflow', async () => {
    const onRun = vi.fn();
    const { stdin, lastFrame } = render(<WorkflowBuilder onBack={() => {}} onRun={onRun} />);

    // Default selection is 0 (Inbound Message)
    // Press enter to add to canvas
    stdin.write('\r');
    await new Promise(r => setTimeout(r, 20));

    // Go down to "Web Research" (index 2)
    stdin.write('\u001B[B'); // Down
    await new Promise(r => setTimeout(r, 20));
    stdin.write('\u001B[B'); // Down
    await new Promise(r => setTimeout(r, 20));

    // Press enter to add
    stdin.write('\r');
    await new Promise(r => setTimeout(r, 20));

    const output = lastFrame();
    expect(output).toContain('[Trigger] Inbound Message');
    expect(output).toContain('[Action] Web Research');

    // Go to RUN WORKFLOW button
    for(let i=0; i<6; i++) {
        stdin.write('\u001B[B'); // Down
        await new Promise(r => setTimeout(r, 20));
    }

    // Press enter on RUN WORKFLOW
    stdin.write('\r');
    await new Promise(r => setTimeout(r, 20));

    expect(onRun).toHaveBeenCalled();
    const payload = onRun.mock.calls[0][0];
    expect(payload.version).toBe('1.0');
    expect(Object.keys(payload.nodes).length).toBe(2);
  });

  test('can go back', async () => {
    const onBack = vi.fn();
    const { stdin } = render(<WorkflowBuilder onBack={onBack} onRun={() => {}} />);

    // Go to BACK button (index = available blocks length + 1 = 8 + 1 = 9)
    for(let i=0; i<15; i++) {
        stdin.write('\u001B[B'); // Down
        await new Promise(r => setTimeout(r, 20));
    }

    // Also test up arrow a little
    stdin.write('\u001B[A'); // Up
    await new Promise(r => setTimeout(r, 20));
    stdin.write('\u001B[B'); // Down
    await new Promise(r => setTimeout(r, 20));

    // Press enter on BACK
    stdin.write('\r');
    await new Promise(r => setTimeout(r, 20));

    expect(onBack).toHaveBeenCalled();
  });
});
