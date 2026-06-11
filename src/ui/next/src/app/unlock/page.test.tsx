import { render, screen, fireEvent, waitFor } from '@testing-library/react';
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

  it('unlocks code when Share on X is clicked', async () => {
    const windowOpenSpy = vi.spyOn(window, 'open').mockImplementation(() => null);

    render(<UnlockPage />);

    const shareButton = screen.getByText('Share on X');
    fireEvent.click(shareButton);

    expect(windowOpenSpy).toHaveBeenCalled();

    // Wait for the simulated verification (1.5s)
    await waitFor(() => {
        expect(screen.getByText('Congratulations! Here is your reward:')).toBeDefined();
        expect(screen.getByText('Copy Code')).toBeDefined();
    }, { timeout: 2000 });

    windowOpenSpy.mockRestore();
  });
});
