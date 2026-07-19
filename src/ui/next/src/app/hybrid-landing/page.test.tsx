import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
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

  it('fails closed when no desktop installer is available', () => {
    render(<HybridLandingPage />);

    const downloadButton = screen.getByText('Download Desktop');
    expect(downloadButton).toBeDefined();

    // Click the button
    fireEvent.click(downloadButton);
    expect(screen.getByText('The desktop installer is not available for download.')).toBeDefined();
  });

  it('navigates to dashboard for cloud trial', () => {
    render(<HybridLandingPage />);

    const startTrialLink = screen.getByText('Start Web Trial');
    expect(startTrialLink).toBeDefined();
    // Since it's a Next.js Link component, we check its href attribute
    expect(startTrialLink.getAttribute('href')).toBe('/dashboard');
  });
});
