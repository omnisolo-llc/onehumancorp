import React from 'react';
import { render, screen, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import CartRecoveryPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('CartRecoveryPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ count: 3 })
    } as any);
  });

  it('renders the cart recovery page', async () => {
    await act(async () => {
      render(<CartRecoveryPage />);
    });
    expect(screen.getByText('Abandoned Cart Recovery 🛒')).toBeDefined();
    expect(screen.getByText('Recover Abandoned Carts')).toBeDefined();
  });
});
