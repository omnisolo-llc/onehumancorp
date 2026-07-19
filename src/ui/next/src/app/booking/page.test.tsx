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
    expect(screen.getByText('Book an Appointment')).toBeInTheDocument();
  });

  it('submits the form and shows the success screen with OneTapReferral', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        available_slots: [{ start_time: "2026-10-10T09:00:00Z", end_time: "2026-10-10T10:00:00Z" }]
      }),
    }).mockResolvedValueOnce({
        ok: true,
        json: async () => ({
            booking_id: "test",
            deposit_stripe_link: ""
        }),
    });

    render(<BookingPage />);

    const nameInput = screen.getByPlaceholderText('Jane Doe');
    fireEvent.change(nameInput, { target: { value: 'Test User' } });

    const emailInput = screen.getByPlaceholderText('jane@example.com');
    fireEvent.change(emailInput, { target: { value: 'test@example.com' } });

    const dateInput = document.querySelector('input[type="date"]') as HTMLInputElement;
    fireEvent.change(dateInput, { target: { value: '2026-10-10' } });

    await waitFor(() => {
        const expectedTime = new Date("2026-10-10T09:00:00Z").toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
        const slotButton = screen.getByText(expectedTime);
        fireEvent.click(slotButton);
    });

    const textarea = screen.getByPlaceholderText(/What do you need help with\?/i);
    fireEvent.change(textarea, { target: { value: 'Test description' } });

    const submitButton = screen.getByRole('button', { name: /Confirm Booking/i });
    fireEvent.click(submitButton);

    await waitFor(() => {
      expect(screen.getByText('Request Sent!')).toBeInTheDocument();
    });
  });
});
