import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import WinBackCampaignPage from './page';

vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));

describe('WinBackCampaignPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url === '/api/v1/auth/session-identity') return Promise.resolve(Response.json({ userId: 'win-owner', tenantId: 'win-tenant', expiresAt: Date.now() + 60000 }));
      if (url === '/api/v1/billing/my-plan') return Promise.resolve(Response.json({ current_plan: 'Pro' }));
      if (url === '/api/v1/growth/campaign/generate-win-back') return Promise.resolve({ ok: true, json: async () => ({ subject: 'Come back', body: 'We miss you' }) });
      return Promise.resolve({ ok: false, json: async () => ({}) });
    });
  });

  it('renders the backend draft but does not offer fabricated campaign dispatch', async () => {
    render(<WinBackCampaignPage />);
    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/my-plan', expect.objectContaining({ credentials: 'same-origin' })));
    fireEvent.change(screen.getByLabelText('Discount Offer (%)'), { target: { value: '15' } });
    fireEvent.click(screen.getByRole('button', { name: 'Generate Campaign Template' }));
    const draft = await screen.findByRole('textbox', { name: 'Campaign draft' });
    expect(draft).toHaveValue('Subject: Come back\n\nWe miss you');
    fireEvent.change(draft, { target: { value: 'Owner-edited campaign' } });
    expect(draft).toHaveValue('Owner-edited campaign');
    expect(screen.getByText(/template does not create a discount code/i)).toBeVisible();
    expect(screen.getByRole('button', { name: 'Campaign sending unavailable' })).toBeDisabled();
    expect(global.fetch).not.toHaveBeenCalledWith('/api/v1/growth/campaign/send', expect.anything());
  });

  it('allows a free owner to close the paywall without losing offer inputs', async () => {
    vi.mocked(fetch).mockImplementation(async input => input === '/api/v1/auth/session-identity'
      ? Response.json({ userId: 'win-owner', tenantId: 'win-tenant', expiresAt: Date.now() + 60000 })
      : Response.json({ current_plan: 'Free' }));
    render(<WinBackCampaignPage />);
    const offer = screen.getByLabelText('Discount Offer (%)');
    fireEvent.change(offer, { target: { value: '15' } });
    const generate = screen.getByRole('button', { name: 'Generate Campaign Template' });
    await waitFor(() => expect(generate).toBeEnabled());
    fireEvent.click(generate);
    fireEvent.click(screen.getByRole('button', { name: 'Check trial availability' }));
    fireEvent.click(screen.getByRole('button', { name: 'Close paywall' }));
    expect(screen.queryByRole('heading', { name: 'Upgrade to Pro' })).not.toBeInTheDocument();
    expect(offer).toHaveValue(15);
  });

});
