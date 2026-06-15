import React from 'react';
import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import HybridLandingPage from './page';

describe('HybridLandingPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the hybrid landing page with two main cards', () => {
    render(<HybridLandingPage />);

    // Check main headings
    expect(screen.getByText('OHC Hybrid OS')).toBeDefined();

    // Check card 1: Local Sovereignty
    expect(screen.getByText('Local Sovereignty')).toBeDefined();
    expect(screen.getByText(/Zero Data Leakage/i)).toBeDefined();
    expect(screen.getByText(/Air-Gapped Autonomy/i)).toBeDefined();
    expect(screen.getByText(/Self-Hosted LLMs/i)).toBeDefined();

    // Check card 2: Cloud Convenience
    expect(screen.getByText('Cloud Convenience')).toBeDefined();
    expect(screen.getByText(/Team Collaboration/i)).toBeDefined();
    expect(screen.getByText(/Anywhere Access/i)).toBeDefined();
    expect(screen.getByText(/Fully Managed/i)).toBeDefined();
  });

  it('downloads desktop app when CTA is clicked', async () => {
    // The handleDownload function calls window.alert
    const mockAlert = vi.spyOn(window, 'alert').mockImplementation(() => {});

    render(<HybridLandingPage />);

    const downloadButton = screen.getByText('Download Desktop');
    expect(downloadButton).toBeDefined();

    // Click the button
    await act(async () => { fireEvent.click(downloadButton); });

    // Verify downloading state
    expect(screen.getByText('Downloading...')).toBeDefined();

    // Wait for async operations
    await act(async () => {
      await vi.waitFor(() => {
          expect(mockAlert).toHaveBeenCalledWith("Desktop App Download Started! (Simulation)");
      }, { timeout: 2000 });
    });

    // Cleanup mocks
    mockAlert.mockRestore();
  });

  it('navigates to dashboard for cloud trial', () => {
    render(<HybridLandingPage />);

    const startTrialLink = screen.getByText('Start Web Trial');
    expect(startTrialLink).toBeDefined();
    // Since it's a Next.js Link component, we check its href attribute
    expect(startTrialLink.getAttribute('href')).toBe('/dashboard');
  });
});
