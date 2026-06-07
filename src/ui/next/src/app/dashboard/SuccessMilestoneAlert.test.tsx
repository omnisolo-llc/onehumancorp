import { render, screen, waitFor } from '@testing-library/react';
import { expect, test, vi, beforeEach } from 'vitest';
import { SuccessMilestoneAlert } from './SuccessMilestoneAlert';

const fetchMock = vi.fn();
global.fetch = fetchMock;

Object.assign(navigator, {
  clipboard: {
    writeText: vi.fn().mockResolvedValue(undefined),
  },
});

beforeEach(() => {
  fetchMock.mockClear();
});

test('renders empty state initially or when not reached', async () => {
  fetchMock.mockResolvedValueOnce({
    ok: true,
    json: () => Promise.resolve({ reached: false })
  });

  const { container } = render(<SuccessMilestoneAlert />);
  await waitFor(() => {
    expect(screen.queryByTestId('success-milestone-alert')).not.toBeInTheDocument();
  });
});

test('renders alert when reached', async () => {
  fetchMock.mockResolvedValueOnce({
    ok: true,
    json: () => Promise.resolve({ reached: true, type: 'first_order', message: 'You completed your first order!' })
  });

  render(<SuccessMilestoneAlert />);

  await waitFor(() => {
    expect(screen.getByTestId('success-milestone-alert')).toBeInTheDocument();
    expect(screen.getByText('You completed your first order!')).toBeInTheDocument();
  });
});
