import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import GiftCardsPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn() }),
}));

describe('GiftCardsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  it('renders the gift card generator page', () => {
    render(<GiftCardsPage />);
    expect(screen.getAllByText('Gift Card Generator 🎁')).toBeDefined();
    expect(screen.getAllByText('Gift Card Value')).toBeDefined();
    expect(screen.getByRole('button', { name: 'Generate Gift Card' })).toBeDefined();
  });

  it('updates the gift card preview when value is changed', () => {
    render(<GiftCardsPage />);

    const valueInput = screen.getByRole("spinbutton") as HTMLInputElement;
    fireEvent.change(valueInput, { target: { value: '150' } });

    expect(screen.getAllByText('$150')).toBeDefined();
  });

  it('includes the Powered by OHC branding in the preview by default', () => {
    render(<GiftCardsPage />);
    const branding = screen.getAllByText(/Powered by OHC/i);
    expect(branding).toBeDefined();
  });

  it('generates a share link when clicking the generate button', () => {
    render(<GiftCardsPage />);

    const generateBtn = screen.getByRole('button', { name: 'Generate Gift Card' });
    fireEvent.click(generateBtn);

    expect(screen.getAllByText('Share Your Gift Card')).toBeDefined();
    const shareLink = screen.getByRole('textbox', { name: /Gift Card Link/i }) as HTMLInputElement;
    expect(shareLink.value).toContain('/gift-card?amount=50');
  });
});
