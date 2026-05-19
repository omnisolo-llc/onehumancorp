import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import BuilderPage from '../page';

describe('BuilderPage', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn(() =>
      Promise.resolve({
        json: () => Promise.resolve({ blocks: [{ type: 'Hero', props: { headline: 'Generated Hero', copy: "Hero copy" } }] })
      })
    ));
    vi.useFakeTimers({ shouldAdvanceTime: true });
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.useRealTimers();
  });

  it('renders initial state', () => {
    render(<BuilderPage />);
    expect(screen.getByText('Welcome to OHC Smart Builder')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Build My Storefront' })).toBeDisabled();
  });

  it('enables button when input is sufficient and handles generation flow', async () => {
    render(<BuilderPage />);

    const textarea = screen.getByPlaceholderText(/e.g. I run a mobile dog grooming service/i);
    fireEvent.change(textarea, { target: { value: 'Valid business description' } });

    const buildButton = screen.getByRole('button', { name: 'Build My Storefront' });
    expect(buildButton).toBeEnabled();

    fireEvent.click(buildButton);

    expect(screen.getByText(/The Promoter is picking colors/i)).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Preview Mode')).toBeInTheDocument();
    });

    // Check block generation mock was applied
    expect(screen.getByText('Generated Hero')).toBeInTheDocument();

    const launchButton = screen.getByRole('button', { name: '1-Tap Launch' });
    fireEvent.click(launchButton);

    // Run timers for the 1500ms delay in handleLaunch
    vi.advanceTimersByTime(1500);

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
    });

    const goDashboardButton = screen.getByRole('button', { name: 'Go to Dashboard' });
    fireEvent.click(goDashboardButton);

    expect(screen.getByText('Welcome to OHC Smart Builder')).toBeInTheDocument();
  });

  it('handles error in generation flow gracefully', async () => {
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    vi.stubGlobal('fetch', vi.fn(() => Promise.reject(new Error('Fetch failed'))));

    render(<BuilderPage />);

    const textarea = screen.getByPlaceholderText(/e.g. I run a mobile dog grooming service/i);
    fireEvent.change(textarea, { target: { value: 'Valid business description' } });

    const buildButton = screen.getByRole('button', { name: 'Build My Storefront' });
    fireEvent.click(buildButton);

    await waitFor(() => {
        expect(consoleSpy).toHaveBeenCalledWith("Failed to generate storefront", expect.any(Error));
    });

    expect(screen.getByText('Welcome to OHC Smart Builder')).toBeInTheDocument();
    consoleSpy.mockRestore();
  });
});
