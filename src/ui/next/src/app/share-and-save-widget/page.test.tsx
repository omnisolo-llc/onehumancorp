import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import ShareAndSaveWidgetPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

describe('ShareAndSaveWidgetPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'business_display_name') return 'test-tenant';
        if (key === 'has_pro') return 'false';
        return null;
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });

    global.window.open = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly', () => {
    render(<ShareAndSaveWidgetPage />);
    expect(screen.getByText('Share This Store')).toBeDefined();
    expect(screen.getByRole('button', { name: 'Share on X' })).toBeDefined();
    expect(screen.getByText(/sharing will not issue a code/i)).toBeDefined();
  });

  it('renders Powered by OHC branding by default', () => {
    render(<ShareAndSaveWidgetPage />);
    expect(screen.getByText('⚡ Powered by OHC')).toBeDefined();
  });

  it('shows paywall when trying to remove branding without pro', () => {
    render(<ShareAndSaveWidgetPage />);
    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    expect(screen.getAllByText('Upgrade to Remove Branding').length).toBeGreaterThan(0);
    expect(screen.getByText('⚡ Powered by OHC')).toBeDefined(); // branding still there
  });

  it('navigates back to dashboard', () => {
    render(<ShareAndSaveWidgetPage />);
    const backBtn = screen.getByRole('button', { name: /Back to Dashboard/i });
    fireEvent.click(backBtn);
    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });

  it('opens the share window without fabricating a discount code', async () => {
    render(<ShareAndSaveWidgetPage />);

    const shareBtn = screen.getByRole('button', { name: 'Share on X' });
    fireEvent.click(shareBtn);

    expect(global.window.open).toHaveBeenCalled();
    expect(await screen.findByText('Share window opened')).toBeDefined();
    expect(screen.getByText(/no discount code was issued/i)).toBeDefined();
    expect(screen.queryByText('SHARE10')).toBeNull();
  });
});
