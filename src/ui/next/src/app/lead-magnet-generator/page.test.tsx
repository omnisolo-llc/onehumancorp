import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, beforeEach, vi } from 'vitest';
import LeadMagnetGeneratorPage from './page';

// Mock useRouter
vi.mock('next/navigation', () => ({
  useRouter() {
    return {
      push: vi.fn(),
    };
  },
}));

describe('LeadMagnetGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  it('renders the configurator with default values', () => {
    render(<LeadMagnetGeneratorPage />);

    expect(screen.getByText('Lead Magnet Generator')).toBeInTheDocument();

    // Check inputs
    const headlineInput = screen.getByDisplayValue('Unlock the Ultimate Business Checklist');
    expect(headlineInput).toBeInTheDocument();

    // Check embed code generated
    expect(screen.getByText(/<iframe src="https:\/\/ohc.app\/api\/v1\/growth\/lead-magnet\/embed/)).toBeInTheDocument();
  });

  it('updates embed code when inputs change', async () => {
    render(<LeadMagnetGeneratorPage />);

    const headlineInput = screen.getByDisplayValue('Unlock the Ultimate Business Checklist');

    fireEvent.change(headlineInput, { target: { value: 'New Custom Headline' } });

    await waitFor(() => {
        expect(screen.getByText(/title=New%20Custom%20Headline/)).toBeInTheDocument();
    });
  });

  it('displays soft paywall when attempting to remove branding without Pro', async () => {
    render(<LeadMagnetGeneratorPage />);

    const checkbox = screen.getByLabelText(/Remove "Powered by OHC" Branding/i);
    fireEvent.click(checkbox);

    await waitFor(() => {
        expect(screen.getByText('Upgrade to OHC Pro')).toBeInTheDocument();
    });
  });

  it('removes branding when Pro is active', async () => {
    localStorage.setItem('has_pro', 'true');
    render(<LeadMagnetGeneratorPage />);

    const checkbox = screen.getByLabelText(/Remove "Powered by OHC" Branding/i);
    fireEvent.click(checkbox);

    await waitFor(() => {
        // The modal should NOT appear
        expect(screen.queryByText('Upgrade to OHC Pro')).not.toBeInTheDocument();
        // The embed code should include hideBranding=true
        expect(screen.getByText(/hideBranding=true/)).toBeInTheDocument();
    });
  });

  it('has powered by OHC footer in preview by default', () => {
    render(<LeadMagnetGeneratorPage />);

    const links = screen.getAllByText('⚡ Powered by OHC');
    expect(links.length).toBeGreaterThan(0);
  });
});
