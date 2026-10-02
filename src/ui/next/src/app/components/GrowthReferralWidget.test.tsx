import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { vi } from 'vitest';
import '@testing-library/jest-dom';
import GrowthReferralWidget from './GrowthReferralWidget';

describe('GrowthReferralWidget', () => {
  let inviteReply: () => Promise<Response>;
  beforeEach(() => {
    localStorage.clear();
    inviteReply = async () => Response.json({ invite_link: 'https://cloud.omnisolo.co/invite/123' });
    global.fetch = vi.fn(async url => url === '/api/v1/auth/session-identity'
      ? Response.json({ userId: 'owner', tenantId: 'tenant', expiresAt: Date.now() + 60_000 })
      : inviteReply());
    Object.defineProperty(navigator, 'locks', { value: { request: async (_name: string, _options: unknown, callback: (lock: object) => Promise<void>) => callback({}) } });
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockImplementation(() => Promise.resolve()),
      },
    });
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it('renders correctly', () => {
    const { container } = render(<GrowthReferralWidget />);
    expect(screen.getByText('Grow Your Team')).toBeInTheDocument();
    expect(screen.getByText('Invite to Cloud Team')).toBeInTheDocument();
    // Validate aesthetic token existence
    expect(container.firstChild).toHaveClass('omnisolo-growth-card');
  });

  it('generates a link successfully', async () => {


    render(<GrowthReferralWidget />);

    const button = screen.getByText('Invite to Cloud Team');
    await waitFor(() => expect(button).toBeEnabled());
    fireEvent.click(button);

    expect(screen.getByText('Generating...')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByDisplayValue('https://cloud.omnisolo.co/invite/123')).toBeInTheDocument();
    });

    expect(screen.getByText('Copy')).toBeInTheDocument();
    expect(screen.getByText('Share on WhatsApp')).toBeInTheDocument();
  });

  it('handles error when generating link', async () => {
    inviteReply = async () => Response.json({ error: 'unavailable' }, { status: 503 });

    render(<GrowthReferralWidget />);

    const button = screen.getByText('Invite to Cloud Team');
    await waitFor(() => expect(button).toBeEnabled());
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByRole('status', { name: 'Team invitation status' })).toHaveTextContent('could not be confirmed or saved');
    });
  });

  it('copies link to clipboard', async () => {


    render(<GrowthReferralWidget />);
    await waitFor(() => expect(screen.getByText('Invite to Cloud Team')).toBeEnabled());
    fireEvent.click(screen.getByText('Invite to Cloud Team'));

    await waitFor(() => {
      expect(screen.getByDisplayValue('https://cloud.omnisolo.co/invite/123')).toBeInTheDocument();
    });

    const copyButton = screen.getByText('Copy');
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith('https://cloud.omnisolo.co/invite/123');
    expect(await screen.findByText('Copied!')).toBeInTheDocument();
  });
});
