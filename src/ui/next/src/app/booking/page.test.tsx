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

    const nameInput = screen.getByPlaceholderText('First Last');
    fireEvent.change(nameInput, { target: { value: 'Test User' } });

    const emailInput = screen.getByPlaceholderText('email@example.com');
    fireEvent.change(emailInput, { target: { value: 'test@example.com' } });

    const dateInput = screen.getByLabelText('Select a Date');
    fireEvent.change(dateInput, { target: { value: '2026-10-10' } });

    const slotButton = screen.getByRole('button', { name: '09:00 AM' });
    fireEvent.click(slotButton);

    const textarea = screen.getByPlaceholderText(/Any details we should know before the appointment/i);
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
