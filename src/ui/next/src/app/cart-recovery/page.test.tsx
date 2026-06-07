import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import CartRecoveryPage from './page';
import * as navigation from 'next/navigation';
import userEvent from '@testing-library/user-event';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: any) => <>{children}</>,
  TooltipProvider: ({ children }: any) => <>{children}</>
}));

describe('CartRecoveryPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(navigation.useRouter).mockReturnValue({
      push: vi.fn(),
    } as any);
  });

  it('renders correctly', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ count: 3 })
    });
    render(<CartRecoveryPage />);
    expect(screen.getByText('Abandoned Cart Recovery 🛒')).toBeInTheDocument();
  });
});
