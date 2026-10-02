import React, { act } from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralPoweredByOmniSoloWidgetPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({
    push: vi.fn(),
  })),
}));

describe('ViralPoweredByOmniSoloWidgetPage', () => {
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

  it('renders correctly', async () => {
    await act(async () => {
      render(<ViralPoweredByOmniSoloWidgetPage />);
    });
    expect(screen.getByText('Footer Badge Generator')).toBeDefined();
  });

  it('copies embed code to clipboard', async () => {
    await act(async () => {
      render(<ViralPoweredByOmniSoloWidgetPage />);
    });
    const copyButton = screen.getAllByRole('button', { name: /Copy Embed Code/i })[0];
    await act(async () => {
      fireEvent.click(copyButton);
    });
    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByText('Copied to Clipboard!')).toBeDefined();
  });

  it('shows paywall when removing branding without pro', async () => {
    await act(async () => {
      render(<ViralPoweredByOmniSoloWidgetPage />);
    });
    const checkbox = screen.getByRole('checkbox', { name: /Remove "Powered by OmniSolo" Badge/i });
    await act(async () => {
      fireEvent.click(checkbox);
    });
    expect(screen.getAllByText('Upgrade to Remove Branding').length).toBeGreaterThan(0);
  });
});
