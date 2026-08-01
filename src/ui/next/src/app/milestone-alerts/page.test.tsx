import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import MilestoneAlertsPage from './page';

vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: any) => <div data-testid="app-shell">{children}</div>,
}));

describe('MilestoneAlertsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'business_display_name') return 'test-tenant';
        return null;
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });

    global.fetch = vi.fn().mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({
            milestones: [
                { id: 1, title: '10th Order!', message: 'Congratulations on reaching 10 orders! Share the good news with your network.', icon: '🎉', achieved: true },
                { id: 2, title: '100th Customer', message: 'You just welcomed your 100th customer. Keep growing!', icon: '💯', achieved: false },
                { id: 3, title: '$10k Revenue', message: 'You have reached $10,000 in revenue. Amazing milestone!', icon: '💸', achieved: false }
            ]
        })
    }) as any;
  });

  it('renders correctly', async () => {
    render(<MilestoneAlertsPage />);
    await waitFor(() => {
      expect(screen.getByText('Success Milestone Alerts')).toBeDefined();
    });
  });

  it('copies share text to clipboard when share is clicked (and navigator.share is undefined)', async () => {
    window.alert = vi.fn();
    render(<MilestoneAlertsPage />);
    await waitFor(() => screen.getByRole('button', { name: /Share Milestone/i }));
    const shareButton = screen.getByRole('button', { name: /Share Milestone/i });
    fireEvent.click(shareButton);
    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(window.alert).toHaveBeenCalledWith('Milestone share text copied to clipboard!');
  });
});
