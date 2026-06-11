import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import AgentProtocolPage from './page';

const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('Agent Protocol UI', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockFetch.mockResolvedValue({
      ok: true,
      json: async () => ({ tasks: [] }),
    });
  });

  it('renders the agent protocol page', async () => {
    render(<AgentProtocolPage />);
    expect(screen.getByText('Agent Protocol UI')).toBeInTheDocument();
    expect(screen.getByText('Tasks')).toBeInTheDocument();

    await waitFor(() => {
      expect(mockFetch).toHaveBeenCalledWith('/api/agents/protocol?method=ap_list_tasks');
    });
  });

  it('allows creating a task', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ tasks: [] }), // initial load
    });

    render(<AgentProtocolPage />);

    const taskInput = screen.getByPlaceholderText('New Task Input...');
    fireEvent.change(taskInput, { target: { value: 'Write a poem' } });

    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ task_id: 'task-1' }), // create response
    });

    // The second fetchTasks call
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ tasks: [{ task_id: 'task-1', input: 'Write a poem' }] }),
    });

    fireEvent.click(screen.getByRole('button', { name: 'Create' }));

    await waitFor(() => {
      expect(screen.getByText('Write a poem')).toBeInTheDocument();
    });
  });

  it('allows selecting a task and executing a step', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ tasks: [{ task_id: 'task-1', input: 'Write a poem' }] }), // initial load
    });

    render(<AgentProtocolPage />);

    await waitFor(() => {
      expect(screen.getByText('Write a poem')).toBeInTheDocument();
    });

    // Select the task
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ steps: [] }), // fetch steps response
    });

    fireEvent.click(screen.getByText('Write a poem'));

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Execute Step' })).toBeInTheDocument();
    });

    const stepInput = screen.getByPlaceholderText('Optional Step Input...');
    fireEvent.change(stepInput, { target: { value: 'Line 1' } });

    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ step_id: 'step-1' }), // execute response
    });

    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ steps: [{ step_id: 'step-1', status: 'completed', input: 'Line 1', output: 'Roses are red' }] }), // fetch steps
    });

    fireEvent.click(screen.getByRole('button', { name: 'Execute Step' }));

    await waitFor(() => {
      expect(screen.getByText('Roses are red')).toBeInTheDocument();
    });
  });
});

describe('AgentProtocolPage Artifacts', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders artifacts tab and fetches artifacts', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ tasks: [{ task_id: 'task-1', input: 'Test Task' }] }), // fetch tasks
    });

    render(<AgentProtocolPage />);

    await waitFor(() => {
      expect(screen.getByText('Test Task')).toBeInTheDocument();
    });

    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ steps: [] }), // fetch steps
    });

    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ artifacts: [{ artifact_id: 'art-1', file_name: 'test.txt' }] }), // fetch artifacts
    });

    fireEvent.click(screen.getByText('Test Task'));
    fireEvent.click(screen.getByText('Artifacts'));

    await waitFor(() => {
      expect(screen.getByText('test.txt')).toBeInTheDocument();
    });
  });
});
