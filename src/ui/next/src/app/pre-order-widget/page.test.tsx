import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import PreOrderWidgetPage from './page';

describe('PreOrderWidgetPage', () => {
  it('renders the configuration form correctly', () => {
    render(<PreOrderWidgetPage />);
    expect(screen.getByText('Pre-Order Waitlist Engine')).toBeInTheDocument();
    expect(screen.getByText('Product Name')).toBeInTheDocument();
    expect(screen.getByText('Special Offer (Optional)')).toBeInTheDocument();
    expect(screen.getByText('Theme')).toBeInTheDocument();
  });

  it('updates the live preview when form is filled', () => {
    render(<PreOrderWidgetPage />);
    const nameInput = screen.getByPlaceholderText('e.g. The Vegan Chocolate Cake');
    fireEvent.change(nameInput, { target: { value: 'Limited Sneakers' } });

    // Check that live preview reflects the change
    expect(screen.getByText('Limited Sneakers')).toBeInTheDocument();
  });

  it('shows the embed modal when button is clicked', () => {
    render(<PreOrderWidgetPage />);

    // Check that modal is not initially visible
    expect(screen.queryByText('Embed Your Waitlist')).not.toBeInTheDocument();

    const embedButton = screen.getByText('Get Widget Embed Code');
    fireEvent.click(embedButton);

    // Check that modal is visible
    expect(screen.getByText('Embed Your Waitlist')).toBeInTheDocument();
  });
});
