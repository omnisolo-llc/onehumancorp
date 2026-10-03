import React from 'react';
import { act, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { ViralUpgradePaywallWidget } from './ViralUpgradePaywallWidget';

beforeEach(() => {
  localStorage.clear();
  localStorage.setItem('business_display_name', 'not-an-account-identity');
});
afterEach(() => { vi.unstubAllGlobals(); vi.restoreAllMocks(); });

it.each([
  ['failed progress lookup', () => Response.json({ error: 'unavailable' }, { status: 503 })],
  ['legacy target reached without a grant', () => Response.json({ progress: 3, target: 3 })],
])('does not promise or fabricate a referral reward from %s', async (_scenario, response) => {
  vi.stubGlobal('fetch', vi.fn(async () => response()));
  await act(async () => { render(<ViralUpgradePaywallWidget tenantId="owner-tenant" />); });
  expect(screen.getByRole('status')).toHaveTextContent('Referral rewards are unavailable until a verified program is configured.');
  expect(screen.queryByText('Unlocked!')).not.toBeInTheDocument();
  expect(screen.queryByText(/1 \/ 3|30 days|more to unlock/)).not.toBeInTheDocument();
  expect(screen.queryByRole('button', { name: /Copy Link/ })).not.toBeInTheDocument();
  expect(screen.getByRole('link', { name: 'Review plans' })).toHaveAttribute('href', '/pricing');
});
