import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ReviewCampaignsPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('ReviewCampaignsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
    // Reset window.open mock
    window.open = vi.fn();
  });

  it('renders the page and generates a draft', async () => {
    render(<ReviewCampaignsPage />);

    // Check initial render
    expect(screen.getByText('Automated Review Campaigns ⭐️')).toBeDefined();

    // Simulate generation
    const generateBtn = screen.getByText(/Generate Email Draft/i);
    fireEvent.click(generateBtn);

    // Should show drafting state, then generated text
    await waitFor(() => {
      expect(screen.getByText(/AI Generated Draft/i)).toBeDefined();
    });
  });

  it('shows upgrade modal on send when no pro and no trial', async () => {
    render(<ReviewCampaignsPage />);

    // Click Generate to reveal Send button
    fireEvent.click(screen.getByText(/Generate Email Draft/i));

    await waitFor(() => {
      const sendBtn = screen.getByText(/Send to Audience/i);
      fireEvent.click(sendBtn);
    });

    // Upgrade modal should appear
    expect(screen.getByText('Unlock Automated Campaigns')).toBeDefined();
  });

  it('unlocks trial when sharing on X', async () => {
    render(<ReviewCampaignsPage />);

    // Click Generate to reveal Send button
    fireEvent.click(screen.getByText(/Generate Email Draft/i));

    await waitFor(() => {
      const sendBtn = screen.getByText(/Send to Audience/i);
      fireEvent.click(sendBtn);
    });

    // Click the share to unlock button
    const shareBtn = screen.getByText(/Unlock 1 Free Send by sharing on X/i);
    fireEvent.click(shareBtn);

    // Assert localStorage was updated
    expect(localStorage.getItem('ohc_pro_trial_unlocked')).toBe('true');
    expect(window.open).toHaveBeenCalledWith(
      expect.stringContaining('twitter.com/intent/tweet'),
      '_blank'
    );

    // Modal should close (wait for re-render)
    await waitFor(() => {
      expect(screen.queryByText('Unlock Automated Campaigns')).toBeNull();
    });

    // Send button should now successfully send
    const sendBtn = screen.getByText(/Send to Audience/i);
    fireEvent.click(sendBtn);

    await waitFor(() => {
      expect(screen.getByText(/Campaign sent to/i)).toBeDefined();
    });
  });
});
