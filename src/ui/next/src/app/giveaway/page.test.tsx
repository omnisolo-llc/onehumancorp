import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import GiveawayGeneratorPage from './page';

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

  it('renders correctly', async () => {
    await act(async () => { render(<GiveawayGeneratorPage />); });
    expect(screen.getByText('Viral Giveaway Generator 🎁')).toBeDefined();
    expect(screen.getByText('Giveaway Details')).toBeDefined();
  });

  it('updates preview when inputs change', async () => {
    await act(async () => { render(<GiveawayGeneratorPage />); });

    const titleInput = screen.getByPlaceholderText('e.g. Win a $100 Gift Card!');
    await act(async () => { fireEvent.change(titleInput, { target: { value: 'Win a Free Pizza!' } }); });

    // Check if the preview updates
    const textElements = screen.getAllByText('Win a Free Pizza!');
    expect(textElements.length).toBeGreaterThan(0);
  });
});
