import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { expect, test, vi, describe, beforeEach } from 'vitest';
import AutoDreamPage from './page';

// Mock useRouter
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('AutoDreamPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn() as any;
    window.HTMLElement.prototype.scrollIntoView = vi.fn();
  });

  test('renders initial agent greeting', () => {
    render(<AutoDreamPage />);
    expect(screen.getByText(/Hi! I'm your Operations Manager/i)).toBeInTheDocument();
  });

  test('allows user to submit a prompt', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        tenant_id: 'test-org-123',
        business_name: 'Test Bakery',
        business_type: 'Home Bakery',
        products: [{ name: 'Cake', price: '20' }]
      })
    });

    render(<AutoDreamPage />);
    const input = screen.getByPlaceholderText(/Describe your business/i);
    const button = screen.getByRole('button');

    fireEvent.change(input, { target: { value: 'I bake cakes' } });
    fireEvent.click(button);

    // User message should appear
    expect(screen.getByText('I bake cakes')).toBeInTheDocument();

    // Verify fetch was called after simulated wait
    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/autodream/scaffold', expect.any(Object));
    }, { timeout: 6000 });

    // Verify agent response
    await waitFor(() => {
      expect(screen.getAllByText(/Test Bakery/i).length).toBeGreaterThan(0);
      expect(screen.getByText(/Draft Ready for Review/i)).toBeInTheDocument();
    }, { timeout: 3000 });
  }, 10000); // increase test timeout
});
