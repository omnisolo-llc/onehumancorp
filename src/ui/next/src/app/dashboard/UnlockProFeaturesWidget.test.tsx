import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { UnlockProFeaturesWidget } from './UnlockProFeaturesWidget';

const link = 'https://omnisolo.co/invite/recorded-unlock-test';
let count = 1;
describe('UnlockProFeaturesWidget', () => {
  beforeEach(() => {
    localStorage.clear(); count = 1;
    Object.defineProperty(navigator, 'locks', { value: { request: async (_name: string, _options: unknown, callback: (lock: object) => Promise<void>) => callback({}) } });
    Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: vi.fn().mockResolvedValue(undefined) } });
    vi.stubGlobal('fetch', vi.fn(async url => url === '/api/v1/auth/session-identity'
      ? Response.json({ userId: 'test-owner', tenantId: 'test-tenant', expiresAt: Date.now() + 60_000 })
      : url === '/api/v1/growth/team-invites/aggregated-metrics' ? Response.json({ total_invites: count })
        : Response.json({ invite_link: link })));
  });
  afterEach(() => { vi.unstubAllGlobals(); vi.restoreAllMocks(); });
  it('renders progress bar and title correctly', async () => {
    render(<UnlockProFeaturesWidget />);
    expect(await screen.findByText(/Referral Progress/i)).toBeDefined();
    expect(await screen.findByText(/1 \/ 3 Invites/i)).toBeDefined();
    expect(screen.getByRole('progressbar', { name: 'Recorded invitation progress' })).toHaveAttribute('aria-valuenow', '1');
  });
  it('copies share link to clipboard and updates button state', async () => {
    render(<UnlockProFeaturesWidget />);
    const create = screen.getByRole('button', { name: 'Create Invite Link' });
    await waitFor(() => expect(create).toBeEnabled()); fireEvent.click(create);
    fireEvent.click(await screen.findByRole('button', { name: 'Copy Invite Link' }));
    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(link);
    expect(await screen.findByText(/Copied Link!/i)).toBeDefined();
  });
  it('shows a reached target without claiming Pro entitlement', async () => {
    count = 3; render(<UnlockProFeaturesWidget />);
    expect(await screen.findByText(/Invite target reached/i)).toBeDefined();
    expect(screen.getByText(/Billing verification is required/)).toBeDefined();
    expect(screen.queryByText(/Pro Features Unlocked!/i)).toBeNull();
    expect(screen.queryByText(/Copy Invite Link/i)).toBeNull();
  });
});
