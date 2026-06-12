import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import ViralCouponGeneratorPage from './page';
import { vi } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter() {
    return {
      push: vi.fn(),
      replace: vi.fn(),
      prefetch: vi.fn(),
    };
  },
}));

describe('ViralCouponGeneratorPage', () => {
  beforeEach(() => {
    window.localStorage.clear();
  });

  it('renders the viral coupon generator form', () => {
    render(<ViralCouponGeneratorPage />);
    expect(screen.getByRole('heading', { name: /Viral Coupon Generator/i })).toBeInTheDocument();
  });

  it('shows soft paywall when generating without pro', () => {
    render(<ViralCouponGeneratorPage />);
    fireEvent.change(screen.getByLabelText(/Offer Title/i), { target: { value: 'Test Offer' } });
    fireEvent.click(screen.getByRole('button', { name: /Generate Embed Code/i }));
    expect(screen.getByRole('heading', { name: /Upgrade to Pro/i })).toBeInTheDocument();
  });

  it('generates embed code when pro is active', () => {
    window.localStorage.setItem('has_pro', 'true');
    render(<ViralCouponGeneratorPage />);
    fireEvent.change(screen.getByLabelText(/Offer Title/i), { target: { value: 'Test Offer' } });
    fireEvent.click(screen.getByRole('button', { name: /Generate Embed Code/i }));
    expect(screen.getByText(/Code Ready!/i)).toBeInTheDocument();

    // We get the input area directly instead of using the aria-name because it's a readonly code block
    const textareas = screen.getAllByRole('textbox');
    // Ensure the textarea has the encoded title in its value
    expect(textareas[textareas.length - 1].value).toContain('Test%20Offer');
  });
});
