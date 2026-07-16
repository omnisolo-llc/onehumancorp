import React from 'react';
import { render, screen, act, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import MilestoneAlertsPage from './page';
import * as navigation from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({ push: vi.fn() })),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('MilestoneAlertsPage', () => {
  const mockPush = vi.fn();

  beforeEach(() => {
    (navigation.useRouter as any).mockReturnValue({ push: mockPush });
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        milestones: [
          { id: 'first_sale', title: 'First Sale!', description: 'Congrats!', reached: true },
          { id: '10th_order', title: '10th Order!', description: 'Booming!', reached: false }
        ]
      })
    });

    const localStorageMock = {
      getItem: vi.fn().mockReturnValue('mock-tenant'),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });
  });

  it('renders milestones list correctly', async () => {
    await act(async () => {
      render(<MilestoneAlertsPage />);
    });

    expect(screen.getByText('Your Achievements')).toBeDefined();
    expect(screen.getByText('First Sale!')).toBeDefined();
  });

  it('shows card preview and embed generator after clicking a milestone', async () => {
    await act(async () => {
      render(<MilestoneAlertsPage />);
    });

    const milestoneTitle = screen.getByText('First Sale!');
    const container = milestoneTitle.closest('div.glassmorphism');
    expect(container).toBeDefined();

    await act(async () => {
        fireEvent.click(container!);
    });

    // Verify embed generator
    expect(screen.getByText('Embed on your website')).toBeDefined();

    const textarea = document.querySelector('textarea');
    expect(textarea).toBeDefined();
    expect(textarea?.value).toContain('mock-tenant');
    expect(textarea?.value).toContain('milestone_id=first_sale');
  });

  it('contains the invite a friend CTA and navigates to referrals', async () => {
    await act(async () => {
      render(<MilestoneAlertsPage />);
    });

    const milestoneTitle = screen.getByText('First Sale!');
    const container = milestoneTitle.closest('div.glassmorphism');

    await act(async () => {
        fireEvent.click(container!);
    });

    const inviteButton = screen.getByText('Invite a friend and get a $50 credit');
    expect(inviteButton).toBeDefined();

    await act(async () => {
      fireEvent.click(inviteButton);
    });

    expect(mockPush).toHaveBeenCalledWith('/referrals?ref=milestone');
  });

  it('contains WhatsApp and Facebook share buttons', async () => {
    await act(async () => {
      render(<MilestoneAlertsPage />);
    });

    const milestoneTitle = screen.getByText('First Sale!');
    const container = milestoneTitle.closest('div.glassmorphism');

    await act(async () => {
        fireEvent.click(container!);
    });

    const whatsappButton = screen.getByText('Share to WhatsApp');
    expect(whatsappButton).toBeDefined();

    const facebookButton = screen.getByText('Share on Facebook');
    expect(facebookButton).toBeDefined();
  });

  it('renders the PoweredByOHC component', async () => {
    await act(async () => {
      render(<MilestoneAlertsPage />);
    });

    expect(screen.getByTestId('powered-by-ohc')).toBeDefined();
  });

  it('shows soft paywall when trying to remove branding without Pro', async () => {
    await act(async () => {
      render(<MilestoneAlertsPage />);
    });

    const milestoneTitle = screen.getByText('First Sale!');
    const container = milestoneTitle.closest('div.glassmorphism');

    await act(async () => {
        fireEvent.click(container!);
    });

    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    expect(removeBrandingCheckbox).toBeDefined();

    await act(async () => {
      fireEvent.click(removeBrandingCheckbox);
    });

    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });

});
