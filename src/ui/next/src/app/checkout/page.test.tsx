import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { expect, test, vi, beforeEach } from 'vitest';
import CheckoutPage from './page';

// Mock useRouter
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

// Mock the UpsellDrawer component to avoid fetching during page tests,
// or we can test it integrated.
// We will test it integrated but mock fetch.
global.fetch = vi.fn() as unknown as typeof fetch;

beforeEach(() => {
  vi.resetAllMocks();
});

test('renders checkout page with upsell drawer', async () => {
  // Mock the upsell API response
  (global.fetch as any).mockImplementation(async (url: string) => {
    if (url.includes('/api/tooltips')) {
      return {
        ok: true,
        json: async () => ({ tooltips: {} })
      };
    }
    return {
      ok: true,
      json: async () => ({
        success: true,
        recommendations: [
          {
            id: 'upsell_matches',
            name: 'Premium Matches',
            price: 5.00,
            original_price: 8.00,
            description: 'Perfectly pairs with your candle.',
            image_url: 'http://example.com/matches.jpg'
          }
        ]
      })
    };
  });

  render(
    <TooltipProvider>
      <CheckoutPage />
    </TooltipProvider>
  );

  // Should show the title
  expect(screen.getByText('Checkout')).toBeInTheDocument();

  // Wait for the UpsellDrawer to render items
  await waitFor(() => {
    expect(screen.getByText('Frequently Bought Together')).toBeInTheDocument();
  });

  // Verify the upsell item is displayed
  expect(screen.getByText('Premium Matches')).toBeInTheDocument();
  expect(screen.getByText('$5.00')).toBeInTheDocument();

  // Click Add
  const addButton = screen.getByText('Add');
  fireEvent.click(addButton);

  // Since we added an item, subtotal should theoretically update.
  // In our simplified mock state, we don't display subtotal explicitly yet,
  // but we can verify the function was called or state was updated if we had a subtotal display.
});
