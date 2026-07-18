import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import UnlockPage from './page';

const mockUseSearchParams = vi.fn();
vi.mock('next/navigation', () => ({
  useSearchParams: () => mockUseSearchParams(),
}));

describe('UnlockPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseSearchParams.mockReturnValue(new URLSearchParams('?title=My%20Awesome%20Promo&reward=50%25%20Off&code=HALFPRICE'));
  });

  it('renders correctly', () => {
    render(<UnlockPage />);
    expect(screen.getByText('My Awesome Promo')).toBeDefined();
    expect(screen.getByText('50% Off')).toBeDefined();
    expect(screen.getByText('HALFPRICE')).toBeDefined();
  });

  it('keeps the code locked when verification is unavailable', () => {
    const windowOpenSpy = vi.spyOn(window, 'open').mockImplementation(() => null);

    render(<UnlockPage />);

    const shareButton = screen.getByText('Share on X');
    fireEvent.click(shareButton);

    expect(windowOpenSpy).toHaveBeenCalled();

    expect(screen.getByText('Reward verification is unavailable. The code remains locked.')).toBeDefined();
    expect(screen.getByText('Locked')).toBeDefined();
    expect(screen.queryByText('Copy Code')).toBeNull();

    windowOpenSpy.mockRestore();
  });
});
