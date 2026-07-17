import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
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
  });

  it('renders Powered by OHC branding in preview by default', () => {
    render(<ProjectShowcasePage />);

    const brandingElements = screen.getAllByTestId('powered-by-ohc');
    expect(brandingElements.length).toBeGreaterThan(0);
    expect(brandingElements[0]).toBeTruthy();
  });

  it('shows paywall when free user tries to remove branding', () => {
    localStorage.setItem('has_pro', 'false');
    render(<ProjectShowcasePage />);

    const toggle = screen.getByRole('checkbox');
    fireEvent.click(toggle);

    expect(screen.getByText('Upgrade to Pro')).toBeTruthy();

    const brandingElements = screen.getAllByTestId('powered-by-ohc');
    expect(brandingElements[0]).toBeTruthy();
  });

  it('allows pro users to remove branding', () => {
    localStorage.setItem('has_pro', 'true');
    render(<ProjectShowcasePage />);

    // Checkbox should be checked by default for pro users (per useEffect)
    const toggle = screen.getByRole('checkbox');
    expect(toggle).toBeChecked();

    // PoweredBy should not be rendered
    expect(screen.queryByTestId('powered-by-ohc')).toBeNull();
  });
});
