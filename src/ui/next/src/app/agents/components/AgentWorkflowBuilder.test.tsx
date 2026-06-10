import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { expect, test, vi } from 'vitest';
import { AgentWorkflowBuilder } from './AgentWorkflowBuilder';

test('renders builder and allows adding/removing blocks', async () => {
  const mockOnSave = vi.fn().mockResolvedValue(undefined);
  render(<AgentWorkflowBuilder onSave={mockOnSave} />);

  // Initial state empty
  expect(screen.getByText(/Click blocks on the left/i)).toBeTruthy();

  // Type workflow name
  fireEvent.change(screen.getByPlaceholderText('e.g., Auto-reply to VIPs'), { target: { value: 'My Workflow' } });

  // Click palette blocks
  fireEvent.click(screen.getByTestId('palette-block-trigger_message'));
  fireEvent.click(screen.getByTestId('palette-block-action_analyze'));

  // Blocks appear
  expect(screen.getByTestId('canvas-block-0')).toBeTruthy();
  expect(screen.getByTestId('canvas-block-1')).toBeTruthy();

  // Remove first block
  fireEvent.click(screen.getByTestId('canvas-block-0').querySelector('button')!);

  // Save
  fireEvent.click(screen.getByText('Create & Run Workflow'));

  await waitFor(() => {
    expect(mockOnSave).toHaveBeenCalled();
  });

  // Verify compiled JSON structure
  const [nameArg, payloadArg] = mockOnSave.mock.calls[0];
  expect(nameArg).toBe('My Workflow');

  const payload = JSON.parse(payloadArg);
  expect(payload.version).toBe('1.0');

  // It should be action_analyze since the first one was removed
  expect(Object.values(payload.nodes).length).toBe(1);
  const firstNode: any = Object.values(payload.nodes)[0];
  expect(firstNode.type).toBe('Action');
  expect(firstNode.label).toBe('Analyze Sentiment');
});

test('shows error when onSave fails', async () => {
  const mockOnSave = vi.fn().mockRejectedValue(new Error('API Down'));
  render(<AgentWorkflowBuilder onSave={mockOnSave} />);

  fireEvent.change(screen.getByPlaceholderText('e.g., Auto-reply to VIPs'), { target: { value: 'Bad Workflow' } });
  fireEvent.click(screen.getByTestId('palette-block-trigger_message'));
  fireEvent.click(screen.getByText('Create & Run Workflow'));

  await waitFor(() => {
    expect(screen.getByTestId('builder-error')).toHaveTextContent('API Down');
  });
});
