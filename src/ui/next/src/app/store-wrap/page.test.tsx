import { render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import StoreWrapPage from './page';

vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));

describe('StoreWrapPage', () => {
  beforeEach(() => {
    localStorage.clear();
    localStorage.setItem('business_display_name', 'forged-tenant');
  });

  it('loads metrics through the claim-bound dashboard endpoint without browser identity', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ total_sales: 125, active_customers: 4 }),
    });
    global.fetch = fetchMock;

    render(<StoreWrapPage />);

    await waitFor(() => expect(fetchMock).toHaveBeenCalledWith('/api/v1/ui/dashboard/metrics'));
    expect(fetchMock).not.toHaveBeenCalledWith(expect.anything(), expect.objectContaining({ method: 'POST' }));
    expect(await screen.findByText('Recorded Store Snapshot')).toBeInTheDocument();
    expect(document.body.textContent).not.toContain('forged-tenant');
  });
});
