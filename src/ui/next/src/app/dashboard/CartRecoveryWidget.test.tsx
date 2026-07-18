import { render, screen, waitFor } from '@testing-library/react';
import type { ReactNode } from 'react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { CartRecoveryWidget } from './CartRecoveryWidget';

vi.mock('next/link', () => ({
  default: ({ children, href }: { children: ReactNode; href: string }) => (
    <a href={href}>{children}</a>
  ),
}));

describe('CartRecoveryWidget', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    vi.unstubAllGlobals();
  });

  it('shows the real abandoned cart count without inventing recoveries or revenue', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ count: 8 }),
    }));

    render(<CartRecoveryWidget />);

    expect(await screen.findByText('8')).toBeInTheDocument();
    expect(screen.getByText('Abandoned carts')).toBeInTheDocument();
    expect(screen.queryByText('Revenue Saved')).not.toBeInTheDocument();
    expect(screen.queryByText('$200')).not.toBeInTheDocument();
  });

  it('fails closed when recovery data cannot be loaded', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({ ok: false, status: 503 }));

    render(<CartRecoveryWidget />);

    expect(await screen.findByText('Cart recovery data is unavailable.')).toBeInTheDocument();
    await waitFor(() => expect(screen.queryByText('$120')).not.toBeInTheDocument());
  });
});
