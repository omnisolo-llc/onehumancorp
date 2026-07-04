import { render, screen, fireEvent, waitFor } from '@testing-library/react';
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
    render(<GiveawayPage />);
    expect(screen.getByText('Viral Giveaway Generator 🎁')).toBeDefined();
    expect(screen.getByText('Giveaway Details')).toBeDefined();
  });

  it('updates preview when inputs change', () => {
    render(<GiveawayPage />);

    const titleInput = screen.getByPlaceholderText('e.g. Win a $100 Gift Card!');
    fireEvent.change(titleInput, { target: { value: 'Win a New Car!' } });

    const textElements = screen.getAllByText('Win a New Car!');
    expect(textElements.length).toBeGreaterThan(0);
  });

  it('shows paywall when removing branding without pro', async () => {
    render(<GiveawayPage />);

    const removeBrandingCheckbox = document.getElementById('removeBranding') as HTMLInputElement;
    fireEvent.click(removeBrandingCheckbox);

    await waitFor(() => {
        expect(screen.getAllByText('Upgrade to Pro')[0]).toBeDefined();
    });
  });
});
