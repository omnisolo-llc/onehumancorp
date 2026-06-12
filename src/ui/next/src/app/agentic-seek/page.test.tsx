import React from 'react';
import { render, screen, act, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import AgenticSeekPage from './page';

describe('AgenticSeek Page', () => {
  let fetchMock: any;

  beforeEach(() => {
    fetchMock = vi.spyOn(global, 'fetch').mockImplementation(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ result: 'Local success output' }),
      } as Response)
    );
  });

  afterEach(() => {
    fetchMock.mockRestore();
  });

  it('renders the AgenticSeek UI correctly', async () => {
    await act(async () => {
      render(<AgenticSeekPage />);
    });

    expect(screen.getByRole('heading', { name: 'AgenticSeek Local Agent' })).toBeInTheDocument();
    expect(screen.getByPlaceholderText(/Analyze the local log files/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Execute Local Task' })).toBeDisabled();
  });

  it('enables the button when input is provided and executes task', async () => {
    await act(async () => {
      render(<AgenticSeekPage />);
    });

    const textarea = screen.getByPlaceholderText(/Analyze the local log files/i);
    const button = screen.getByRole('button', { name: 'Execute Local Task' });

    await act(async () => {
      fireEvent.change(textarea, { target: { value: 'Test task' } });
    });

    expect(button).toBeEnabled();

    await act(async () => {
      fireEvent.click(button);
    });

    expect(fetchMock).toHaveBeenCalledWith('/api/agents/agentic-seek', expect.objectContaining({
      method: 'POST',
      body: JSON.stringify({ task: 'Test task' })
    }));

    expect(await screen.findByText('Local Execution Result')).toBeInTheDocument();
    expect(screen.getByText('Local success output')).toBeInTheDocument();
  });

  it('handles and displays errors', async () => {
    fetchMock.mockImplementationOnce(() =>
      Promise.resolve({
        ok: false,
        json: () => Promise.resolve({ error: 'Local failure' }),
      } as Response)
    );

    await act(async () => {
      render(<AgenticSeekPage />);
    });

    const textarea = screen.getByPlaceholderText(/Analyze the local log files/i);
    const button = screen.getByRole('button', { name: 'Execute Local Task' });

    await act(async () => {
      fireEvent.change(textarea, { target: { value: 'Test error task' } });
    });

    await act(async () => {
      fireEvent.click(button);
    });

    expect(await screen.findByText('Execution Error:')).toBeInTheDocument();
    expect(screen.getByText('Local failure')).toBeInTheDocument();
  });
});
