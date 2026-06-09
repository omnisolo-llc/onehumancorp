import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { Omnibox } from './Omnibox';
import { useRouter } from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
}));

describe('Omnibox', () => {
  const mockPush = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    (useRouter as any).mockReturnValue({ push: mockPush });
    global.fetch = vi.fn();
  });

  it('opens on Cmd+K and closes on Escape', () => {
    render(<Omnibox />);
    expect(screen.queryByPlaceholderText(/Search customers/)).not.toBeInTheDocument();

    fireEvent.keyDown(window, { key: 'k', metaKey: true });
    expect(screen.getByPlaceholderText(/Search customers/)).toBeInTheDocument();

    fireEvent.keyDown(window, { key: 'Escape' });
    expect(screen.queryByPlaceholderText(/Search customers/)).not.toBeInTheDocument();
  });

  it('fetches data when typing', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        customers: [{ id: '1', name: 'John Doe', email: 'john@example.com' }],
        orders: [],
        messages: [],
      }),
    });

    render(<Omnibox />);
    fireEvent.keyDown(window, { key: 'k', metaKey: true });

    const input = screen.getByPlaceholderText(/Search customers/);
    fireEvent.change(input, { target: { value: 'John' } });

    await waitFor(() => {
      expect(screen.getByText('John Doe')).toBeInTheDocument();
    });

    // click item to test navigation
    fireEvent.click(screen.getByText('John Doe'));
    expect(mockPush).toHaveBeenCalledWith('/dashboard/customers/1');

    // check if it closed
    expect(screen.queryByPlaceholderText(/Search customers/)).not.toBeInTheDocument();
  });
});
