import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { LocalizationToggle } from './LocalizationToggle';
import { useLocalizationStore } from '../lib/localizationStore';

// Mock the zustand store
vi.mock('../lib/localizationStore', () => ({
  useLocalizationStore: vi.fn(),
}));

describe('LocalizationToggle', () => {
  it('renders correctly with default values', () => {
    (useLocalizationStore as any).mockReturnValue({
      locale: 'en',
      currency: 'USD',
      setLocale: vi.fn(),
      setCurrency: vi.fn(),
    });

    render(<LocalizationToggle />);
    expect(screen.getByText('🇺🇸')).toBeDefined();
    expect(screen.getByText('USD')).toBeDefined();
  });

  it('toggles dropdown when clicked', () => {
    (useLocalizationStore as any).mockReturnValue({
      locale: 'en',
      currency: 'USD',
      setLocale: vi.fn(),
      setCurrency: vi.fn(),
    });

    render(<LocalizationToggle />);

    // Dropdown should be initially closed
    expect(screen.queryByText('Language')).toBeNull();

    // Click to open
    fireEvent.click(screen.getByRole('button'));
    expect(screen.getByText('Language')).toBeDefined();
    expect(screen.getByText('Currency')).toBeDefined();
    expect(screen.getByText('العربية')).toBeDefined();

    // Click outside/overlay to close
    const overlay = screen.getByText('Language').parentElement?.parentElement?.previousElementSibling;
    if (overlay) fireEvent.click(overlay);
    expect(screen.queryByText('Language')).toBeNull();
  });

  it('calls setLocale when a language is selected', () => {
    const setLocaleMock = vi.fn();
    (useLocalizationStore as any).mockReturnValue({
      locale: 'en',
      currency: 'USD',
      setLocale: setLocaleMock,
      setCurrency: vi.fn(),
    });

    render(<LocalizationToggle />);

    // Open dropdown
    fireEvent.click(screen.getByRole('button'));

    // Select Arabic
    fireEvent.click(screen.getByText('العربية'));
    expect(setLocaleMock).toHaveBeenCalledWith('ar');
  });

  it('calls setCurrency when a currency is selected', () => {
    const setCurrencyMock = vi.fn();
    (useLocalizationStore as any).mockReturnValue({
      locale: 'en',
      currency: 'USD',
      setLocale: vi.fn(),
      setCurrency: setCurrencyMock,
    });

    render(<LocalizationToggle />);

    // Open dropdown
    fireEvent.click(screen.getByRole('button'));

    // Select EUR
    const eurButton = screen.getByText('EUR').closest('button');
    if (eurButton) fireEvent.click(eurButton);
    expect(setCurrencyMock).toHaveBeenCalledWith('EUR');
  });
});
