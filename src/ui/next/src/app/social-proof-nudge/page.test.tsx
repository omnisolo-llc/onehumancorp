import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import SocialProofNudgePage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('SocialProofNudgePage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'free' }) });
  });

  it('renders the Social Proof Nudge page correctly', () => {
    render(<SocialProofNudgePage />);
    expect(screen.getByText('Social Proof Nudge 🚀')).toBeDefined();
    expect(screen.getByText('Boost Sales with FOMO')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('updates the live preview and embed code when input changes', () => {
    render(<SocialProofNudgePage />);

    const productInput = screen.getByPlaceholderText('e.g. Signature Coffee Blend');
    fireEvent.change(productInput, { target: { value: 'Magic Wand' } });

    expect(screen.getByText('Magic Wand')).toBeDefined();
    expect(screen.getByText(/data-product="Magic Wand"/)).toBeDefined();

    const locationInput = screen.getByPlaceholderText('e.g. Someone in London');
    fireEvent.change(locationInput, { target: { value: 'Alice from Wonderland' } });

    // The component splits the location display differently. It shows "Alice from Wonderland purchased"
    expect(screen.getByText((content, element) => content.startsWith('Alice from Wonderland'))).toBeDefined();
    expect(screen.getByText(/data-location="Alice from Wonderland"/)).toBeDefined();
  });

  it('escapes hostile values in generated HTML attributes', () => {
    render(<SocialProofNudgePage />);
    fireEvent.change(screen.getByPlaceholderText('e.g. Signature Coffee Blend'), {
      target: { value: '\"><script>alert(1)</script>&' },
    });

    const code = document.querySelector('#embed-code')?.textContent ?? '';
    expect(code).toContain('&quot;&gt;&lt;script&gt;alert(1)&lt;/script&gt;&amp;');
    expect(code).not.toContain('data-product=""><script>');
  });

  it('shows the soft paywall when trying to remove branding without Pro', () => {
    render(<SocialProofNudgePage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    fireEvent.click(removeBrandingCheckbox);

    // Expect the paywall modal to appear
    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
    expect(screen.getByText('Upgrade to Pro')).toBeDefined();
  });

  it('allows removing branding if the plan API reports Pro', async () => {
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'pro' }) });
    render(<SocialProofNudgePage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/my-plan'));
    fireEvent.click(removeBrandingCheckbox);

    expect(screen.queryByText('Upgrade to Remove Branding')).toBeNull();
  });
});
