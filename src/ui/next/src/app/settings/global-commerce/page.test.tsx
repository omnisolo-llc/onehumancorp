import { render, screen } from '@testing-library/react';
import GlobalCommerceSettings from './page';
import { vi } from 'vitest';
import { TooltipProvider } from '@/components/TooltipRegistry';

vi.mock('@/lib/utils/api', () => ({
  fetchJson: vi.fn().mockResolvedValue({ tenant: { base_currency: 'USD', enabled_currencies: ['USD', 'EUR'] } }),
  putJson: vi.fn().mockResolvedValue({}),
}));

vi.mock('@/components/AppShell', () => ({
  AppShell: ({ children }: any) => <div data-testid="app-shell">{children}</div>,
}));

describe('GlobalCommerceSettings', () => {
  it('renders loading state initially', () => {
    render(<TooltipProvider><GlobalCommerceSettings /></TooltipProvider>);
    expect(screen.getByText('Loading settings...')).toBeInTheDocument();
  });
});
