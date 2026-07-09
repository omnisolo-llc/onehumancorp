import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import AffiliateBadgeBuilderPage from './page';

describe('AffiliateBadgeBuilderPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(() => Promise.resolve()),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant_id') return 'test-tenant-123';
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
    render(<AffiliateBadgeBuilderPage />);
    expect(screen.getByText('Affiliate Badge Builder')).toBeDefined();
    expect(screen.getByText('Share OHC & Earn Credits')).toBeDefined();
  });

  it('updates badge text when typed', () => {
    render(<AffiliateBadgeBuilderPage />);
    const input = screen.getByLabelText(/Badge Text/i);
    fireEvent.change(input, { target: { value: 'New Test Badge' } });
    const elements = screen.getAllByText('New Test Badge');
    expect(elements.length).toBeGreaterThan(0);
  });

  it('opens modal and copies code to clipboard', async () => {
    render(<AffiliateBadgeBuilderPage />);
    const getCodeBtn = screen.getByRole('button', { name: /Get Embed Code/i });
    fireEvent.click(getCodeBtn);

    await waitFor(() => {
      expect(screen.getByText('Embed Badge')).toBeDefined();
    });

    const copyBtn = screen.getByRole('button', { name: /Copy Code/i });
    fireEvent.click(copyBtn);

    await waitFor(() => {
      expect(navigator.clipboard.writeText).toHaveBeenCalled();
      expect(screen.getByText('Copied!')).toBeDefined();
    });
  });

  it('changes theme style', () => {
    render(<AffiliateBadgeBuilderPage />);
    const lightBtn = screen.getByRole('button', { name: /Light/i });
    fireEvent.click(lightBtn);
    expect(lightBtn.className).toContain('border-indigo-600');
  });
});
