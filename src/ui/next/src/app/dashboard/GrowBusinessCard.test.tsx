import { render, screen } from '@testing-library/react';
import { GrowBusinessCard } from './GrowBusinessCard';

describe('GrowBusinessCard', () => {
  it('renders correctly', () => {
    render(<GrowBusinessCard />);
    expect(screen.getByText('Grow Business')).toBeInTheDocument();
  });

  it('contains correct links', () => {
    render(<GrowBusinessCard />);

    const promoterLink = screen.getByRole('link', { name: /Promoter Agent/i });
    expect(promoterLink).toHaveAttribute('href', '/viral-post-generator');

    const giveawayLink = screen.getByRole('link', { name: /Giveaway/i });
    expect(giveawayLink).toHaveAttribute('href', '/giveaway');

    const groupBuyLink = screen.getByRole('link', { name: /Group Buy/i });
    expect(groupBuyLink).toHaveAttribute('href', '/group-buy-widget');


    const goalTrackerLink = screen.getByRole('link', { name: /Goal Tracker/i });
    expect(goalTrackerLink).toHaveAttribute('href', '/viral-goal-tracker');

    const widgetLink = screen.getByRole('link', { name: /Viral Widget/i });
    expect(widgetLink).toHaveAttribute('href', '/viral-powered-by-ohc-widget');

    const bizCardLink = screen.getByRole('link', { name: /Digital Business Card/i });
    expect(bizCardLink).toHaveAttribute('href', '/digital-business-card');

    const setupLink = screen.getByRole('link', { name: /Review Storefront/i });
    expect(setupLink).toHaveAttribute('href', '/edge-storefront-setup');

    const eventRsvpLink = screen.getByRole('link', { name: /Event RSVP/i });
    expect(eventRsvpLink).toHaveAttribute('href', '/event-rsvp-builder');
  });
});
