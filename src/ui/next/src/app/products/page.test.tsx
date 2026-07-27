import { render, screen, fireEvent } from '@testing-library/react';
import { beforeEach, describe, it, expect, vi } from 'vitest';
import '@testing-library/jest-dom/vitest';
import { act } from 'react';
import ProductsPage from './page';

// Mock AppShell to avoid complex rendering issues
vi.mock('../components/AppShell', () => ({
  AppShell: ({ children, actions }: any) => (
    <div data-testid="app-shell">
      {actions.map((action: any) => (
        <a key={action.label} href={action.href}>{action.label}</a>
      ))}
      {children}
    </div>
  ),
}));

describe('ProductsPage', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      json: async () => [{
        id: 'product-1',
        title: 'Seeded Tea',
        price_cents: 1250,
        image_url: '/seeded-tea.png',
      }],
    }));
  });

  it('renders correctly', () => {
    act(() => { render(<ProductsPage />); });
    expect(screen.getByText('Imported Products')).toBeDefined();

  });

  it('does not invent product status and generates checkout QR data from the real product id', async () => {
    act(() => { render(<ProductsPage />); });

    expect(await screen.findByText('Seeded Tea')).toBeDefined();
    expect(screen.queryByText('Active')).toBeNull();

    fireEvent.click(screen.getByRole('button', { name: 'Generate QR Code' }));
    const qr = screen.getByAltText('QR Code for Seeded Tea') as HTMLImageElement;
    const qrRequest = new URL(qr.src);
    expect(qrRequest.hostname).toBe('api.qrserver.com');
    expect(qrRequest.searchParams.get('data')).toBe('https://ohc.app/checkout?product_id=product-1');
  });

});
