import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import TestimonialEmbedPage from './page';

// Mock Next.js useSearchParams
const mockGet = vi.fn();
vi.mock('next/navigation', () => ({
  useSearchParams: () => ({
    get: mockGet,
  }),
}));

describe('TestimonialEmbedPage', () => {
  it('renders correctly', () => {
    mockGet.mockReturnValue('test-tenant');
    render(<TestimonialEmbedPage />);

    expect(screen.getByText('Leave a Review')).toBeDefined();
    expect(screen.getByText('We value your feedback.')).toBeDefined();
    expect(screen.getByLabelText('Your Name')).toBeDefined();
    expect(screen.getByLabelText('Review')).toBeDefined();

    const poweredByLink = screen.getByText('Powered by OHC');
    expect(poweredByLink.closest('a')?.href).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=test-tenant');
  });

  it('shows success message on form submit', () => {
    mockGet.mockReturnValue('test-tenant');
    render(<TestimonialEmbedPage />);

    const nameInput = screen.getByLabelText('Your Name');
    const reviewInput = screen.getByLabelText('Review');

    fireEvent.change(nameInput, { target: { value: 'John Doe' } });
    fireEvent.change(reviewInput, { target: { value: 'Great service' } });

    const submitButton = screen.getByText('Submit Review');
    fireEvent.click(submitButton);

    // We also need to submit the form actually to trigger the onSubmit
    // Actually, button type="submit" inside a form triggers onSubmit in testing-library if we fire submit on the form
    // Let's fire submit on the form itself to be safe
    const form = nameInput.closest('form');
    if (form) {
        fireEvent.submit(form);
    }

    expect(screen.getByText('Thank you!')).toBeDefined();
    expect(screen.getByText('Your review has been submitted successfully.')).toBeDefined();

    // Viral loop should still be present
    const poweredByLink = screen.getByText('Powered by OHC');
    expect(poweredByLink.closest('a')?.href).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=test-tenant');
  });
});
