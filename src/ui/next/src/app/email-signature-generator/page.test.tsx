import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import EmailSignatureGeneratorPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('EmailSignatureGeneratorPage', () => {
  it('exposes color choice and keeps a repeated selection idempotent', () => {
    render(<EmailSignatureGeneratorPage />);
    const blue=screen.getByRole('button',{name:'Select color #0066FF'}); const green=screen.getByRole('button',{name:'Select color #34C759'});
    expect(blue).toHaveAttribute('aria-pressed','true'); expect(green).toHaveAttribute('aria-pressed','false');
    fireEvent.click(green);
    expect(blue).toHaveAttribute('aria-pressed','false'); expect(green).toHaveAttribute('aria-pressed','true');
    fireEvent.click(green); expect(green).toHaveAttribute('aria-pressed','true');
  });

  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  it('renders the email signature generator page correctly', () => {
    render(<EmailSignatureGeneratorPage />);
    expect(screen.getByText('Free Email Signature Generator')).toBeDefined();
    expect(screen.getByText('Your Details')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
    expect(screen.getByText('Remove "OmniSolo" branding')).toBeDefined();
    expect(screen.getByRole('checkbox', { name: 'Remove "OmniSolo" branding' })).toBeDefined();
  });

  it('shows soft paywall when clicking remove branding checkbox', () => {
    render(<EmailSignatureGeneratorPage />);
    const checkbox = screen.getByRole('checkbox', { name: 'Remove "OmniSolo" branding' });
    fireEvent.click(checkbox);
    expect(screen.getByRole('heading', { name: 'Upgrade to Pro' })).toBeDefined();
  });

  it('updates live preview when fields are edited', () => {
    render(<EmailSignatureGeneratorPage />);
    const nameInput = screen.getByPlaceholderText('e.g. Jane Doe');
    fireEvent.change(nameInput, { target: { value: 'Alice Wonderland' } });
    expect(screen.getByText('Alice Wonderland')).toBeDefined();
  });
});
