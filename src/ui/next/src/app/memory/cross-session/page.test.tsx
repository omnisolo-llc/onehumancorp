import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import CrossSessionRecall from './page';

describe('CrossSessionRecall', () => {
  beforeEach(() => {
    global.fetch = vi.fn();
  });

  it('renders correctly', () => {
    render(<CrossSessionRecall />);
    expect(screen.getByText('Cross-Session Recall')).toBeDefined();
  });

  it('handles search correctly', async () => {
    vi.mocked(global.fetch).mockResolvedValue(new Response(JSON.stringify({
      results: ['Result 1', 'Result 2']
    }), { status: 200 }));

    render(<CrossSessionRecall />);
    const input = screen.getByPlaceholderText('Search past conversations');
    const button = screen.getByText('Search Memory');

    fireEvent.change(input, { target: { value: 'query' } });
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByText('Result 1')).toBeDefined();
      expect(screen.getByText('Result 2')).toBeDefined();
    });
  });
});
