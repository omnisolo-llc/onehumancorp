import React from 'react';
import { render, screen, act, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import StoreWrapPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn() }),
}));

describe('StoreWrapPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ total_sales: 1000, active_customers: 50 })
    });

    const localStorageMock = {
      getItem: vi.fn().mockReturnValue('mock-tenant'),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });
  });

  it('renders store wrap list correctly', async () => {
    await act(async () => {
      render(<StoreWrapPage />);
    });

    expect(screen.getByText('Store Wrap-Up 🎁')).toBeDefined();
    expect(screen.getAllByRole('link', { name: /powered by ohc/i }).length).toBeGreaterThan(0);
  });

  it('shows soft paywall when attempting to remove branding without Pro', async () => {
    await act(async () => {
      render(<StoreWrapPage />);
    });

    // Click checkbox
    const checkbox = screen.getByRole('checkbox', { name: /Remove Badge/i });

    await act(async () => {
      fireEvent.click(checkbox);
    });

    // Paywall should appear
    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
    expect(screen.getByText(/Make the Store Wrap 100% yours/i)).toBeDefined();
  });

  it('removes branding without paywall when user has Pro', async () => {
    // Override local storage mock to return Pro
    const proLocalStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'ohc_plan') return 'Pro';
        return 'mock-tenant';
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: proLocalStorageMock,
      writable: true
    });

    await act(async () => {
      render(<StoreWrapPage />);
    });

    // Verify Powered by OHC is visible initially
    expect(screen.getAllByRole('link', { name: /powered by ohc/i }).length).toBeGreaterThan(0);

    // Click checkbox
    const checkbox = screen.getByRole('checkbox', { name: /Remove Badge/i });

    await act(async () => {
      fireEvent.click(checkbox);
    });

    // Paywall should NOT appear
    expect(screen.queryByText('Upgrade to Remove Branding')).toBeNull();

    // Powered by OHC should be hidden
    expect(screen.queryByRole('link', { name: /powered by ohc/i })).toBeNull();
  });
});
