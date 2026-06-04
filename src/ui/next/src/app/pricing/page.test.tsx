import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import PricingPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('PricingPage', () => {
  const renderWithProviders = (ui: React.ReactElement) => render(<TooltipProvider>{ui}</TooltipProvider>);

  it('renders the pricing headers', () => {
    renderWithProviders(<PricingPage />);
    expect(screen.getByRole('heading', { name: /Pricing Plans/i })).toBeDefined();
    expect(screen.getByText(/Plain-language pricing — no hidden fees/i)).toBeDefined();
  });

  it('renders the free tier limits', () => {
    renderWithProviders(<PricingPage />);
    expect(screen.getByRole('heading', { name: /Free/i })).toBeDefined();
    expect(screen.getByText('1 Agent Limit')).toBeDefined();
    expect(screen.getByText('500MB Storage Quota')).toBeDefined();
  });

  it('renders the starter tier limits', () => {
    renderWithProviders(<PricingPage />);
    expect(screen.getByRole('heading', { name: /Starter/i })).toBeDefined();
    expect(screen.getByText('3 Agents Limit')).toBeDefined();
    expect(screen.getByText('5GB Storage Quota')).toBeDefined();
  });
});
// comment to trigger push
// force push re-trigger 2
// timestamp: 1780554444
