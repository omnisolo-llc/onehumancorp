import { render, screen } from '@testing-library/react';
import LeadGenCampaignPage from './page';
import { describe, it, expect } from 'vitest';

describe('LeadGenCampaignPage', () => {
  it('renders the page correctly', () => {
    render(<LeadGenCampaignPage />);
    expect(screen.getByText('Local Lead Generator')).toBeInTheDocument();
  });
});
