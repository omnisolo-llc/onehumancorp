import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { TapToPayOverlay } from './TapToPayOverlay';

describe('TapToPayOverlay Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn();
  });

  it('renders correctly when open', () => {
    render(<TapToPayOverlay isOpen={true} onClose={() => {}} amount={1500} currency="usd" onSuccess={() => {}} />);
    expect(screen.getByTestId('tap-to-pay-overlay')).toBeInTheDocument();
    expect(screen.getByText('Total: $15.00')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /accept payment/i })).toBeInTheDocument();
  });

  it('does not render when closed', () => {
    const { container } = render(<TapToPayOverlay isOpen={false} onClose={() => {}} amount={1500} currency="usd" onSuccess={() => {}} />);
    expect(container).toBeEmptyDOMElement();
  });

  it('handles the full payment flow successfully', async () => {
    const onSuccess = vi.fn();

    // Mock the 3 fetch calls
    (global.fetch as any)
      .mockResolvedValueOnce({ ok: true, json: async () => ({ secret: 'test_token' }) }) // connection-token
      .mockResolvedValueOnce({ ok: true, json: async () => ({ id: 'pi_test123' }) })     // payment-intent
      .mockResolvedValueOnce({ ok: true, json: async () => ({ status: 'succeeded' }) }); // capture

    render(<TapToPayOverlay isOpen={true} onClose={() => {}} amount={2050} currency="usd" onSuccess={onSuccess} />);

    const acceptBtn = screen.getByRole('button', { name: /accept payment/i });
    fireEvent.click(acceptBtn);

    // Initializing state
    await waitFor(() => {
      expect(screen.getByText('Hold card or phone near reader')).toBeInTheDocument();
      expect(screen.getByText('$20.50')).toBeInTheDocument();
    });

    // We simulated a 2s delay in the component. Let's wait for success state.
    await waitFor(() => {
      expect(screen.getByText('Payment Successful!')).toBeInTheDocument();
    }, { timeout: 3000 });

    // Verify it called onSuccess after delay
    await waitFor(() => {
      expect(onSuccess).toHaveBeenCalledWith('pi_test123');
    }, { timeout: 2000 });

    expect(global.fetch).toHaveBeenCalledTimes(3);
  });

  it('handles API errors gracefully', async () => {
    // Mock the connection-token call to fail
    (global.fetch as any).mockResolvedValueOnce({ ok: false });

    render(<TapToPayOverlay isOpen={true} onClose={() => {}} amount={500} currency="usd" onSuccess={() => {}} />);

    fireEvent.click(screen.getByRole('button', { name: /accept payment/i }));

    await waitFor(() => {
      expect(screen.getByText('Payment Failed')).toBeInTheDocument();
      expect(screen.getByText('Failed to initialize terminal.')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: /try again/i })).toBeInTheDocument();
    });
  });
});
