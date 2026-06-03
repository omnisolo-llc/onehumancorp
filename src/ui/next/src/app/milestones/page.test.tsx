import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import MilestonesPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('MilestonesPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders milestones correctly', () => {
    render(<MilestonesPage />);
    expect(screen.getByText('Success Milestones 🏆')).toBeDefined();
    expect(screen.getByText('First Order! 🎉')).toBeDefined();
    expect(screen.getByText('$1,000 Revenue')).toBeDefined();
  });

  it('selects an unlocked milestone and updates share card', async () => {
    render(<MilestonesPage />);

    // Click on "10th Order Milestone"
    fireEvent.click(screen.getByText('10th Order Milestone'));

    await waitFor(() => {
      // The share card preview should show the milestone details
      expect(screen.getByText('Share Your Success')).toBeDefined();
      const milestoneElements = screen.getAllByText('10th Order Milestone');
      expect(milestoneElements.length).toBeGreaterThan(1);
      const descriptionElements = screen.getAllByText('Double digits! Your business is gaining momentum.');
      expect(descriptionElements.length).toBeGreaterThan(1);
    });
  });

  it('copies share message', async () => {
    render(<MilestonesPage />);

    // Ensure one milestone is selected
    fireEvent.click(screen.getByText('First Order! 🎉'));

    // Mock clipboard
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockImplementation(() => Promise.resolve()),
      },
    });

    const copyButton = screen.getByText('Copy Share Message');
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(expect.stringContaining('https://ohc.app/join?ref=milestone'));
    expect(screen.getByText('Copied Message!')).toBeDefined();
  });
});
