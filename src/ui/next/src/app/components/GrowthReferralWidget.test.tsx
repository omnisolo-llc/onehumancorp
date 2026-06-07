import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { vi } from 'vitest';
import '@testing-library/jest-dom';
import GrowthReferralWidget from './GrowthReferralWidget';

describe('GrowthReferralWidget', () => {
  beforeEach(() => {
    global.fetch = vi.fn() as unknown as typeof fetch;
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
    render(<GrowthReferralWidget />);
    expect(screen.getByText('Grow Your Team')).toBeInTheDocument();
    expect(screen.getByText('Get My Invite Link')).toBeInTheDocument();
  });

  it('generates a link successfully', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ invite_link: 'https://ohc.app/invite/123' }),
    });

    render(<GrowthReferralWidget />);

    const button = screen.getByText('Get My Invite Link');
    fireEvent.click(button);

    expect(screen.getByText('Generating...')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByDisplayValue('https://ohc.app/invite/123')).toBeInTheDocument();
    });

    expect(screen.getByText('Copy')).toBeInTheDocument();
    expect(screen.getByText('Share on WhatsApp')).toBeInTheDocument();
  });

  it('handles error when generating link', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
    });

    render(<GrowthReferralWidget />);

    const button = screen.getByText('Get My Invite Link');
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByText('Failed to generate invite')).toBeInTheDocument();
    });
  });

  it('copies link to clipboard', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ invite_link: 'https://ohc.app/invite/123' }),
    });

    render(<GrowthReferralWidget />);
    fireEvent.click(screen.getByText('Get My Invite Link'));

    await waitFor(() => {
      expect(screen.getByDisplayValue('https://ohc.app/invite/123')).toBeInTheDocument();
    });

    const copyButton = screen.getByText('Copy');
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith('https://ohc.app/invite/123');
    expect(screen.getByText('Copied!')).toBeInTheDocument();
  });
});
