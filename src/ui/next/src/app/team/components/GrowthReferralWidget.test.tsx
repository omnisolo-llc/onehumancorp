import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { vi, describe, beforeEach, afterEach, it, expect } from 'vitest';
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
    expect(screen.getByText('Grow Your Team')).toBeDefined();
    expect(screen.getByText('Invite to Cloud Team')).toBeDefined();
  });

  it('generates a link successfully', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ invite_link: 'https://ohc.app/invite/123' }),
    });

    render(<GrowthReferralWidget />);

    const button = screen.getByText('Invite to Cloud Team');
    fireEvent.click(button);

    expect(screen.getByText('Generating...')).toBeDefined();

    await waitFor(() => {
      expect((screen.getByRole('textbox') as HTMLInputElement).value).toBe('https://ohc.app/invite/123');
    });

    expect(screen.getByText('Copy')).toBeDefined();
    expect(screen.getByText('Share on WhatsApp')).toBeDefined();
  });

  it('handles error when generating link', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
    });

    render(<GrowthReferralWidget />);

    const button = screen.getByText('Invite to Cloud Team');
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByText('Failed to generate invite')).toBeDefined();
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
      expect((screen.getByRole('textbox') as HTMLInputElement).value).toBe('https://ohc.app/invite/123');
    });

    const copyButton = screen.getByText('Copy');
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith('https://ohc.app/invite/123');
    expect(screen.getByText('Copied!')).toBeDefined();
  });
});
