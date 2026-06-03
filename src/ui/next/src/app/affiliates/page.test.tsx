import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import AffiliatesHubPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('AffiliatesHubPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the page correctly', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ total_affiliates: 5, total_commission_cents: 1000 })
    } as any);

    render(<AffiliatesHubPage />);

    expect(screen.getByText('Affiliate & Partner Hub')).toBeDefined();

    await waitFor(() => {
      expect(screen.getByText('5')).toBeDefined();
    });
  });

  it('handles generating an affiliate link', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ affiliate_link: 'https://ohc.store/ref/12345678' })
    } as any);

    render(<AffiliatesHubPage />);

    const generateBtn = screen.getByText('Generate Affiliate Link');
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(screen.getByDisplayValue('https://ohc.store/ref/12345678')).toBeDefined();
    });
  });
});
