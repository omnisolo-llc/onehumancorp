import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import TeamPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

// Mock the fetch call for approvals
global.fetch = vi.fn(() =>
  Promise.resolve({
    ok: true,
    json: () => Promise.resolve({ pending_approvals: [] }),
  })
) as any;

describe('TeamPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the grow your team section and modal interaction', async () => {
    render(
      <TooltipProvider>
        <TeamPage />
      </TooltipProvider>
    );

    // Check loading state first
    await waitFor(() => {
        expect(screen.getByText('Grow Your Team')).toBeDefined();
    });

    // Check modal opens
    const inviteBtn = screen.getByRole('button', { name: 'Invite to Cloud Team' });
    fireEvent.click(inviteBtn);

    expect(screen.getByRole('heading', { name: 'Cloud Bridge Invite' })).toBeDefined();

    // Check modal closes
    const closeBtn = screen.getByRole('button', { name: 'Close Cloud Bridge Invite' });
    fireEvent.click(closeBtn);

    expect(screen.queryByRole('heading', { name: 'Cloud Bridge Invite' })).toBeNull();
  });
});
