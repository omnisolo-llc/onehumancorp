import React from 'react';
import '@testing-library/jest-dom';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import AnalyticsPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

vi.mock('next/navigation', () => ({
  useRouter() {
    return {
      push: vi.fn(),
    };
  },
  usePathname() {
    return '/analytics';
  },
}));

describe('AnalyticsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn((key) => {
          if (key === 'has_pro') return 'false';
          if (key === 'tenant' || key === 'tenant_id') return 'my-store';
          return null;
        }),
        setItem: vi.fn(),
      },
      writable: true
    });
    vi.spyOn(window, 'alert').mockImplementation(() => {});
    vi.spyOn(window, 'open').mockImplementation(() => null);
  });

  it('renders basic analytics', () => {
    render(<TooltipProvider><AnalyticsPage /></TooltipProvider>);
    expect(screen.getByText('Business Analytics 📊')).toBeInTheDocument();
    expect(screen.getByText('Total Revenue')).toBeInTheDocument();
    expect(screen.getByText('Active Customers')).toBeInTheDocument();
    expect(screen.getByText('Conversion Rate')).toBeInTheDocument();
  });

  it('shows soft paywall when trying to unlock advanced insights', () => {
    render(<TooltipProvider><AnalyticsPage /></TooltipProvider>);

    // Check that advanced section is locked
    expect(screen.getByText('Unlock Advanced Insights')).toBeInTheDocument();

    // Click to unlock
    fireEvent.click(screen.getByText('Unlock Now'));

    // Check that soft paywall appears
    expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
    expect(screen.getByText('Share on X to unlock 7 Days Free')).toBeInTheDocument();
  });

  it('grants pro status after sharing', async () => {
    render(<TooltipProvider><AnalyticsPage /></TooltipProvider>);

    // Click to unlock
    fireEvent.click(screen.getByText('Unlock Now'));

    // Click share button
    const shareButton = screen.getByText('Share on X to unlock 7 Days Free');
    fireEvent.click(shareButton);

    expect(window.open).toHaveBeenCalledWith(
      expect.stringContaining(encodeURIComponent('http://localhost:3000/onboarding?ref=my-store')),
      '_blank'
    );
    expect(window.localStorage.setItem).toHaveBeenCalledWith('has_pro', 'true');

    await waitFor(() => {
        expect(screen.queryByText('Unlock Now')).not.toBeInTheDocument();
    }, { timeout: 1000 });
  });
});
