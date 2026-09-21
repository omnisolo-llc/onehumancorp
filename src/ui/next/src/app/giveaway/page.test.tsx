import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import GiveawayPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

describe('GiveawayPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly', () => {
    act(() => { render(<GiveawayPage />); });
    expect(screen.getByText('Viral Giveaway Generator 🎁')).toBeDefined();
    expect(screen.getByText('Giveaway Details')).toBeDefined();
  });

  it('updates preview when inputs change', () => {
    act(() => { render(<GiveawayPage />); });

    const titleInput = screen.getByPlaceholderText('e.g. Win a $100 Gift Card!');
    act(() => {
      fireEvent.change(titleInput, { target: { value: 'Win a New Car!' } });
    });

    const textElements = screen.getAllByText('Win a New Car!');
    expect(textElements.length).toBeGreaterThan(0);
  });

  it('shows paywall when removing branding without pro', async () => {
    act(() => { render(<GiveawayPage />); });

    const removeBrandingCheckbox = document.getElementById('removeBranding') as HTMLInputElement;
    act(() => {
      fireEvent.click(removeBrandingCheckbox);
    });

    await waitFor(() => {
        expect(screen.getAllByText('Upgrade to Pro')[0]).toBeDefined();
    });
  });
});
