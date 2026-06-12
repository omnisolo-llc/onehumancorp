import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { vi } from 'vitest';
import AgentBuilderPage from './page';

// Mock fetch globally for the test
const mockFetch = vi.fn();
global.fetch = mockFetch as any;

describe('AgentBuilderPage', () => {
  beforeEach(() => {
    mockFetch.mockClear();
  });

  it('renders correctly and loads default graph', () => {
    render(<AgentBuilderPage />);

    // Check title
    expect(screen.getByText('Agent Builder (Visual Canvas)')).toBeInTheDocument();

    // Check default input value
    const inputsTextarea = screen.getByRole('textbox') as HTMLTextAreaElement;
    expect(inputsTextarea.value).toContain('Hello world');
  });

  it('handles run workflow successfully', async () => {
    mockFetch.mockResolvedValueOnce({
      json: async () => ({ success: true, result: 'Mocked Output' })
    });

    render(<AgentBuilderPage />);

    const runButton = screen.getByRole('button', { name: /Run Agent Workflow/i });
    fireEvent.click(runButton);

    expect(screen.getByText('Running...')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Mocked Output')).toBeInTheDocument();
    });

    expect(mockFetch).toHaveBeenCalledWith('/api/workflow/run', expect.objectContaining({
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
    }));
  });

  it('handles run workflow error', async () => {
    mockFetch.mockResolvedValueOnce({
      json: async () => ({ success: false, error: 'Internal Server Error' })
    });

    render(<AgentBuilderPage />);

    const runButton = screen.getByRole('button', { name: /Run Agent Workflow/i });
    fireEvent.click(runButton);

    await waitFor(() => {
      expect(screen.getByText('Error: Internal Server Error')).toBeInTheDocument();
    });
  });
});
