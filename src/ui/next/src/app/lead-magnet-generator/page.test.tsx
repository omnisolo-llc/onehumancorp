import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, beforeEach, vi } from 'vitest';
import '@testing-library/jest-dom/vitest';
import { act } from 'react';
import LeadMagnetGeneratorPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
  usePathname: () => '/lead-magnet-generator',
  useSearchParams: () => new URLSearchParams('?tenant=my-store'),
}));

describe('LeadMagnetGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'free' }) });
  });

  it('renders the configurator with default values', () => {
    render(
      <TooltipProvider>
        <LeadMagnetGeneratorPage />
      </TooltipProvider>
    );

    expect(screen.getByText('Lead Magnet Generator')).toBeInTheDocument();

    // Check inputs
    const headlineInput = screen.getByDisplayValue('Unlock the Ultimate Business Checklist');
    expect(headlineInput).toBeInTheDocument();

    // Check embed code generated
    expect(screen.getByText(/<iframe src="https:\/\/ohc.app\/api\/v1\/growth\/lead-magnet\/embed/)).toBeInTheDocument();
  });

  it('updates embed code when inputs change', async () => {
    render(
      <TooltipProvider>
        <LeadMagnetGeneratorPage />
      </TooltipProvider>
    );

    const headlineInput = screen.getByDisplayValue('Unlock the Ultimate Business Checklist');

    fireEvent.change(headlineInput, { target: { value: 'New Custom Headline' } });

    await waitFor(() => {
        expect(screen.getByText(/title=New%20Custom%20Headline/)).toBeInTheDocument();
    });
  });

  it('displays soft paywall when attempting to remove branding without Pro', async () => {
    render(
      <TooltipProvider>
        <LeadMagnetGeneratorPage />
      </TooltipProvider>
    );

    const checkbox = screen.getByLabelText(/Remove "Powered by OHC" Branding/i);
    fireEvent.click(checkbox);

    await waitFor(() => {
        expect(screen.getByText('Upgrade to OHC Pro')).toBeInTheDocument();
    });
  });

  it('removes branding when Pro is active', async () => {
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'pro' }) });
    render(
      <TooltipProvider>
        <LeadMagnetGeneratorPage />
      </TooltipProvider>
    );

    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/my-plan'));
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
    render(
      <TooltipProvider>
        <LeadMagnetGeneratorPage />
      </TooltipProvider>
    );

    const links = screen.getAllByText('⚡ Powered by OHC');
    expect(links.length).toBeGreaterThan(0);
  });
});
