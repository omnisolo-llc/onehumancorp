import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import ReferralMilestonesWidget from './ReferralMilestonesWidget';
import { vi, describe, it, expect, beforeEach } from 'vitest';

const mockFetch = vi.fn();
global.fetch = mockFetch;

Object.assign(navigator, {
  clipboard: {
    writeText: vi.fn(),
  },
});

describe('ReferralMilestonesWidget', () => {
  beforeEach(() => {
    mockFetch.mockClear();
  });

  it('renders loading state initially', () => {
    mockFetch.mockImplementationOnce(() => new Promise(() => {})); // Never resolves
    const { container } = render(<ReferralMilestonesWidget tenantId="test-tenant" />);
    expect(container.querySelector('.animate-pulse')).toBeInTheDocument();
  });

  it('renders milestones when data is loaded', async () => {
    const mockData = {
      tenant_id: "test-tenant",
      total_referrals: 2,
      milestones: [
        { target: 1, title: "First Referral", reward: "$10 Credit", reached: true },
        { target: 5, title: "Team Builder", reward: "1 Month Free Pro", reached: false }
      ]
    };

    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => mockData
    });

    render(<ReferralMilestonesWidget tenantId="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Unlock Rewards')).toBeInTheDocument();
    });

    expect(screen.getByText('First Referral')).toBeInTheDocument();
    expect(screen.getByText('Team Builder')).toBeInTheDocument();
    expect(screen.getByText('2 Referrals')).toBeInTheDocument();
    expect(screen.getByText('Progress to Team Builder')).toBeInTheDocument();
    expect(screen.getByText('3 more to get 1 Month Free Pro')).toBeInTheDocument();
  });

  it('handles copy link button', async () => {
    const mockData = {
      tenant_id: "test-tenant",
      total_referrals: 0,
      milestones: [
        { target: 1, title: "First Referral", reward: "$10 Credit", reached: false }
      ]
    };

    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => mockData
    });

    render(<ReferralMilestonesWidget tenantId="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Copy Referral Link')).toBeInTheDocument();
    });

    const user = userEvent.setup();
    await user.click(screen.getByText('Copy Referral Link'));

    expect(screen.getByText('Copied!')).toBeInTheDocument();
  });

  it('renders nothing when data fetch fails', async () => {
    mockFetch.mockRejectedValueOnce(new Error('Network error'));

    const { container } = render(<ReferralMilestonesWidget tenantId="test-tenant" />);

    await waitFor(() => {
        expect(mockFetch).toHaveBeenCalled();
    });

    // Component returns null on error
    expect(container.firstChild).toBeNull();
  });
});
