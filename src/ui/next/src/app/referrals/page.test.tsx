import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ReferralsPage from './page';

describe('ReferralsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn();

    // Mock localStorage
    const localStorageMock = {
      getItem: vi.fn(),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock
    });
  });

  it('renders loading state initially', async () => {
    // Return an unresolved promise to keep it in loading state
    (global.fetch as any).mockImplementation(() => new Promise(() => {}));

    render(<ReferralsPage />);
    expect(screen.getAllByText('Generating your unique link...')[0]).toBeDefined();

    // Copy button should be disabled
    const copyButton = screen.getAllByText('Copy Link')[0];
    expect(copyButton.hasAttribute('disabled')).toBe(true);
  });

  it('renders how it works section', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ referral_link: 'https://ohc.app/ref/test1234' }),
    });
    await act(async () => {
      render(<ReferralsPage />);
    });
    expect(screen.getByText('How it works')).toBeDefined();
    expect(screen.getByText('Share Link')).toBeDefined();
    expect(screen.getByText('They Sign Up')).toBeDefined();
    expect(screen.getByText('You Get $50')).toBeDefined();
  });

  it('fetches and displays dynamic referral link', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ referral_link: 'https://ohc.app/ref/test1234' }),
    });

    render(<ReferralsPage />);

    // Wait for the fetch to resolve
    await waitFor(() => {
      expect(screen.queryByText('Generating your unique link...')).toBeNull();
    });

    const referralSpan = document.getElementById('referral-link');
    expect(referralSpan?.textContent).toBe('https://ohc.app/ref/test1234');

    // Copy button should be enabled
    const copyButton = screen.getAllByText('Copy Link')[0];
    expect(copyButton.hasAttribute('disabled')).toBe(false);
  });

  it('falls back to tenant link on api error', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    (global.fetch as any).mockRejectedValueOnce(new Error('API failed'));
    (window.localStorage.getItem as any).mockReturnValue('my-tenant-store');

    render(<ReferralsPage />);

    await waitFor(() => {
      expect(screen.queryByText('Generating your unique link...')).toBeNull();
    });

    const referralSpan = document.getElementById('referral-link');
    expect(referralSpan?.textContent).toBe('http://localhost:3000/onboarding?ref=my-tenant-store');

    consoleErrorSpy.mockRestore();
  });
});
