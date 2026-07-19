import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralCountdownWidgetPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('ViralCountdownWidgetPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn(() => null),
        setItem: vi.fn(),
      },
      writable: true,
    });
    Object.defineProperty(navigator, 'clipboard', {
      value: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
      writable: true,
    });
    Object.defineProperty(window, 'location', {
      value: {
        origin: 'http://localhost:3000',
        href: '',
      },
      writable: true,
    });
  });

  it('renders the page correctly', () => {
    render(<ViralCountdownWidgetPage />);
    expect(screen.getByText('Viral Countdown Widget')).toBeDefined();
    expect(screen.getByLabelText('Event Name')).toBeDefined();
    expect(screen.getByLabelText('Target Date & Time')).toBeDefined();
  });

  it('updates state when inputs change', () => {
    render(<ViralCountdownWidgetPage />);

    const eventNameInput = screen.getByLabelText('Event Name');
    fireEvent.change(eventNameInput, { target: { value: 'New Event' } });
    expect((eventNameInput as HTMLInputElement).value).toBe('New Event');

    const themeSelect = screen.getByLabelText('Theme');
    fireEvent.change(themeSelect, { target: { value: 'dark' } });
    expect((themeSelect as HTMLSelectElement).value).toBe('dark');
  });

  it('shows paywall when trying to remove branding without pro', () => {
    render(<ViralCountdownWidgetPage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    fireEvent.click(removeBrandingCheckbox);

    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });

  it('allows removing branding if user has pro', () => {
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn((key) => {
          if (key === 'has_pro') return 'true';
          return null;
        }),
      },
      writable: true,
    });

    render(<ViralCountdownWidgetPage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    fireEvent.click(removeBrandingCheckbox);

    expect(screen.queryByText('Upgrade to Remove Branding')).toBeNull();
    expect((removeBrandingCheckbox as HTMLInputElement).checked).toBe(true);
  });

  it('copies embed code to clipboard', async () => {
    render(<ViralCountdownWidgetPage />);

    const copyButton = screen.getByText('Copy Embed Code');
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(await screen.findByText('Copied to Clipboard!')).toBeDefined();
  });
});
