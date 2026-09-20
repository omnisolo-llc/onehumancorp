import { act, render, screen } from '@testing-library/react';
import SeasonalPromoPage from './page';
import { describe, it, expect, beforeEach, vi } from 'vitest';

vi.mock('next/navigation', () => {
    return {
        useRouter: () => ({ push: vi.fn() })
    };
});

vi.mock('../components/PoweredByOmniSolo', () => ({
  PoweredByOmniSolo: () => <div data-testid="powered-by-omnisolo" />
}));

describe('SeasonalPromoPage', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn<typeof fetch>(async () => Response.json({ current_plan: 'free' })));
    localStorage.clear();
    vi.spyOn(window, 'open').mockImplementation(() => null);
  });

  it('renders the page correctly', async () => {
    await act(async () => { render(<SeasonalPromoPage />); });
    expect(screen.getByText('Seasonal Promotion Generator ✨')).toBeInTheDocument();
  });

  it('renders the PoweredByOmniSolo component', async () => {
    await act(async () => { render(<SeasonalPromoPage />); });
    expect(screen.getByTestId('powered-by-omnisolo')).toBeInTheDocument();
  });
});
