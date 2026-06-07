import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import CartRecoveryPage from './page';

const mockFetch = vi.fn();

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

beforeEach(() => {
  vi.clearAllMocks();
  global.fetch = mockFetch;
});

import { act } from 'react';

test('renders the cart recovery page', async () => {
  await act(async () => {
    render(<CartRecoveryPage />);
  });
  expect(screen.getByText('Recover Abandoned Carts')).toBeInTheDocument();
});
