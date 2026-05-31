import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { vi } from 'vitest';
import ViralCloudInvite from './ViralCloudInvite';

// Mock clipboard API
Object.assign(navigator, {
  clipboard: {
    writeText: vi.fn().mockImplementation(() => Promise.resolve()),
  },
});

// Mock fetch
global.fetch = vi.fn();

describe('ViralCloudInvite Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly', () => {
    render(<ViralCloudInvite />);

    expect(screen.getByRole('heading', { name: 'Grow Your Team' })).toBeInTheDocument();
    expect(screen.getByText('Bring your team online easily. Share access to your workspace securely.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Invite Team Member' })).toBeInTheDocument();
  });

  it('opens modal and generates a link on button click (success)', async () => {
    (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ id: 'mock-invite-id' }),
    });

    render(<ViralCloudInvite />);

    const inviteButton = screen.getByRole('button', { name: 'Invite Team Member' });
    fireEvent.click(inviteButton);

    // Modal should be visible
    expect(screen.getByRole('heading', { name: 'Team Invite' })).toBeInTheDocument();
    expect(screen.getByText(/Share this secure link with your team member/i)).toBeInTheDocument();

    // Verify loading state
    const input = screen.getByDisplayValue('Generating link...');
    expect(input).toBeInTheDocument();

    // Wait for the mock API response
    await waitFor(() => {
      expect(screen.getByDisplayValue('https://ohc.app/invite/mock-invite-id')).toBeInTheDocument();
    });

    // Verify fetch call payload
    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'default' : 'default';
    expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/team-invites', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ team_id: tenantId, inviter_id: 'current-user', invitee_id: 'new-collaborator' })
    });
  });

  it('handles fallback link on API error', async () => {
    (global.fetch as ReturnType<typeof vi.fn>).mockRejectedValueOnce(new Error('Network error'));

    render(<ViralCloudInvite />);

    const inviteButton = screen.getByRole('button', { name: 'Invite Team Member' });
    fireEvent.click(inviteButton);

    // Wait for fallback
    await waitFor(() => {
      const input = screen.getByRole('textbox') as HTMLInputElement;
      expect(input.value).toMatch(/^https:\/\/ohc\.app\/invite\/temp-\d+/);
    });
  });

  it('copies the link to clipboard', async () => {
    (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ id: 'mock-invite-id' }),
    });

    render(<ViralCloudInvite />);

    fireEvent.click(screen.getByRole('button', { name: 'Invite Team Member' }));

    await waitFor(() => {
      expect(screen.getByDisplayValue('https://ohc.app/invite/mock-invite-id')).toBeInTheDocument();
    });

    const copyButton = screen.getByRole('button', { name: /Copy Link/i });
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith('https://ohc.app/invite/mock-invite-id');

    // Button text changes
    expect(screen.getByRole('button', { name: /Copied!/i })).toBeInTheDocument();
  });

  it('closes the modal when "Done" or "X" button is clicked', async () => {
    render(<ViralCloudInvite />);

    fireEvent.click(screen.getByRole('button', { name: 'Invite Team Member' }));

    expect(screen.getByRole('heading', { name: 'Team Invite' })).toBeInTheDocument();

    const closeButton = screen.getByRole('button', { name: 'Close Team Invite' });
    fireEvent.click(closeButton);

    expect(screen.queryByRole('heading', { name: 'Team Invite' })).not.toBeInTheDocument();

    // Reopen and try "Done" button
    fireEvent.click(screen.getByRole('button', { name: 'Invite Team Member' }));
    expect(screen.getByRole('heading', { name: 'Team Invite' })).toBeInTheDocument();

    const doneButton = screen.getByRole('button', { name: 'Done' });
    fireEvent.click(doneButton);

    expect(screen.queryByRole('heading', { name: 'Team Invite' })).not.toBeInTheDocument();
  });
});
