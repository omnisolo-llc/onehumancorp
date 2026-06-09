import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { Omnibox } from './Omnibox';
import { expect, test, vi, describe, beforeEach } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('Omnibox', () => {
  beforeEach(() => {
    global.fetch = vi.fn();
  });

  test('should not be visible by default', () => {
    render(<Omnibox />);
    expect(screen.queryByPlaceholderText('Search customers, orders, or messages...')).toBeNull();
  });

  test('should open when Cmd+K or Ctrl+K is pressed', async () => {
    render(<Omnibox />);

    fireEvent.keyDown(window, { key: 'k', metaKey: true });

    await waitFor(() => {
      expect(screen.getByPlaceholderText('Search customers, orders, or messages...')).not.toBeNull();
    });
  });

  test('should close when Escape is pressed', async () => {
    render(<Omnibox />);

    fireEvent.keyDown(window, { key: 'k', metaKey: true });
    await waitFor(() => {
      expect(screen.getByPlaceholderText('Search customers, orders, or messages...')).not.toBeNull();
    });

    fireEvent.keyDown(window, { key: 'Escape' });
    await waitFor(() => {
      expect(screen.queryByPlaceholderText('Search customers, orders, or messages...')).toBeNull();
    });
  });
});
