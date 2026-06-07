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
    json: () => Promise.resolve({ milestones: [] })
  });

  const { container } = render(<SuccessMilestoneAlert />);
  await waitFor(() => {
    expect(screen.queryByTestId('success-milestone-alert')).not.toBeInTheDocument();
  });
});

test('renders alert when reached', async () => {
  fetchMock.mockResolvedValueOnce({
    ok: true,
    json: () => Promise.resolve({
      milestones: [
        {
          id: 'first_order',
          title: 'First Order!',
          description: 'You completed your first order!',
          reached: true
        }
      ]
    })
  });

  render(<SuccessMilestoneAlert />);

  await waitFor(() => {
    expect(screen.getByTestId('success-milestone-alert')).toBeInTheDocument();
    expect(screen.getByText('First Order!')).toBeInTheDocument();
    expect(screen.getByText('You completed your first order!')).toBeInTheDocument();
  });
});
