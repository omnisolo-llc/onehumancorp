import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralPoweredByOHCWidgetPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({
    push: vi.fn(),
  })),
}));

describe('ViralPoweredByOHCWidgetPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant') return 'test-tenant';
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
  });

  it('renders correctly', () => {
    render(<ViralPoweredByOHCWidgetPage />);
    expect(screen.getByText('Footer Badge Generator')).toBeDefined();
  });

  it('copies embed code to clipboard', () => {
    render(<ViralPoweredByOHCWidgetPage />);
    const copyButton = screen.getByRole('button', { name: /Copy Embed Code/i });
    fireEvent.click(copyButton);
    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByText('Copied to Clipboard!')).toBeDefined();
  });

  it('shows paywall when removing branding without pro', () => {
    render(<ViralPoweredByOHCWidgetPage />);
    const checkbox = screen.getByRole('checkbox', { name: /Remove "Powered by OHC" Badge/i });
    fireEvent.click(checkbox);
    expect(screen.getAllByText('Upgrade to Remove Branding').length).toBeGreaterThan(0);
  });
});
