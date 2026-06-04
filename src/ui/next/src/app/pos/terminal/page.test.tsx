import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import TerminalPage from './page';
import { loadStripeTerminal } from '@stripe/terminal-js';

vi.mock('@stripe/terminal-js', () => ({
  loadStripeTerminal: vi.fn(),
}));

vi.mock('../../../lib/localizationStore', () => ({
  useTranslation: () => ({ t: (str: string) => str }),
  useCurrency: () => ({
    currency: 'USD',
    convert: (amount: number, from: string, to: string) => ({ amount, isOffline: false })
  }),
  useLocalizationStore: () => ({
    locale: 'en',
    currency: 'USD',
    setLocale: vi.fn(),
    setCurrency: vi.fn(),
  })
}));

describe('TerminalPage Tap to Pay Integration', () => {
  beforeEach(() => {
    localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: '1', name: 'Test Staff', role: 'Staff', pin_hash: '1234' }]));
  });
  afterEach(() => {
    localStorage.clear();
    vi.clearAllMocks();
  });

  it('renders and allows tap to pay', async () => {
    // Mock the Stripe terminal instance
    const mockTerminal = {
      discoverReaders: vi.fn().mockResolvedValue({ discoveredReaders: [{ id: 'reader_123' }], error: null }),
      connectReader: vi.fn().mockResolvedValue({ reader: { id: 'reader_123' }, error: null }),
      collectPaymentMethod: vi.fn().mockResolvedValue({ paymentIntent: { id: 'pi_123' }, error: null }),
      processPayment: vi.fn().mockResolvedValue({ paymentIntent: { id: 'pi_123', status: 'succeeded' }, error: null }),
    };

    (loadStripeTerminal as any).mockResolvedValue({
      create: vi.fn().mockImplementation((config) => {
        if (config && config.onFetchConnectionToken) {
           config.onFetchConnectionToken();
        }
        return mockTerminal;
      }),
    });

    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url === '/api/v1/pos/terminal/token') {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ token: 'mock_token' }) });
      }
      if (url === '/api/v1/pos/terminal/intent') {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ intent_id: 'pi_123' }) });
      }
      if (url === '/api/pos/orders') {
        return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
    }) as any;

    const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});

    await act(async () => {
      render(<TerminalPage />);
    });

    // Login
    await act(async () => {
      fireEvent.click(screen.getByText('1'));
    });
    await act(async () => {
      fireEvent.click(screen.getByText('2'));
    });
    await act(async () => {
      fireEvent.click(screen.getByText('3'));
    });
    await act(async () => {
      fireEvent.click(screen.getByText('4'));
    });

    expect(screen.getByText('Test Staff')).toBeDefined();

    // Wait for terminal instance to be loaded
    await waitFor(() => {
      // The button might still be clickable
    });

    // Click New Order
    const newOrderButton = screen.getByText('New Order');
    await act(async () => {
      fireEvent.click(newOrderButton);
    });

    await waitFor(() => {
      expect(screen.getByText('Payment successful!')).toBeDefined();
    });

    await waitFor(() => {
      expect(mockTerminal.collectPaymentMethod).toHaveBeenCalledWith('pi_123');
      expect(mockTerminal.processPayment).toHaveBeenCalledWith('pi_123');
    });
  });
});
