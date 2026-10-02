import { render, screen } from '@testing-library/react';
import { afterEach, expect, it, vi } from 'vitest';
import type { ReactNode } from 'react';
import ProposalReviewPage from './page';

vi.mock('next/navigation', () => ({
  useParams: () => ({ id: 'proposal-reviewed' }),
  useRouter: () => ({ back: vi.fn() }),
}));
vi.mock('../../components/AppShell', () => ({
  AppShell: ({ children }: { children: ReactNode }) => <main>{children}</main>,
}));
afterEach(() => vi.unstubAllGlobals());

it('loads the resolved route proposal instead of reading synchronous Next params', async () => {
  vi.stubGlobal('fetch', vi.fn(async (url: string) => new Response(JSON.stringify(
    url === '/api/v1/proposals/proposal-reviewed'
      ? { proposal: { id: 'proposal-reviewed', status: 'NEEDS_PRICING', total_amount_cents: 0, required_deposit_cents: 0 }, line_items: [{ id: 'line-1', description: 'Reviewed image', quantity: 2, unit_price_cents: 1200 }] }
      : { error: 'not found' },
  ), { status: url === '/api/v1/proposals/proposal-reviewed' ? 200 : 404 })));

  render(<ProposalReviewPage />);
  expect(await screen.findByText('NEEDS_PRICING')).toBeVisible();
  expect(screen.getByText('Reviewed image (x2)')).toBeVisible();
  expect(screen.queryByRole('button', { name: /Approve.*Send/ })).toBeNull();
});
