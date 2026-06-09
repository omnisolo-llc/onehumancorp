import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import OfferWidgetPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('OfferWidgetPage Growth Loops', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly and generates draft with viral loop', () => {
    render(<OfferWidgetPage />);

    expect(screen.getByText('Embeddable Offer Widget 🎁')).toBeDefined();

    // Check if the title input works
    const titleInput = screen.getByDisplayValue('Special Offer');
    fireEvent.change(titleInput, { target: { value: 'New Test Offer' } });

    // Check if live preview updates
    // Since live preview uses an iframe now, we verify the iframe src updates
    const iframe = document.querySelector('iframe');
    expect(iframe?.src).toContain('title=New%20Test%20Offer');

    // The "Powered by OHC" footer should be visible by default
    // The iframe should NOT have branding=false by default
    expect(iframe?.src).not.toContain('branding=false');

    // Generate code
    const generateBtn = screen.getByText('Get Embed Code');
    fireEvent.click(generateBtn);

    // Verify textarea contains the embed code with viral loop
    const textareas = screen.getAllByRole('textbox') as HTMLTextAreaElement[];
    const textarea = textareas.find(ta => ta.value.includes('<iframe'))!;
    expect(textarea).toBeDefined();
    expect(textarea.value).toContain('btn=Claim%20Offer');
    expect(textarea.value).toContain('title=New%20Test%20Offer');
  });

  it('removes branding when checkbox is checked', () => {
    render(<OfferWidgetPage />);

    // Click checkbox to remove branding
    const checkbox = screen.getByLabelText('Remove "Powered by OHC" branding');
    fireEvent.click(checkbox);

    // branding=false should be added to iframe src
    const iframe = document.querySelector('iframe');
    expect(iframe?.src).toContain('branding=false');
  });
});
