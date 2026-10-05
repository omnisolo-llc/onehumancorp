import React from 'react';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import GrowthAffiliatesPage from './page';

// Isolate the actual page's data contract from the navigation shell's unrelated
// effects. All metric rendering and response validation run in production code.
vi.mock('../../../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <main>{children}</main>,
}));

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

function metric(label: string) {
  return screen.getByText(label, { exact: true }).parentElement!;
}

describe('Affiliate statistics', () => {
  it('shows loading instead of fabricated metrics before the response arrives', async () => {
    let complete!: (value: Response) => void;
    vi.stubGlobal('fetch', vi.fn(() => new Promise<Response>((resolve) => { complete = resolve; })));
    render(<GrowthAffiliatesPage />);

    expect(screen.getByRole('status')).toHaveTextContent('Loading affiliate statistics');
    expect(screen.queryByText('12', { exact: true })).not.toBeInTheDocument();
    expect(screen.queryByText('48', { exact: true })).not.toBeInTheDocument();
    expect(screen.queryByText('$1240', { exact: true })).not.toBeInTheDocument();
    complete(Response.json({ total_affiliates: 0, total_commission_cents: 0 }));
    await waitFor(() => expect(metric('Total Affiliates')).toHaveTextContent(/^Total Affiliates0$/));
  });

  it('preserves explicit zero and marks unsupported referral counts unavailable', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json({ total_affiliates: 0, total_commission_cents: 0 })));
    render(<GrowthAffiliatesPage />);

    await waitFor(() => expect(metric('Total Affiliates')).toHaveTextContent(/^Total Affiliates0$/));
    expect(metric('Active Referrals')).toHaveTextContent('Unavailable');
    expect(metric('Commission Total (cents)')).toHaveTextContent(/^Commission Total \(cents\)0$/);
    expect(screen.queryByText('Commissions Paid', { exact: true })).not.toBeInTheDocument();
    expect(screen.queryByText('48', { exact: true })).not.toBeInTheDocument();
    expect(screen.queryByText('$1240', { exact: true })).not.toBeInTheDocument();
  });

  it('uses the actual count and signed commission cents without converting them into payouts', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json({ total_affiliates: 3, total_commission_cents: -125 })));
    render(<GrowthAffiliatesPage />);

    await waitFor(() => expect(metric('Total Affiliates')).toHaveTextContent(/^Total Affiliates3$/));
    expect(metric('Commission Total (cents)')).toHaveTextContent('-125');
    expect(metric('Active Referrals')).toHaveTextContent('Unavailable');
  });

  it.each([
    { total_affiliates: 0 },
    { total_affiliates: 0, total_commission_cents: null },
    { total_affiliates: 0, total_commission_cents: '1240' },
    { total_affiliates: 0, total_commission_cents: 1.5 },
    { total_affiliates: 0, total_commission_cents: Number.MAX_SAFE_INTEGER + 1 },
  ])('marks missing or invalid cents unavailable without discarding a valid count: %j', async (body) => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json(body)));
    render(<GrowthAffiliatesPage />);

    await waitFor(() => expect(metric('Total Affiliates')).toHaveTextContent(/^Total Affiliates0$/));
    expect(metric('Commission Total (cents)')).toHaveTextContent('Unavailable');
  });

  it.each([null, [], { total_affiliates: -1 }, { total_affiliates: '0' }, { total_affiliates: 2.5 }, { error: 'query failed', total_affiliates: 0, total_commission_cents: 0 }].map((body) => ({ body })))('never interprets an invalid statistics response as an empty account: $body', async ({ body }) => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json(body)));
    render(<GrowthAffiliatesPage />);

    await waitFor(() => expect(metric('Total Affiliates')).toHaveTextContent('Unavailable'));
    expect(metric('Active Referrals')).toHaveTextContent('Unavailable');
  });

  it.each([401, 403, 500])('shows failure and supports an actual retry after HTTP %s', async (status) => {
    const fetchStats = vi.fn()
      .mockResolvedValueOnce(Response.json({ total_affiliates: 0 }, { status }))
      .mockResolvedValueOnce(Response.json({ total_affiliates: 2, total_commission_cents: 1284 }));
    vi.stubGlobal('fetch', fetchStats);
    render(<GrowthAffiliatesPage />);

    expect(await screen.findByRole('alert')).toHaveTextContent('Affiliate statistics could not be loaded');
    expect(metric('Total Affiliates')).toHaveTextContent('Unavailable');
    fireEvent.click(screen.getByRole('button', { name: 'Retry statistics' }));
    await waitFor(() => expect(metric('Total Affiliates')).toHaveTextContent(/^Total Affiliates2$/));
    expect(metric('Commission Total (cents)')).toHaveTextContent('1,284');
    expect(screen.queryByRole('alert')).not.toBeInTheDocument();
  });

  it('keeps network failure separate from zero statistics', async () => {
    vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new TypeError('Disconnected')));
    render(<GrowthAffiliatesPage />);

    expect(await screen.findByRole('alert')).toHaveTextContent('Affiliate statistics could not be loaded');
    expect(metric('Total Affiliates')).toHaveTextContent('Unavailable');
    expect(metric('Commission Total (cents)')).toHaveTextContent('Unavailable');
  });
});
