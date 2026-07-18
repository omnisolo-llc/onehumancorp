import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { expect, test, vi, beforeEach } from 'vitest';
import ViralTrustBadgeBuilderPage from './page';

// Mock PoweredByOHC component
vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc">Powered by OHC</div>
}));

// Mock useRouter
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

// Mock window.open
const mockOpen = vi.fn();
Object.defineProperty(window, 'open', { value: mockOpen });

describe('ViralTrustBadgeBuilderPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  test('renders the builder with default values', () => {
    render(<ViralTrustBadgeBuilderPage />);

    // Check header
    expect(screen.getByText('Trust Badge Builder 🛡️')).toBeInTheDocument();

    // Check settings panel
    expect(screen.getByText('Badge Settings')).toBeInTheDocument();

    // Check default input values
    const businessNameInput = screen.getByDisplayValue('My Store');
    expect(businessNameInput).toBeInTheDocument();

    const statLabelInput = screen.getByDisplayValue('Happy Customers');
    expect(statLabelInput).toBeInTheDocument();

    const statValueInput = screen.getByDisplayValue('500+');
    expect(statValueInput).toBeInTheDocument();

    // Check preview updates
    expect(screen.getByText('Happy Customers')).toBeInTheDocument();
    expect(screen.getByText('500+ at My Store')).toBeInTheDocument();
  });

  test('updates preview when inputs change', () => {
    render(<ViralTrustBadgeBuilderPage />);

    const businessNameInput = screen.getByDisplayValue('My Store');
    fireEvent.change(businessNameInput, { target: { value: 'Awesome Bakery' } });

    const statLabelInput = screen.getByDisplayValue('Happy Customers');
    fireEvent.change(statLabelInput, { target: { value: 'Cakes Baked' } });

    const statValueInput = screen.getByDisplayValue('500+');
    fireEvent.change(statValueInput, { target: { value: '10,000' } });

    // Check updated preview
    expect(screen.getByText('Cakes Baked')).toBeInTheDocument();
    expect(screen.getByText('10,000 at Awesome Bakery')).toBeInTheDocument();
  });

  test('shows paywall when trying to remove branding without Pro', () => {
    render(<ViralTrustBadgeBuilderPage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    expect(removeBrandingCheckbox).not.toBeChecked();

    fireEvent.click(removeBrandingCheckbox);

    // Paywall modal should appear
    expect(screen.getByText('Upgrade to Remove Branding')).toBeInTheDocument();
    expect(screen.getByText('Upgrade to Pro')).toBeInTheDocument();
    expect(screen.getByText('Share on X to get 7 Days Free')).toBeInTheDocument();
  });

  test('allows removing branding when Pro is active', () => {
    // Setup Pro state
    localStorage.setItem('has_pro', 'true');
    render(<ViralTrustBadgeBuilderPage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    fireEvent.click(removeBrandingCheckbox);

    // Paywall modal should NOT appear
    expect(screen.queryByText('Upgrade to Remove Branding')).not.toBeInTheDocument();
  });

  test('generates embed code snippet', () => {
    render(<ViralTrustBadgeBuilderPage />);

    const embedCodeContainer = document.getElementById('embed-code');
    expect(embedCodeContainer).toBeInTheDocument();

    // Check if the snippet contains the dynamic values
    expect(embedCodeContainer?.textContent).toContain('Happy Customers');
    expect(embedCodeContainer?.textContent).toContain('500+ at My Store');
    expect(embedCodeContainer?.textContent).toContain('Powered by OHC');
  });
});
