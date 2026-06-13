import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import AffiliateBadgeBuilderPage from './page';

describe('AffiliateBadgeBuilderPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn(() => 'test-tenant'),
      },
      writable: true
    });
  });

  it('renders the badge builder page', () => {
    render(<AffiliateBadgeBuilderPage />);
    expect(screen.getByText('Affiliate Badge Builder')).toBeDefined();
    expect(screen.getByText('Share OHC & Earn Credits')).toBeDefined();
    expect(screen.getByText('Customize Your Badge')).toBeDefined();
  });

  it('updates live preview text when input changes', () => {
    render(<AffiliateBadgeBuilderPage />);

    // The "LIVE PREVIEW" badge wrapper also contains the text, so we check for exact matches if needed
    // or just rely on the input value changing the DOM.
    const input = screen.getByDisplayValue('Powered by OHC');
    fireEvent.change(input, { target: { value: 'Made with OHC' } });

    // The text should now be 'Made with OHC'
    expect(screen.getAllByText('Made with OHC').length).toBeGreaterThan(0);
  });

  it('generates the correct embed code', () => {
    render(<AffiliateBadgeBuilderPage />);

    const embedButton = screen.getByText('Get Embed Code');
    fireEvent.click(embedButton);

    const embedCodeSection = screen.getByText(/<!-- OHC Affiliate Badge -->/);
    expect(embedCodeSection).toBeDefined();

    // Check that it contains the tenant ID in the affiliate link
    expect(embedCodeSection.textContent).toContain('ref=test-tenant');
    expect(embedCodeSection.textContent).toContain('Powered by OHC');
  });

  it('changes theme style in embed code when theme buttons are clicked', () => {
    render(<AffiliateBadgeBuilderPage />);

    const lightThemeBtn = screen.getByText('Light');
    fireEvent.click(lightThemeBtn);

    const embedButton = screen.getByText('Get Embed Code');
    fireEvent.click(embedButton);

    const embedCodeSection = screen.getByText(/<!-- OHC Affiliate Badge -->/);
    // Light theme uses white background
    expect(embedCodeSection.textContent).toContain('background-color: #ffffff');
  });
});
