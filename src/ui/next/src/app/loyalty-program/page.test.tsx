import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import LoyaltyProgramPage from './page';
import * as React from 'react';

const mockPush = vi.fn();

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

describe('LoyaltyProgramPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn((key) => {
          if (key === 'has_pro') return 'true';
          if (key === 'tenant') return 'test-store';
          return null;
        }),
      },
      writable: true
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly and has all required fields', () => {
    render(<LoyaltyProgramPage />);
    expect(screen.getByText('Customer Loyalty Program 🤝')).toBeDefined();
    expect(screen.getByText('Generate Email')).toBeDefined();
  });

  it('calls generate endpoint and displays result', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        message: 'Test loyalty campaign message generated successfully.',
      }),
    } as any);

    render(<LoyaltyProgramPage />);

    const inputs = screen.getAllByPlaceholderText('e.g. 10');
    fireEvent.change(inputs[0], { target: { value: '20' } });
    fireEvent.change(inputs[1], { target: { value: '20' } });

    const generateButton = screen.getByText('Generate Email');
    fireEvent.click(generateButton);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/loyalty/generate', expect.objectContaining({
        method: 'POST',
        headers: expect.objectContaining({
            'Content-Type': 'application/json'
        }),
        body: expect.stringContaining('test-store')
      }));
      expect(screen.getByDisplayValue('Test loyalty campaign message generated successfully.')).toBeDefined();
    });
  });
});
