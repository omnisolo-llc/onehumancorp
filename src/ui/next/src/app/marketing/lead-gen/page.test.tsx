import { render, screen } from '@testing-library/react';
import LeadGenCampaignPage from './page';
import { describe, it, expect } from 'vitest';
import { act } from 'react';

describe('LeadGenCampaignPage', () => {
  it('renders the page correctly', () => {
    act(() => { render(<LeadGenCampaignPage />); });
    expect(screen.getByText('Local Lead Generator')).toBeDefined();
  });
});
