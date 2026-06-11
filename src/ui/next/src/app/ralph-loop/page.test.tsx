import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { expect, test, vi, beforeEach } from 'vitest';
import RalphLoopPage from './page';

beforeEach(() => {
  global.fetch = vi.fn();
});

test('renders Ralph Loop page', () => {
  render(<RalphLoopPage />);
  expect(screen.getByText('The Ralph Loop (Long-Running Agent)')).toBeInTheDocument();
  expect(screen.getByText(/Enter a complex task spanning multiple context windows/)).toBeInTheDocument();
  expect(screen.getByRole('button', { name: /Start Ralph Loop/ })).toBeDisabled();
});

test('can type task and execute successfully', async () => {
  const mockResult = { status: 'success', features_completed: 3 };
  (global.fetch as any).mockResolvedValueOnce({
    ok: true,
    json: async () => ({ result: mockResult }),
  });

  render(<RalphLoopPage />);

  const textarea = screen.getByLabelText(/Long-Running Task Description/);
  fireEvent.change(textarea, { target: { value: 'Build a server' } });

  const button = screen.getByRole('button', { name: /Start Ralph Loop/ });
  expect(button).not.toBeDisabled();

  fireEvent.click(button);

  expect(screen.getByRole('button', { name: /Ralph Loop Executing/ })).toBeInTheDocument();

  await waitFor(() => {
    expect(screen.getByTestId('success-message')).toBeInTheDocument();
  });

  expect(screen.getByText(/features_completed/)).toBeInTheDocument();
});

test('handles errors correctly', async () => {
  (global.fetch as any).mockResolvedValueOnce({
    ok: false,
    json: async () => ({ error: 'Backend failed to process' }),
  });

  render(<RalphLoopPage />);

  const textarea = screen.getByLabelText(/Long-Running Task Description/);
  fireEvent.change(textarea, { target: { value: 'Build a server' } });

  fireEvent.click(screen.getByRole('button', { name: /Start Ralph Loop/ }));

  await waitFor(() => {
    expect(screen.getByTestId('error-message')).toBeInTheDocument();
  });

  expect(screen.getByText(/Backend failed to process/)).toBeInTheDocument();
});
