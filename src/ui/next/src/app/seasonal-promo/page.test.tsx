import { render, screen } from '@testing-library/react';
import SeasonalPromoPage from './page';
import { describe, it, expect, beforeEach, vi } from 'vitest';
import '@testing-library/jest-dom/vitest';
import { act } from 'react';

vi.mock('next/navigation', () => {
    return {
        useRouter: () => ({ push: vi.fn() })
    };
});

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />
}));

describe('SeasonalPromoPage', () => {
  beforeEach(() => {
    Object.defineProperty(window, 'location', {
      value: { origin: 'http://localhost:3000' },
      writable: true
    });
    vi.spyOn(window, 'open').mockImplementation(() => null);
  });

  it('renders the page correctly', () => {
    act(() => { render(<SeasonalPromoPage />); });
    expect(screen.getByText('Seasonal Promotion Generator ✨')).toBeInTheDocument();
  });

  it('renders the PoweredByOHC component', () => {
    act(() => { render(<SeasonalPromoPage />); });
    expect(screen.getByTestId('powered-by-ohc')).toBeInTheDocument();
  });
});
