import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import AffiliateProgramPage from './page';

// Mock fetch
global.fetch = vi.fn();

describe('AffiliateProgramPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    // Setup localStorage mock
    const store: Record<string, string> = { tenant_id: 'test-tenant' };
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn((key) => store[key] || null),
        setItem: vi.fn((key, value) => { store[key] = value.toString(); }),
      },
      writable: true,
    });

    // Setup clipboard mock
    Object.defineProperty(navigator, 'clipboard', {
      value: {
        writeText: vi.fn(() => Promise.resolve()),
      },
      writable: true,
    });
  });

  it('renders loading state initially', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ affiliates: [] }),
    });

    render(<AffiliateProgramPage />);
    expect(screen.getByText('Loading affiliates...')).toBeDefined();
  });

  it('renders truthful empty state when no affiliates exist', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ affiliates: [] }),
    });

    render(<AffiliateProgramPage />);

    await waitFor(() => {
      expect(screen.queryByText('Loading affiliates...')).toBeNull();
    });

    expect(screen.getByText('No active affiliates yet')).toBeDefined();
  });

  it('renders affiliates when they exist', async () => {
    const mockAffiliates = [
      { id: '1', affiliate_code: 'TEST20', customer_id: 'cust_123', commission_percentage: 20 },
      { id: '2', affiliate_code: 'SAVE10', customer_id: 'cust_456', commission_percentage: 10 },
    ];

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ affiliates: mockAffiliates }),
    });

    render(<AffiliateProgramPage />);

    await waitFor(() => {
      expect(screen.queryByText('Loading affiliates...')).toBeNull();
    });

    expect(screen.getByText('TEST20')).toBeDefined();
    expect(screen.getByText('SAVE10')).toBeDefined();
    expect(screen.getByText('20%')).toBeDefined();
  });

  it('handles copy link functionality', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ affiliates: [] }),
    });

    render(<AffiliateProgramPage />);

    const copyButton = screen.getByText('Copy Link');
    await userEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByText('Copied!')).toBeDefined();
  });
});