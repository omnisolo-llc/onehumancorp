import '@testing-library/jest-dom';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import React from 'react';
import AbandonedCartPage from './page';
import { useRouter } from 'next/navigation';
import { expect, test, vi, describe, beforeEach, afterEach } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
}));

describe('AbandonedCartPage', () => {
  beforeEach(() => {
    (useRouter as any).mockReturnValue({
      push: vi.fn(),
    });
    vi.clearAllMocks();
    global.fetch = vi.fn() as any;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  test('renders form correctly', () => {
    render(<AbandonedCartPage />);
    expect(screen.getByText('Abandoned Cart Campaign 🛒')).toBeDefined();
    expect(screen.getByLabelText('Customer Name')).toBeDefined();
    expect(screen.getByLabelText('Cart Value')).toBeDefined();
    expect(screen.getByText('Generate AI Campaign')).toBeDefined();
  });

  test('button is disabled when fields are empty', () => {
    render(<AbandonedCartPage />);
    const button = screen.getByText('Generate AI Campaign') as HTMLButtonElement;
    expect(button.disabled).toBe(true);
  });

  test('calls API and displays result', async () => {
    const mockMessage = 'Hi Alice, you left some items in your cart. Powered by OHC';
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: vi.fn().mockResolvedValue({ message: mockMessage }),
    });

    render(<AbandonedCartPage />);

    fireEvent.change(screen.getByLabelText('Customer Name'), { target: { value: 'Alice' } });
    fireEvent.change(screen.getByLabelText('Cart Value'), { target: { value: '$50.00' } });

    const button = screen.getByText('Generate AI Campaign') as HTMLButtonElement;
    expect(button.disabled).toBe(false);

    fireEvent.click(button);

    expect(screen.getByText('Generating...')).toBeDefined();

    await waitFor(() => {
      expect(screen.getByText(mockMessage)).toBeDefined();
    });

    expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/campaign/generate-cart', expect.objectContaining({
      method: 'POST',
      body: JSON.stringify({ customer_name: 'Alice', cart_value: '$50.00' }),
    }));
  });

  test('displays error message on failed request', async () => {
    (global.fetch as any).mockResolvedValue({
      ok: false,
    });

    render(<AbandonedCartPage />);

    fireEvent.change(screen.getByLabelText('Customer Name'), { target: { value: 'Alice' } });
    fireEvent.change(screen.getByLabelText('Cart Value'), { target: { value: '$50.00' } });

    fireEvent.click(screen.getByText('Generate AI Campaign'));

    await waitFor(() => {
      expect(screen.getByText('Error generating campaign.')).toBeDefined();
    });
  });
});
