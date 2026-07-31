import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
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
  });

  it('renders correctly', () => {
    render(<MilestoneAlertsPage />);
    expect(screen.getByText('Success Milestone Alerts')).toBeDefined();
  });

  it('copies share text to clipboard when share is clicked (and navigator.share is undefined)', () => {
    window.alert = vi.fn();
    render(<MilestoneAlertsPage />);
    const shareButton = screen.getByRole('button', { name: /Share Milestone/i });
    fireEvent.click(shareButton);
    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(window.alert).toHaveBeenCalledWith('Milestone share text copied to clipboard!');
  });
});
