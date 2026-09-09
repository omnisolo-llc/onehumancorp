import { render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import Wrapped from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn() }),
}));

describe('Wrapped', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ total_sales: 123, pending_orders: 4, top_product: 'Custom Cake' }),
    }));
    localStorage.clear();
  });

  it('does not depend on an external Google Fonts stylesheet', async () => {
    render(<Wrapped />);

    await waitFor(() => expect(screen.getByText('Your OmniSolo Snapshot')).toBeTruthy());

    const styleText = [...document.querySelectorAll('style')]
      .map((style) => style.textContent || '')
      .join('\n');

    expect(styleText).not.toMatch(/fonts\.(googleapis|gstatic)\.com/i);
  });
});
