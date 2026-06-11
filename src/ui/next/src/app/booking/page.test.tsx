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

  it('renders the booking form', async () => {
    render(<BookingPage />);
    await waitFor(() => {
        expect(screen.getByText('Book an Appointment')).toBeInTheDocument();
    });
  });

  it('submits the form and shows the success screen with OneTapReferral', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
        if (url.includes('/availability')) {
            return Promise.resolve({
                ok: true,
                json: async () => ({ available_slots: [{ start_time: '2026-06-06T10:00:00Z', end_time: '2026-06-06T11:00:00Z' }] })
            });
        }
        return Promise.resolve({
            ok: true,
            json: async () => ({ success: true })
        });
    });

    render(<BookingPage />);

    await waitFor(() => {
        expect(screen.queryByText('Loading slots...')).not.toBeInTheDocument();
    });

    const textarea = screen.getByPlaceholderText(/I have a leaky faucet/i);
    fireEvent.change(textarea, { target: { value: 'Test description' } });

    const slotButton = await screen.findByRole('button', { name: /10:00 AM/i });
    fireEvent.click(slotButton);

    const submitButtons = screen.getAllByRole('button', { name: /Confirm Booking/i });
    fireEvent.click(submitButtons[0]);

    await waitFor(() => {
      expect(screen.getByText('Booking Confirmed!')).toBeInTheDocument();
    });

    // Check for the OneTapReferral component
    expect(screen.getByText('Refer & Earn $50')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Copy Link/i })).toBeInTheDocument();
  });
});
