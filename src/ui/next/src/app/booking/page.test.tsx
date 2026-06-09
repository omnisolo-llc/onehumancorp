import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import BookingPage from './page';
import * as React from 'react';

const mockUseSearchParams = vi.fn();

vi.mock('next/navigation', () => ({
  useSearchParams: () => mockUseSearchParams(),
}));

global.fetch = vi.fn();

describe('BookingPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseSearchParams.mockReturnValue(new URLSearchParams('?tenant=test-store'));
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({ success: true }),
    });
  });

  afterEach(() => {
    mockUseSearchParams.mockReturnValue(new URLSearchParams(''));
  });

  it('renders the booking form', () => {
    render(<BookingPage />);
    expect(screen.getByText('Request a Service')).toBeInTheDocument();
  });

  it('submits the form and shows the success screen with OneTapReferral', async () => {
    render(<BookingPage />);

    const textarea = screen.getByPlaceholderText(/I have a leaky faucet/i);
    fireEvent.change(textarea, { target: { value: 'Test description' } });

    const submitButton = screen.getByRole('button', { name: /Get a Quote/i });
    fireEvent.click(submitButton);

    await waitFor(() => {
      expect(screen.getByText('Request Sent!')).toBeInTheDocument();
    });

    // Check for the OneTapReferral component
    expect(screen.getByText('Refer & Earn $50')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Copy Link/i })).toBeInTheDocument();
  });
});
