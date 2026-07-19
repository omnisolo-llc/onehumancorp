import { fireEvent, render, screen } from '@testing-library/react';
import ReviewCampaignsPage from './page';
import { beforeEach, describe, it, expect, vi } from 'vitest';

vi.mock('next/navigation', () => {
    return {
        useRouter: () => ({ push: vi.fn() })
    };
});

describe('ReviewCampaignsPage', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    vi.unstubAllGlobals();
    localStorage.clear();
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ current_plan: 'Free' }),
    }));
  });

  it('renders the page correctly', () => {
    render(<ReviewCampaignsPage />);
    expect(screen.getByText('Automated Review Campaigns ⭐️')).toBeInTheDocument();
  });

  it('shows an unavailable state instead of fabricating an email when generation fails', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({ ok: false, status: 503 }));
    render(<ReviewCampaignsPage />);

    fireEvent.click(screen.getByRole('button', { name: 'Generate Email Draft' }));

    expect(await screen.findByText('Review campaign generation is unavailable.')).toBeInTheDocument();
    expect(screen.queryByText(/Hi \[Customer Name\]/)).not.toBeInTheDocument();
  });

  it('does not offer dispatch while the backend only fabricates send counts', async () => {
    const fetchMock = vi.fn()
      .mockResolvedValueOnce({ ok: true, json: async () => ({ message: 'Backend draft' }) });
    vi.stubGlobal('fetch', fetchMock);
    render(<ReviewCampaignsPage />);

    fireEvent.click(screen.getByRole('button', { name: 'Generate Email Draft' }));
    expect(await screen.findByText('Backend draft')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Campaign sending unavailable' })).toBeDisabled();
    expect(fetchMock).not.toHaveBeenCalledWith('/api/v1/growth/campaign/send', expect.anything());
  });
});
