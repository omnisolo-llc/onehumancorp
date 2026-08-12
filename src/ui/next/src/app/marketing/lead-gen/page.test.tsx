import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import LeadGenCampaignPage from './page';
import { beforeEach, describe, it, expect, vi } from 'vitest';

describe('LeadGenCampaignPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ id: 'campaign-1', status: 'Active' }) });
  });

  it('renders the page correctly', () => {
    render(<LeadGenCampaignPage />);
    expect(screen.getByText('Local Lead Generator')).toBeInTheDocument();
  });

  it('creates a campaign with the entered budget and location', async () => {
    render(<LeadGenCampaignPage />);
    fireEvent.change(screen.getByLabelText('Weekly Budget ($)'), { target: { value: '50' } });
    fireEvent.change(screen.getByLabelText('Target Zip Code'), { target: { value: '90210' } });
    fireEvent.click(screen.getByRole('button', { name: 'Start Finding Jobs' }));

    await waitFor(() => expect(screen.getByText(/Campaign Started!/)).toBeInTheDocument());
    expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/campaign/lead-gen', expect.objectContaining({
      method: 'POST',
      body: JSON.stringify({ budget: 50, radius_miles: 10, zip_code: '90210' }),
    }));
  });
});
