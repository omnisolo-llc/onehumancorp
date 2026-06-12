import { render, screen } from '@testing-library/react';
import InvoiceViewPage from './page';
import { useSearchParams } from 'next/navigation';
import { vi } from 'vitest';

vi.mock('next/navigation', () => ({
  useSearchParams: vi.fn(),
}));

describe('InvoiceViewPage', () => {
  it('renders without branding when removeBranding is true', () => {
    const mockData = {
      tenant: 'test-tenant',
      clientName: 'Test Client',
      projectDetails: 'Test Project',
      amount: '100',
      removeBranding: true,
    };
    const base64Str = btoa(unescape(encodeURIComponent(JSON.stringify(mockData))));
    const base64UrlStr = base64Str.replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');

    (useSearchParams as any).mockReturnValue(new URLSearchParams(`?data=${base64UrlStr}`));

    render(<InvoiceViewPage />);
    expect(screen.queryByText(/Powered by OHC/i)).toBeNull();
  });

  it('renders with branding when removeBranding is false', () => {
    const mockData = {
      tenant: 'test-tenant',
      clientName: 'Test Client',
      projectDetails: 'Test Project',
      amount: '100',
      removeBranding: false,
    };
    const base64Str = btoa(unescape(encodeURIComponent(JSON.stringify(mockData))));
    const base64UrlStr = base64Str.replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');

    (useSearchParams as any).mockReturnValue(new URLSearchParams(`?data=${base64UrlStr}`));

    render(<InvoiceViewPage />);
    expect(screen.getByText(/Powered by OHC/i)).toBeDefined();
  });
});
