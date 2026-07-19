import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralProductWidgetPage from './page';

// Mock the AppShell component
vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell">{children}</div>,
}));

describe('ViralProductWidgetPage', () => {
  beforeEach(() => {
    // Clear localStorage before each test
    if (typeof window !== 'undefined' && window.localStorage) {
      window.localStorage.clear();
    }
    vi.clearAllMocks();
  });

  it('renders the initial configuration form', () => {
    render(<ViralProductWidgetPage />);

    expect(screen.getByText('Viral Product Widget Builder')).toBeDefined();
    expect(screen.getByDisplayValue('Premium Artisan Coffee')).toBeDefined();
    expect(screen.getByDisplayValue('$24.99')).toBeDefined();
    expect(screen.getByDisplayValue('A rich, dark roast sourced from the best beans.')).toBeDefined();
    expect(screen.getByText(/Remove Branding/i)).toBeDefined();
    expect(screen.getByText('Copy Code')).toBeDefined();
    expect(screen.getByTitle('Preview')).toBeDefined();
  });

  it('updates iframe source when input changes', () => {
    render(<ViralProductWidgetPage />);

    const nameInput = screen.getByDisplayValue('Premium Artisan Coffee');
    fireEvent.change(nameInput, { target: { value: 'Super Cool Gadget' } });

    const iframe = screen.getByTitle('Preview') as HTMLIFrameElement;
    expect(iframe.src).toContain(encodeURIComponent('Super Cool Gadget'));
  });

  it('shows soft paywall when clicking remove branding without pro', () => {
    render(<ViralProductWidgetPage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove Branding/i);
    fireEvent.click(removeBrandingCheckbox);

    // The paywall should appear
    expect(screen.getByText('Upgrade to Pro')).toBeDefined();
    expect(screen.getByText(/White-label your embedded product widgets/i)).toBeDefined();
  });

  it('allows removing branding if user has pro', () => {
    // Mock user has pro
    if (typeof window !== 'undefined' && window.localStorage) {
        window.localStorage.setItem('has_pro', 'true');
    }

    render(<ViralProductWidgetPage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove Branding/i);
    fireEvent.click(removeBrandingCheckbox);

    // The paywall should not appear
    expect(screen.queryByText('Upgrade to Pro')).toBeNull();

    // The iframe should update branding param
    const iframe = screen.getByTitle('Preview') as HTMLIFrameElement;
    expect(iframe.src).toContain('branding=false');
  });

  it('closes paywall when keep branding is clicked', () => {
    render(<ViralProductWidgetPage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove Branding/i);
    fireEvent.click(removeBrandingCheckbox);

    expect(screen.getByText('Upgrade to Pro')).toBeDefined();

    const keepBrandingBtn = screen.getByText('Keep Branding');
    fireEvent.click(keepBrandingBtn);

    expect(screen.queryByText('Upgrade to Pro')).toBeNull();
  });
});
