import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import '@testing-library/jest-dom/vitest';
import { act } from 'react';
import PreOrderWidgetPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));


describe('PreOrderWidgetPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('shows soft paywall when checkbox is checked without pro and includes viral loop option', () => {
    act(() => { render(<PreOrderWidgetPage />); });

    // Click checkbox
    const checkbox = screen.getByLabelText('Remove "Powered by OHC" branding');
    fireEvent.click(checkbox);

    // Check if soft paywall shows up
    expect(screen.getAllByText('Upgrade to Pro')).toBeDefined();

    // Check if viral loop option is present
    expect(screen.getByText('Share on X to Unlock')).toBeDefined();

    // Checkbox should be un-checked
    expect((checkbox as HTMLInputElement).checked).toBe(false);
  });

  it('updates embed code and preview based on branding toggle', () => {
    act(() => { render(<PreOrderWidgetPage />); });

    // Initially, branding should be present
    expect(screen.getByText('⚡ Powered by OHC')).toBeDefined();

    // Open Modal
    fireEvent.click(screen.getByText('Get Widget Embed Code'));

    // Check embed code in modal
    let embedContainer = screen.getByText(/<script src="https:\/\/assets\.onehumancorp\.com\/widgets\/pre-order\.js" async><\/script>/);
    expect(embedContainer.parentElement?.textContent).toContain('Powered by OHC');
  });

  it('renders the configuration form correctly', () => {
    act(() => { render(<PreOrderWidgetPage />); });
    expect(screen.getByText('Pre-Order Waitlist Engine')).toBeInTheDocument();
    expect(screen.getByText('Product Name')).toBeInTheDocument();
    expect(screen.getByText('Special Offer (Optional)')).toBeInTheDocument();
    expect(screen.getByText('Theme')).toBeInTheDocument();
  });

  it('updates the live preview when form is filled', () => {
    act(() => { render(<PreOrderWidgetPage />); });
    const nameInput = screen.getByPlaceholderText('e.g. The Vegan Chocolate Cake');
    fireEvent.change(nameInput, { target: { value: 'Limited Sneakers' } });

    // Check that live preview reflects the change
    expect(screen.getByText('Limited Sneakers')).toBeInTheDocument();
  });

  it('shows the embed modal when button is clicked', () => {
    act(() => { render(<PreOrderWidgetPage />); });

    // Check that modal is not initially visible
    expect(screen.queryByText('Embed Your Waitlist')).not.toBeInTheDocument();

    const embedButton = screen.getByText('Get Widget Embed Code');
    fireEvent.click(embedButton);

    // Check that modal is visible
    expect(screen.getByText('Embed Your Waitlist')).toBeInTheDocument();
  });
});
