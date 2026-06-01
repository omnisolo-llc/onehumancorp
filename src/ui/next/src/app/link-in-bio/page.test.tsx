import { render, screen, fireEvent, act } from '@testing-library/react';
import LinkInBioPage from './page';
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';

describe('LinkInBioPage', () => {
  let clipboardText = '';

  beforeEach(() => {
    localStorage.clear();
    localStorage.setItem('tenant', 'test-store');

    // Mock navigator.clipboard
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockImplementation((text) => {
          clipboardText = text;
          return Promise.resolve();
        }),
      },
    });

    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('renders the link-in-bio page correctly', () => {
    render(<LinkInBioPage />);
    expect(screen.getByText('Viral Link-in-Bio')).toBeTruthy();
    expect(screen.getByText('Share Your Business Everywhere')).toBeTruthy();
  });

  it('loads tenant from local storage and displays links', () => {
    render(<LinkInBioPage />);

    // Verify the bio URL
    expect(screen.getByText('https://ohc.bio/test-store')).toBeTruthy();

    // Verify links are rendered
    expect(screen.getByText('Shop My Storefront')).toBeTruthy();
    expect(screen.getByText('https://ohc.store/test-store')).toBeTruthy();
  });

  it('allows copying the bio link', async () => {
    render(<LinkInBioPage />);

    const copyButton = screen.getByRole('button', { name: 'Copy Link' });

    await act(async () => {
      fireEvent.click(copyButton);
    });

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith('https://ohc.bio/test-store');
    expect(screen.getByRole('button', { name: 'Copied!' })).toBeTruthy();

    // Wait for timeout to reset
    await act(async () => {
      vi.advanceTimersByTime(2500);
    });

    expect(screen.getByRole('button', { name: 'Copy Link' })).toBeTruthy();
  });

  it('renders the live preview with powered by badge', () => {
    render(<LinkInBioPage />);

    expect(screen.getByText('Live Preview')).toBeTruthy();
    expect(screen.getByText('@test-store')).toBeTruthy();
    expect(screen.getByText(/Powered by/)).toBeTruthy();
    expect(screen.getByText('OHC')).toBeTruthy();
  });
});
