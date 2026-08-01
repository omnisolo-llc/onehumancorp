import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { expect, test, describe, vi, beforeEach } from 'vitest';
import GlobalCommerceSettings from './page';

vi.mock('@/lib/utils/api', () => ({
  fetchJson: vi.fn().mockResolvedValue({ tenant: { base_currency: 'USD', enabled_currencies: ['USD', 'EUR'] } }),
}));

vi.mock('@/app/components/AppShell', () => ({
  AppShell: ({ children }: any) => <div data-testid="app-shell">{children}</div>,
  default: ({ children }: any) => <div data-testid="app-shell">{children}</div>,
}));

describe('GlobalCommerceSettings', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  test('renders loading state initially', () => {
    render(<GlobalCommerceSettings />);
    expect(screen.getByText('Loading settings...')).toBeDefined();
  });

  test('renders global commerce page correctly', async () => {
    render(<GlobalCommerceSettings />);
    await waitFor(() => {
      expect(screen.getByText('Global Commerce')).toBeDefined();
    });
  });
});
