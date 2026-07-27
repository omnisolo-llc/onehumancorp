import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import '@testing-library/jest-dom/vitest';
import WinBackCampaignPage from './page';

vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));

describe('WinBackCampaignPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url === '/api/v1/billing/my-plan') return Promise.resolve({ ok: true, json: async () => ({ current_plan: 'pro' }) });
      if (url === '/api/v1/growth/campaign/generate-win-back') return Promise.resolve({ ok: true, json: async () => ({ subject: 'Come back', body: 'We miss you' }) });
      return Promise.resolve({ ok: false, json: async () => ({}) });
    });
  });

  it('renders the backend draft but does not offer fabricated campaign dispatch', async () => {
    render(<WinBackCampaignPage />);
    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/my-plan'));
    fireEvent.change(screen.getByLabelText('Discount Offer (%)'), { target: { value: '15' } });
    fireEvent.click(screen.getByRole('button', { name: 'Generate AI Campaign' }));
    expect(await screen.findByText(/Subject: Come back/)).toBeDefined();
    expect(screen.getByRole('button', { name: 'Campaign sending unavailable' })).toBeDisabled();
    expect(global.fetch).not.toHaveBeenCalledWith('/api/v1/growth/campaign/send', expect.anything());
  });

});
