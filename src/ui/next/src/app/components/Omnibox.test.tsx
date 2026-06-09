import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { Omnibox } from './Omnibox';
import { vi } from 'vitest';

// Mock useRouter
const pushMock = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: pushMock }),
}));

describe('Omnibox component', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    global.fetch = vi.fn();
  });

  it('renders nothing when closed', () => {
    render(<Omnibox />);
    expect(screen.queryByTestId('omnibox-overlay')).toBeNull();
  });

  it('opens on Cmd+K and focuses input', async () => {
    render(<Omnibox />);
    fireEvent.keyDown(window, { key: 'k', metaKey: true });

    await waitFor(() => {
      expect(screen.getByTestId('omnibox-overlay')).toBeInTheDocument();
    });

    const input = screen.getByTestId('omnibox-input');
    expect(input).toBeInTheDocument();
  });

  it('fetches and displays search results correctly', async () => {
    const mockResults = {
      results: [
        {
          entity_type: 'customer',
          id: 'cust-123',
          title: 'John Doe',
          subtitle: 'john@example.com',
          url_path: '/customers/cust-123'
        }
      ]
    };

    (global.fetch as vi.Mock).mockResolvedValue({
      ok: true,
      json: async () => mockResults
    });

    render(<Omnibox />);
    // Open omnibox
    fireEvent.keyDown(window, { key: 'k', metaKey: true });

    const input = await screen.findByTestId('omnibox-input');
    fireEvent.change(input, { target: { value: 'John' } });

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/search?q=John');
    });

    await waitFor(() => {
      expect(screen.getByText('John Doe')).toBeInTheDocument();
      expect(screen.getByText('john@example.com')).toBeInTheDocument();
    });
  });

  it('navigates to url_path on click', async () => {
    const mockResults = {
      results: [
        {
          entity_type: 'order',
          id: 'ord-456',
          title: 'ord-456',
          subtitle: 'completed',
          url_path: '/orders/ord-456'
        }
      ]
    };

    (global.fetch as vi.Mock).mockResolvedValue({
      ok: true,
      json: async () => mockResults
    });

    render(<Omnibox />);
    fireEvent.keyDown(window, { key: 'k', metaKey: true });

    const input = await screen.findByTestId('omnibox-input');
    fireEvent.change(input, { target: { value: 'ord' } });

    const resultBtn = await screen.findByTestId('omnibox-result-0');
    fireEvent.click(resultBtn);

    expect(pushMock).toHaveBeenCalledWith('/orders/ord-456');
    // Ensure it closes
    expect(screen.queryByTestId('omnibox-overlay')).toBeNull();
  });
});
