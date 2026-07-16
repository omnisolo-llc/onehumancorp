import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import InteractiveInsightWidgetPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({
    push: vi.fn(),
  })),
}));

describe('InteractiveInsightWidgetPage', () => {
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
    render(<InteractiveInsightWidgetPage />);
    expect(screen.getByText('Insight Widget Builder')).toBeDefined();
  });

  it('copies embed code to clipboard', () => {
    render(<InteractiveInsightWidgetPage />);
    const copyButton = screen.getAllByRole('button', { name: /Copy Embed Code/i })[0];
    fireEvent.click(copyButton);
    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByText('Copied to Clipboard!')).toBeDefined();
  });

  it('shows paywall when removing branding without pro', () => {
    render(<InteractiveInsightWidgetPage />);
    const checkbox = screen.getByRole('checkbox', { name: /Remove "Powered by OHC" Badge/i });
    fireEvent.click(checkbox);
    expect(screen.getAllByText('Upgrade to Remove Branding').length).toBeGreaterThan(0);
  });
});
