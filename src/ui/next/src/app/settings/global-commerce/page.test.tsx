import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import GlobalCommerceSettings from './page';
import { vi } from 'vitest';
import { fetchJson, putJson } from '@/lib/utils/api';

vi.mock('@/lib/utils/api', () => ({
  fetchJson: vi.fn().mockResolvedValue({ tenant: { base_currency: 'USD', enabled_currencies: ['USD', 'EUR'] } }),
  putJson: vi.fn().mockResolvedValue({}),
}));

vi.mock('@/app/components/AppShell', () => ({
  AppShell: ({ children }: any) => <div data-testid="app-shell">{children}</div>,
}));

describe('GlobalCommerceSettings', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.stubGlobal('alert', vi.fn());
  });

  it('renders loading state initially', () => {
    render(<GlobalCommerceSettings />);
    expect(screen.getByText('Loading settings...')).toBeInTheDocument();
  });

  it('loads tenant settings from the authenticated API', async () => {
    render(<GlobalCommerceSettings />);

    await waitFor(() => expect(fetchJson).toHaveBeenCalledWith('/api/v1/settings'));
    expect(screen.getByDisplayValue('USD')).toBeInTheDocument();
  });

  it('saves the selected commerce settings through the authenticated API', async () => {
    render(<GlobalCommerceSettings />);
    await screen.findByRole('button', { name: 'Save Changes' });

    fireEvent.change(screen.getByDisplayValue('USD'), { target: { value: 'EUR' } });
    fireEvent.click(screen.getByRole('button', { name: 'Save Changes' }));

    await waitFor(() => {
      expect(putJson).toHaveBeenCalledWith('/api/v1/settings', {
        base_currency: 'EUR',
        enabled_currencies: ['USD', 'EUR'],
      });
    });
  });
});
