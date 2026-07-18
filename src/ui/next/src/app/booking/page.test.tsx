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
    mockUseSearchParams.mockReturnValue(new URLSearchParams('tenant=test-store&service_id=service-real'));
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

  it('requires explicit tenant and service context before loading availability', async () => {
    mockUseSearchParams.mockReturnValue(new URLSearchParams(''));
    render(<BookingPage />);

    expect(screen.getByRole('alert')).toHaveTextContent('A valid booking link is required.');
    expect(screen.queryByText('Book an Appointment')).toBeNull();
    expect(global.fetch).not.toHaveBeenCalled();
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
      expect(screen.getByText('Booking request confirmed.')).toBeInTheDocument();
    });

    const reserveCall = vi.mocked(global.fetch).mock.calls.at(-1);
    expect(reserveCall?.[0]).toBe('/api/v1/booking/engine/reserve');
    const reserveBody = JSON.parse(String((reserveCall?.[1] as RequestInit | undefined)?.body));
    expect(reserveBody).toMatchObject({
      customer_name: 'Test User',
      customer_email: 'test@example.com',
      product_id: 'service-real',
    });
    expect(reserveBody).not.toHaveProperty('customer_id');
    expect(reserveBody).not.toHaveProperty('tenant_id');
  });

  it('keeps the form visible when reservation confirmation fails', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        available_slots: [{ start_time: "2026-10-10T09:00:00Z", end_time: "2026-10-10T10:00:00Z" }]
      }),
    }).mockResolvedValueOnce({ ok: false, json: async () => ({}) });

    render(<BookingPage />);
    fireEvent.change(screen.getByPlaceholderText('Jane Doe'), { target: { value: 'Test User' } });
    fireEvent.change(screen.getByPlaceholderText('jane@example.com'), { target: { value: 'test@example.com' } });
    fireEvent.change(document.querySelector('input[type="date"]') as HTMLInputElement, { target: { value: '2026-10-10' } });

    const expectedTime = new Date("2026-10-10T09:00:00Z").toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    fireEvent.click(await screen.findByText(expectedTime));
    fireEvent.change(screen.getByPlaceholderText(/What do you need help with\?/i), { target: { value: 'Test description' } });
    fireEvent.click(screen.getByRole('button', { name: /Confirm Booking/i }));

    expect(await screen.findByText('The booking request could not be confirmed. Please try again.')).toBeDefined();
    expect(screen.queryByText('Booking request confirmed.')).toBeNull();
  });

  it('rejects malformed availability slots', async () => {
    (global.fetch as any).mockResolvedValueOnce({ ok: true, json: async () => ({ available_slots: [{ start_time: 'not-a-date', end_time: 'also-bad' }] }) });
    render(<BookingPage />);
    fireEvent.change(document.querySelector('input[type="date"]') as HTMLInputElement, { target: { value: '2026-10-10' } });
    expect(await screen.findByText(/Available times could not be loaded/)).toBeDefined();
    expect(screen.queryByRole('button', { name: /Invalid Date/i })).toBeNull();
  });

  it('requires a booking id and does not expose dummy Stripe checkout links', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ available_slots: [{ start_time: '2026-10-10T09:00:00Z', end_time: '2026-10-10T10:00:00Z' }] }),
    }).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ booking_id: 'booking-1', deposit_stripe_link: 'https://checkout.stripe.com/c/pay/cs_test_dummy' }),
    });
    render(<BookingPage />);
    fireEvent.change(screen.getByPlaceholderText('Jane Doe'), { target: { value: 'Test User' } });
    fireEvent.change(screen.getByPlaceholderText('jane@example.com'), { target: { value: 'test@example.com' } });
    fireEvent.change(document.querySelector('input[type="date"]') as HTMLInputElement, { target: { value: '2026-10-10' } });
    fireEvent.click(await screen.findByText(new Date('2026-10-10T09:00:00Z').toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })));
    fireEvent.change(screen.getByPlaceholderText(/What do you need help with\?/i), { target: { value: 'Test description' } });
    fireEvent.click(screen.getByRole('button', { name: /Confirm Booking/i }));

    expect(await screen.findByText('Booking request confirmed.')).toBeDefined();
    expect(screen.getByText(/Deposit checkout is unavailable/)).toBeDefined();
    expect(screen.queryByTestId('pay-deposit-btn')).toBeNull();
    expect(document.body.innerHTML).not.toContain('cs_test_dummy');
  });

  it('fails closed when the reservation response has no booking id', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ available_slots: [{ start_time: '2026-10-10T09:00:00Z', end_time: '2026-10-10T10:00:00Z' }] }),
    }).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ deposit_stripe_link: '/checkout/booking-1' }),
    });
    render(<BookingPage />);
    fireEvent.change(screen.getByPlaceholderText('Jane Doe'), { target: { value: 'Test User' } });
    fireEvent.change(screen.getByPlaceholderText('jane@example.com'), { target: { value: 'test@example.com' } });
    fireEvent.change(document.querySelector('input[type="date"]') as HTMLInputElement, { target: { value: '2026-10-10' } });
    fireEvent.click(await screen.findByText(new Date('2026-10-10T09:00:00Z').toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })));
    fireEvent.change(screen.getByPlaceholderText(/What do you need help with\?/i), { target: { value: 'Test description' } });
    fireEvent.click(screen.getByRole('button', { name: /Confirm Booking/i }));

    expect(await screen.findByText('The booking request could not be confirmed. Please try again.')).toBeDefined();
    expect(screen.queryByTestId('booking-checkout-container')).toBeNull();
  });
});
