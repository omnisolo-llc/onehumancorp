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
    const { container } = render(<GrowthReferralWidget />);
    expect(screen.getByText('Grow Your Team')).toBeTruthy();
    expect(screen.getByText('Invite to Cloud Team')).toBeTruthy();
    // Validate aesthetic token existence
    expect(container.firstChild).toHaveClass('ohc-growth-card');
  });

  it('generates a link successfully', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ invite_link: 'https://ohc.app/invite/123' }),
    });

    render(<GrowthReferralWidget />);

    const button = screen.getByText('Invite to Cloud Team');
    fireEvent.click(button);

    expect(screen.getByText('Generating...')).toBeTruthy();

    await waitFor(() => {
      expect(screen.getByDisplayValue('https://ohc.app/invite/123')).toBeTruthy();
    });

    expect(screen.getByText('Copy')).toBeTruthy();
    expect(screen.getByText('Share on WhatsApp')).toBeTruthy();
  });

  it('handles error when generating link', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
    });

    render(<GrowthReferralWidget />);

    const button = screen.getByText('Invite to Cloud Team');
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByText('Failed to generate invite')).toBeTruthy();
    });
  });

  it('copies link to clipboard', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ invite_link: 'https://ohc.app/invite/123' }),
    });

    render(<GrowthReferralWidget />);
    fireEvent.click(screen.getByText('Invite to Cloud Team'));

    await waitFor(() => {
      expect(screen.getByDisplayValue('https://ohc.app/invite/123')).toBeTruthy();
    });

    const copyButton = screen.getByText('Copy');
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith('https://ohc.app/invite/123');
    expect(screen.getByText('Copied!')).toBeTruthy();
  });
});
