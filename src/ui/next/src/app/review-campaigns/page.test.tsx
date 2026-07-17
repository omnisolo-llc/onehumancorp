import { render, screen } from '@testing-library/react';
import ReviewCampaignsPage from './page';
import { describe, it, expect, vi } from 'vitest';

vi.mock('next/navigation', () => {
    return {
        useRouter: () => ({ push: vi.fn() })
    };
});

describe('ReviewCampaignsPage', () => {
  it('renders the page correctly', () => {
    render(<ReviewCampaignsPage />);
    expect(screen.getByText('Automated Review Campaigns ⭐️')).toBeInTheDocument();
  });
});
