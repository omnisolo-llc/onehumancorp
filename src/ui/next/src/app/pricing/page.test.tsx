import { render, screen, fireEvent } from '@testing-library/react';
import PricingPage from './page';
import { expect, test, vi, beforeEach, afterEach } from 'vitest';
import { TooltipProvider } from '../../components/TooltipRegistry';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

beforeEach(() => {
  vi.clearAllMocks();
});

afterEach(() => {
  vi.restoreAllMocks();
});

test('renders Pricing Plans heading', () => {
    render(<TooltipProvider><PricingPage /></TooltipProvider>);
    expect(screen.getByText('Pricing Plans')).toBeDefined();
});

test('navigates to checkout when clicking upgrade on Pro', () => {
    render(<TooltipProvider><PricingPage /></TooltipProvider>);
    const proButton = screen.getByText('Upgrade to Pro via Stripe');
    fireEvent.click(proButton);
    expect(mockPush).toHaveBeenCalledWith('/checkout?tier=Pro');
});

test('shows ROI Calculator section', () => {
    render(<TooltipProvider><PricingPage /></TooltipProvider>);
    expect(screen.getByText('Calculate Your Pro Plan ROI')).toBeDefined();
});

test('updates ROI calculation when slider changes', () => {
    render(<TooltipProvider><PricingPage /></TooltipProvider>);

    // Initial values
    // Monthly Orders: 50, AOV: 40 -> Current Revenue = $2000
    // Pro Plan uplift: Conversions (+25%), AOV (+15%)
    // Projected Orders: Math.round(50 * 1.25) = 63
    // Projected AOV: 40 * 1.15 = 46
    // Projected Revenue: 63 * 46 = 2898
    // Growth: 2898 - 2000 = $898

    expect(screen.getByText('+$898')).toBeDefined();

    // Find and change the Monthly Orders slider
    const sliders = screen.getAllByRole('slider');
    const ordersSlider = sliders[0];

    fireEvent.change(ordersSlider, { target: { value: '100' } });

    // Monthly Orders: 100, AOV: 40 -> Current Revenue = $4000
    // Projected Orders: Math.round(100 * 1.25) = 125
    // Projected AOV: 40 * 1.15 = 46
    // Projected Revenue: 125 * 46 = 5750
    // Growth: 5750 - 4000 = $1750
    expect(screen.getByText('+$1,750')).toBeDefined();
});

test('allows upgrading to Pro directly from ROI Calculator', () => {
    render(<TooltipProvider><PricingPage /></TooltipProvider>);
    const proNowButton = screen.getByText('Upgrade to Pro Now');
    fireEvent.click(proNowButton);
    expect(mockPush).toHaveBeenCalledWith('/checkout?tier=Pro');
});
