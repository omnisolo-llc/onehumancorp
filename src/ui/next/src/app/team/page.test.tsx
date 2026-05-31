import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import TeamPage from './page';
import { TooltipProvider } from '../../../src/components/TooltipRegistry';

// Mock the fetch call for approvals
global.fetch = vi.fn();

describe('TeamPage - Viral Invite Loop', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({ pending_approvals: [] }),
    });
    // Mock navigator clipboard
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockImplementation(() => Promise.resolve()),
      },
    });
  });

  it('renders the Grow Your Team section', async () => {
    render(
      <TooltipProvider>
        <TeamPage />
      </TooltipProvider>
    );
    expect(screen.getByText('Grow Your Team')).toBeInTheDocument();
    expect(screen.getByText('Bridge your local sovereignty with cloud-native collaboration.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Invite to Cloud Team' })).toBeInTheDocument();
  });

  it('opens the Cloud Bridge Invite modal when clicking the invite button', async () => {
    render(
      <TooltipProvider>
        <TeamPage />
      </TooltipProvider>
    );

    // Initial state: modal should not be visible
    expect(screen.queryByText('Cloud Bridge Invite')).not.toBeInTheDocument();

    const inviteButton = screen.getByRole('button', { name: 'Invite to Cloud Team' });
    fireEvent.click(inviteButton);

    expect(screen.getByText('Cloud Bridge Invite')).toBeInTheDocument();
    expect(screen.getByText('Share this link to provision a temporary multi-tenant context.')).toBeInTheDocument();

    // Check that the input has a generated link
    const input = screen.getByRole('textbox') as HTMLInputElement;
    expect(input.value).toMatch(/https:\/\/ohc\.app\/invite\/bridge-\d+/);
  });

  it('copies the link when the Copy Link button is clicked', async () => {
    render(
      <TooltipProvider>
        <TeamPage />
      </TooltipProvider>
    );

    fireEvent.click(screen.getByRole('button', { name: 'Invite to Cloud Team' }));

    const copyButton = screen.getByRole('button', { name: 'Copy Link' });
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied!' })).toBeInTheDocument();

    // The text should revert after timeout, but we don't strictly need to wait for 2s in this basic test unless we use fake timers
  });

  it('closes the modal when the close button is clicked', async () => {
    render(
      <TooltipProvider>
        <TeamPage />
      </TooltipProvider>
    );

    fireEvent.click(screen.getByRole('button', { name: 'Invite to Cloud Team' }));
    expect(screen.getByText('Cloud Bridge Invite')).toBeInTheDocument();

    const closeButton = screen.getByRole('button', { name: 'Close Cloud Bridge Invite' });
    fireEvent.click(closeButton);

    expect(screen.queryByText('Cloud Bridge Invite')).not.toBeInTheDocument();
  });
});
