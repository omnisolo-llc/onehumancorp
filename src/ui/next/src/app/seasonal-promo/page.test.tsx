import { render, screen } from '@testing-library/react';
import SeasonalPromoPage from './page';
import { describe, it, expect, beforeEach, vi } from 'vitest';

vi.mock('next/navigation', () => {
    return {
        useRouter: () => ({ push: vi.fn() })
    };
});

describe('SeasonalPromoPage', () => {
  beforeEach(() => {
    Object.defineProperty(window, 'location', {
      value: { origin: 'http://localhost:3000' },
      writable: true
    });
    vi.spyOn(window, 'open').mockImplementation(() => null);
  });

  it('renders the page correctly', () => {
    render(<SeasonalPromoPage />);
    expect(screen.getByText('Seasonal Promotion Generator ✨')).toBeInTheDocument();
  });
});
