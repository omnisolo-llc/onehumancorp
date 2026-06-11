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
      expect(screen.getByText('Book a Service')).toBeInTheDocument();
    });
  });

  it('submits the form and shows the success screen with OneTapReferral', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
        if (url.includes('/services')) {
            return Promise.resolve({
                ok: true,
                json: async () => ({ services: [{id: 's1', title: 'Test Service', price_cents: 1000, description: 'Test'}] }),
            });
        }
        return Promise.resolve({
            ok: true,
            json: async () => ({ success: true }),
        });
    });

    render(<BookingPage />);

    await waitFor(() => {
        expect(screen.getByText('Test Service')).toBeInTheDocument();
    });

    // Select service
    fireEvent.click(screen.getByText('Test Service'));

    // Wait for the form fields to appear
    await waitFor(() => {
        expect(screen.getByPlaceholderText('Email address')).toBeInTheDocument();
    });

    // Fill email
    fireEvent.change(screen.getByPlaceholderText('Email address'), { target: { value: 'test@example.com' } });

    // Set date and time programmatically via fireEvent since they are controlled inputs
    const dateInput = document.querySelector('input[type="date"]');
    if (dateInput) {
       fireEvent.change(dateInput, { target: { value: '2025-01-01' } });
    }

    const timeInput = document.querySelector('input[type="time"]');
    if (timeInput) {
       fireEvent.change(timeInput, { target: { value: '14:00' } });
    }

    const submitButton = screen.getByRole('button', { name: /Confirm Booking & Pay Deposit/i });
    fireEvent.click(submitButton);

    await waitFor(() => {
      expect(screen.getByText('Booking Confirmed!')).toBeInTheDocument();
    });

    // Check for the OneTapReferral component
    expect(screen.getByText('Refer & Earn $50')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Copy Link/i })).toBeInTheDocument();
  });
});
