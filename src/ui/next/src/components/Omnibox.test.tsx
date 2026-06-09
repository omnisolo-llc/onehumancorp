import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Omnibox } from './Omnibox';
import { vi } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('Omnibox', () => {
  beforeEach(() => {
    // Mock fetch
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        results: [
          {
            id: '1',
            entity_type: 'customer',
            title: 'John Doe',
            url: '/customers/1'
          }
        ]
      })
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders trigger button', () => {
    render(<Omnibox />);
    expect(screen.getByTestId('omnibox-trigger')).toBeInTheDocument();
  });

  it('opens on button click', async () => {
    render(<Omnibox />);
    fireEvent.click(screen.getByTestId('omnibox-trigger'));
    expect(screen.getByTestId('omnibox-input')).toBeInTheDocument();
  });

  it('opens on shortcut (Cmd+K)', async () => {
    render(<Omnibox />);
    fireEvent.keyDown(window, { key: 'k', metaKey: true });
    expect(screen.getByTestId('omnibox-input')).toBeInTheDocument();
  });

  it('closes on Escape', async () => {
    render(<Omnibox />);
    fireEvent.keyDown(window, { key: 'k', metaKey: true });
    expect(screen.getByTestId('omnibox-input')).toBeInTheDocument();

    fireEvent.keyDown(window, { key: 'Escape' });
    expect(screen.queryByTestId('omnibox-input')).not.toBeInTheDocument();
  });

  it('fetches and displays results on input', async () => {
    render(<Omnibox />);
    fireEvent.click(screen.getByTestId('omnibox-trigger'));

    const input = screen.getByTestId('omnibox-input');
    await userEvent.type(input, 'John');

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/search?q=John');
    });

    await waitFor(() => {
      expect(screen.getByText('John Doe')).toBeInTheDocument();
    });
  });
});
