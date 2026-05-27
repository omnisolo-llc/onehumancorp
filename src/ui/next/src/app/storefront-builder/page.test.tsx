import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import StorefrontBuilderPage from './page';
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';

// Mock TooltipRegistry and help components
vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: any) => <div>{children}</div>
}));
vi.mock('../../components/help', () => ({
  useWalkthrough: () => ({ startWalkthrough: vi.fn() })
}));

describe('StorefrontBuilderPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockResolvedValue({
        json: () => Promise.resolve({ data: {} })
    });
    localStorage.clear();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it('renders initial setup state', () => {
    render(<StorefrontBuilderPage />);
    expect(screen.getByText('Welcome to OHC Smart Builder')).toBeTruthy();
    expect(screen.getByText('Build My Storefront')).toBeTruthy();
  });

  it('handles empty input generation by keeping button disabled', () => {
    render(<StorefrontBuilderPage />);
    const button = screen.getByText('Build My Storefront');
    expect(button.className).toContain('cursor-not-allowed');
  });

  it('enables button with valid input and calls generate', async () => {
    render(<StorefrontBuilderPage />);

    const textarea = screen.getByPlaceholderText(/e.g. I run a mobile dog grooming service/i);
    fireEvent.change(textarea, { target: { value: 'Valid long business bio' } });

    const button = screen.getByText('Build My Storefront');
    expect(button.className).not.toContain('cursor-not-allowed');

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Test Hero' } }
          ]
        }]
      })
    });

    fireEvent.click(button);

    expect(screen.getByText('Agents are building your store...')).toBeTruthy();

    await waitFor(() => {
      expect(screen.getByText('Preview Mode')).toBeTruthy();
    });
  });

  it('handles publish workflow correctly', async () => {
    render(<StorefrontBuilderPage />);

    // Setup state manually or go through flow
    const textarea = screen.getByPlaceholderText(/e.g. I run a mobile dog grooming service/i);
    fireEvent.change(textarea, { target: { value: 'Valid long business bio' } });

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Test Hero' } }
          ]
        }]
      })
    });

    fireEvent.click(screen.getByText('Build My Storefront'));

    await waitFor(() => {
      expect(screen.getByText('1-Tap Launch')).toBeTruthy();
    });

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ domain: 'test' })
    });

    fireEvent.click(screen.getByText('1-Tap Launch'));

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeTruthy();
    });
  });
});
