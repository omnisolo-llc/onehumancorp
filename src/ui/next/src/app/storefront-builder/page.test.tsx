import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import StorefrontBuilderPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';

// Mock TooltipRegistry and help components
vi.mock('../../components/Walkthrough', () => ({
  WalkthroughTarget: ({ children, id }: any) => <div id={id}>{children}</div>,
  InteractiveWalkthrough: () => null
}));
vi.mock('../../components/TooltipRegistry', () => ({
  TooltipProvider: ({ children }: any) => <>{children}</>,
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
    render(<TooltipProvider><StorefrontBuilderPage /></TooltipProvider>);
    expect(screen.getByText('Welcome to OHC Smart Builder')).toBeTruthy();
    expect(screen.getByText('Build My Storefront')).toBeTruthy();
  });

  it('handles empty input generation by keeping button disabled', () => {
    render(<TooltipProvider><StorefrontBuilderPage /></TooltipProvider>);
    const button = screen.getByText('Build My Storefront');
    expect(button.className).toContain('cursor-not-allowed');
  });

  it('enables button with valid input and calls generate', async () => {
    render(<TooltipProvider><StorefrontBuilderPage /></TooltipProvider>);

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
      expect(screen.getByText('⚡ Powered by OHC')).toBeTruthy();
    });
  });

  it('handles publish workflow correctly', async () => {
    render(<TooltipProvider><StorefrontBuilderPage /></TooltipProvider>);

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

  it('handles chat with agent workflow', async () => {
    render(<TooltipProvider><StorefrontBuilderPage /></TooltipProvider>);

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
      expect(screen.getByText('Agent')).toBeTruthy();
    });

    fireEvent.click(screen.getByText('Agent'));

    await act(async () => {
        await new Promise(resolve => setTimeout(resolve, 0));
    });
    expect(screen.queryAllByText(/Marketing Agent/i).length).toBeGreaterThan(0);
    const chatTextarea = screen.getByPlaceholderText(/e.g. Add a new product.../i);
    fireEvent.change(chatTextarea, { target: { value: "Add a new product" } });

    // It works! We just want to check if chat screen is open
  });
});
