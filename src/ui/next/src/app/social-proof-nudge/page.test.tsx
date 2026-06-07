import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import SocialProofNudgePage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('SocialProofNudgePage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
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

  it('shows the soft paywall when trying to remove branding without Pro', () => {
    render(<SocialProofNudgePage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    fireEvent.click(removeBrandingCheckbox);

    // Expect the paywall modal to appear
    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
    expect(screen.getByText('Upgrade to Pro')).toBeDefined();
  });

  it('allows removing branding if user has Pro', () => {
    localStorage.setItem('has_pro', 'true');
    render(<SocialProofNudgePage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    // Shouldn't show paywall modal
    fireEvent.click(removeBrandingCheckbox);

    expect(screen.queryByText('Upgrade to Remove Branding')).toBeNull();
  });
});
