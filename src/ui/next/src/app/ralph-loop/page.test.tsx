import React from 'react';
import { act, render, screen, fireEvent, waitFor } from '@testing-library/react';
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

test('shows disabled loading state until the successful request settles', async () => {
  const mockResult = { status: 'success', features_completed: 3 };
  let finishRequest!: (response: Response) => void;
  const pendingResponse = new Promise<Response>(resolve => { finishRequest = resolve; });
  vi.mocked(global.fetch).mockReturnValueOnce(pendingResponse);

  render(<RalphLoopPage />);

  const textarea = screen.getByLabelText(/Long-Running Task Description/);
  fireEvent.change(textarea, { target: { value: 'Build a server' } });

  const button = screen.getByRole('button', { name: /Start Ralph Loop/ });
  expect(button).not.toBeDisabled();

  await act(async () => { fireEvent.click(button); });

  expect(screen.getByRole('button', { name: /Ralph Loop Executing/ })).toBeDisabled();
  expect(screen.queryByTestId('success-message')).not.toBeInTheDocument();
  expect(screen.queryByTestId('error-message')).not.toBeInTheDocument();

  await act(async () => { finishRequest(Response.json({ result: mockResult })); });
  await waitFor(() => {
    expect(screen.getByTestId('success-message')).toBeInTheDocument();
  });

  expect(screen.getByText(/features_completed/)).toBeInTheDocument();
  expect(screen.getByRole('button', { name: /Start Ralph Loop/ })).toBeEnabled();
});

test('shows disabled loading state until the failed request settles', async () => {
  let finishRequest!: (response: Response) => void;
  const pendingResponse = new Promise<Response>(resolve => { finishRequest = resolve; });
  vi.mocked(global.fetch).mockReturnValueOnce(pendingResponse);

  render(<RalphLoopPage />);

  const textarea = screen.getByLabelText(/Long-Running Task Description/);
  fireEvent.change(textarea, { target: { value: 'Build a server' } });

  await act(async () => {
    fireEvent.click(screen.getByRole('button', { name: /Start Ralph Loop/ }));
  });

  expect(screen.getByRole('button', { name: /Ralph Loop Executing/ })).toBeDisabled();
  expect(screen.queryByTestId('success-message')).not.toBeInTheDocument();
  expect(screen.queryByTestId('error-message')).not.toBeInTheDocument();

  await act(async () => {
    finishRequest(Response.json({ error: 'Backend failed to process' }, { status: 503 }));
  });
  await waitFor(() => {
    expect(screen.getByTestId('error-message')).toBeInTheDocument();
  });

  expect(screen.getByText(/Backend failed to process/)).toBeInTheDocument();
  expect(screen.queryByTestId('success-message')).not.toBeInTheDocument();
  expect(screen.getByRole('button', { name: /Start Ralph Loop/ })).toBeEnabled();
});
