import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
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
    expect(screen.getByText('Unlock 10% Off!')).toBeDefined();
    expect(screen.getByText('Share on X to Unlock')).toBeDefined();
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

  it('handles sharing on twitter and revealing code', async () => {
    render(<ShareAndSaveWidgetPage />);

    const shareBtn = screen.getByRole('button', { name: /Share on X to Unlock/i });
    fireEvent.click(shareBtn);

    expect(global.window.open).toHaveBeenCalled();

    // Wait for real timer since fake timers is breaking React 18 / RTL
    await waitFor(() => {
        expect(screen.getByText('Unlocked!')).toBeDefined();
        expect(screen.getByText('SHARE10')).toBeDefined();
    }, { timeout: 2000 });
  });
});
