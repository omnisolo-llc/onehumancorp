/** @jsxImportSource react */
import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import DigitalBusinessCardGeneratorPage from './page';
import * as navigation from 'next/navigation';
import React from 'react';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({ push: vi.fn() })),
}));

describe('DigitalBusinessCardGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();

    const localStorageMock = {
      getItem: vi.fn().mockImplementation((key) => {
        if (key === 'tenant') return 'mock-tenant';
        if (key === 'has_pro') return 'false';
        if (key === 'ohc_dbc_shared') return 'false';
        return null;
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });

    // Mock for clipboard
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockImplementation(() => Promise.resolve()),
      },
    });
  });

  it('renders the generator page with default values', () => {
    render(<DigitalBusinessCardGeneratorPage />);

    // Check main title
    expect(screen.getByText('Digital Business Card Generator')).toBeInTheDocument();

    // Check live preview default name
    expect(screen.getByText('Jane Doe')).toBeInTheDocument();

    // Check branding checkbox exists
    expect(screen.getByText(/Remove "Powered by OHC" branding/)).toBeInTheDocument();
  });

  it('updates live preview when form is filled', () => {
    render(<DigitalBusinessCardGeneratorPage />);

    const nameInput = screen.getByPlaceholderText('e.g. Jane Doe');
    fireEvent.change(nameInput, { target: { value: 'John Smith' } });

    expect(screen.getByText('John Smith')).toBeInTheDocument();
  });

  it('shows paywall when non-pro user tries to remove branding', () => {
    render(<DigitalBusinessCardGeneratorPage />);

    const brandingCheckbox = screen.getByRole('checkbox');
    fireEvent.click(brandingCheckbox);

    expect(screen.getByRole('heading', { name: 'Upgrade to Pro' })).toBeInTheDocument();
    expect(screen.getByText('Make the Digital Business Card 100% white-labeled. Upgrade to Pro to remove the "Powered by OHC" watermark and unlock full customization.')).toBeInTheDocument();
  });
});