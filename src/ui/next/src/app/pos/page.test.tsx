import { render, screen } from '@testing-library/react';
import { redirect } from 'next/navigation';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import POSPage from './page';

beforeEach(() => {
  localStorage.clear();
  vi.clearAllMocks();
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ success: false }, { status: 503 })));
});
afterEach(() => { vi.unstubAllGlobals(); });

it('routes the legacy POS entry to the maintained terminal without rendering a second catalog', () => {
  render(<POSPage />);
  expect(redirect).toHaveBeenCalledWith('/pos/terminal');
  expect(screen.queryByText('Custom Cake')).toBeNull();
  expect(screen.queryByRole('button', { name: 'Charge via Tap-to-Pay' })).toBeNull();
  expect(fetch).not.toHaveBeenCalled();
});

it('does not resend or remove historical unconfirmed payments when opening the legacy entry', async () => {
  const raw = '[ { "offline_id": "unknown-outcome", "total": 70 } ]';
  localStorage.setItem('pos_offline_queue', raw);
  render(<POSPage />);
  expect(fetch).not.toHaveBeenCalled();
  expect(localStorage.getItem('pos_offline_queue')).toBe(raw);
});
