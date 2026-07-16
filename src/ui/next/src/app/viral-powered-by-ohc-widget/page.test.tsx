import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
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
    expect(screen.getByText('Viral Widget Builder')).toBeDefined();
  });

  it('copies embed code to clipboard', () => {
    render(<ViralPoweredByOHCWidgetPage />);
    const copyButton = screen.getAllByRole('button', { name: /Copy Embed Code/i })[0];
    fireEvent.click(copyButton);
    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByText('Copied to Clipboard!')).toBeDefined();
  });

  it('shows paywall when removing branding without pro', () => {
    render(<ViralPoweredByOHCWidgetPage />);
    const checkbox = screen.getByRole('checkbox', { name: /Remove "Powered by OHC"/i });
    fireEvent.click(checkbox);
    expect(screen.getAllByText('Upgrade to Remove Branding').length).toBeGreaterThan(0);
  });

  it('updates title in iframe preview', async () => {
    render(<ViralPoweredByOHCWidgetPage />);
    const titleInput = screen.getByRole('textbox', { name: /Widget Title/i });
    fireEvent.change(titleInput, { target: { value: 'New Test Title' } });

    await waitFor(() => {
      const iframe = screen.getByTitle('Widget Preview');
      expect(iframe.getAttribute('src')).toContain('title=New%20Test%20Title');
    });
  });

  it('updates theme in iframe preview', async () => {
    render(<ViralPoweredByOHCWidgetPage />);
    const themeSelect = screen.getByRole('combobox', { name: /Theme/i });
    fireEvent.change(themeSelect, { target: { value: 'dark' } });

    await waitFor(() => {
      const iframe = screen.getByTitle('Widget Preview');
      expect(iframe.getAttribute('src')).toContain('theme=dark');
    });
  });

  it('generates embed code with updated parameters', async () => {
    render(<ViralPoweredByOHCWidgetPage />);
    const titleInput = screen.getByRole('textbox', { name: /Widget Title/i });
    fireEvent.change(titleInput, { target: { value: 'Code Test' } });

    await waitFor(() => {
      const pre = document.querySelector('pre');
      expect(pre?.textContent).toContain('title=Code%20Test');
    });
  });
});
