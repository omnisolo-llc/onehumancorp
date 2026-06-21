/**
 * @vitest-environment jsdom
 */

import React from 'react';
import { render, screen, act, fireEvent, cleanup } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import WaitlistPage from './page';
import * as navigation from 'next/navigation';

// Mock navigation
vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({ push: vi.fn() })),
}));

// Mock PoweredByOHC component
vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('WaitlistPage', () => {
  const mockPush = vi.fn();

  beforeEach(() => {
    (navigation.useRouter as any).mockReturnValue({ push: mockPush });
    vi.clearAllMocks();

    // Clear mock fetch
    global.fetch = vi.fn();
  });

  afterEach(() => {
    cleanup();
  });

  it('renders the initial waitlist form', async () => {
    await act(async () => {
      render(<WaitlistPage />);
    });

    expect(screen.getByText('The AI platform for small business.')).toBeDefined();
    expect(screen.getByPlaceholderText('Enter your email address')).toBeDefined();
    expect(screen.getByText('Join the Waitlist')).toBeDefined();
  });

  it('handles successful form submission and shows success state with PoweredByOHC component', async () => {
    const mockResponse = {
      ok: true,
      json: async () => ({
        position: 42,
        referral_link: 'https://example.com/ref/123'
      })
    };
    (global.fetch as any).mockResolvedValue(mockResponse);

    await act(async () => {
      render(<WaitlistPage />);
    });

    const emailInput = screen.getByPlaceholderText('Enter your email address');
    const submitButton = screen.getByText('Join the Waitlist');

    await act(async () => {
      fireEvent.change(emailInput, { target: { value: 'test@example.com' } });
      fireEvent.click(submitButton);
    });

    // Check fetch arguments
    expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/waitlist', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ email: 'test@example.com' }),
    });

    // Verify success state content
    expect(screen.getByText("You're #42 on the list!")).toBeDefined();
    expect(screen.getByText('Move up the list!')).toBeDefined();

    // Verify PoweredByOHC component is rendered in success state
    expect(screen.getByTestId('powered-by-ohc')).toBeDefined();
  });

  it('handles submission errors', async () => {
    const mockResponse = {
      ok: false,
      json: async () => ({})
    };
    (global.fetch as any).mockResolvedValue(mockResponse);

    await act(async () => {
      render(<WaitlistPage />);
    });

    const emailInput = screen.getByPlaceholderText('Enter your email address');
    const submitButton = screen.getByText('Join the Waitlist');

    await act(async () => {
      fireEvent.change(emailInput, { target: { value: 'test@example.com' } });
      fireEvent.click(submitButton);
    });

    // Verify error message
    expect(screen.getByText('Failed to join waitlist. Please try again.')).toBeDefined();

    // Verify we're still on the initial form
    expect(screen.getByText('The AI platform for small business.')).toBeDefined();
  });
});
