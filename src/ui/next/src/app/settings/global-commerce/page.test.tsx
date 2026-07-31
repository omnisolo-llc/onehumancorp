import { render, screen } from '@testing-library/react';
import GlobalCommerceSettings from './page';
import { vi } from 'vitest';

vi.mock('@/lib/utils/api', () => ({
  fetchJson: vi.fn().mockResolvedValue({ tenant: { base_currency: 'USD', enabled_currencies: ['USD', 'EUR'] } }),
  putJson: vi.fn().mockResolvedValue({}),
}));

vi.mock('@/app/components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div data-testid="appshell-mock">{children}</div>,
}));

describe('GlobalCommerceSettings', () => {
  it('renders loading state initially', () => {
    render(<GlobalCommerceSettings />);
    expect(screen.getByText('Loading settings...')).toBeInTheDocument();
  });
});
