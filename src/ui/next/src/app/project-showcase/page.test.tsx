import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import '@testing-library/jest-dom/vitest';
import { act } from 'react';
import ProjectShowcasePage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc">⚡ Powered by OHC</div>,
}));

describe('ProjectShowcasePage', () => {
  beforeEach(() => {
    localStorage.clear();
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'free' }) });
  });

  it('renders Powered by OHC branding in preview by default', () => {
    act(() => { render(<ProjectShowcasePage />); });

    const brandingElements = screen.getAllByTestId('powered-by-ohc');
    expect(brandingElements.length).toBeGreaterThan(0);
    expect(brandingElements[0]).toBeInTheDocument();
  });

  it('shows paywall when free user tries to remove branding', () => {
    act(() => { render(<ProjectShowcasePage />); });

    const toggle = screen.getByRole('checkbox');
    fireEvent.click(toggle);

    expect(screen.getByText('Upgrade to Pro')).toBeInTheDocument();

    const brandingElements = screen.getAllByTestId('powered-by-ohc');
    expect(brandingElements[0]).toBeInTheDocument();
  });

  it('allows pro users to remove branding after the plan API confirms pro', async () => {
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'pro' }) });
    act(() => { render(<ProjectShowcasePage />); });

    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/my-plan'));
    const toggle = screen.getByRole('checkbox');
    fireEvent.click(toggle);
    expect(toggle).toBeChecked();

    // PoweredBy should not be rendered
    expect(screen.queryByTestId('powered-by-ohc')).toBeNull();
  });
});
