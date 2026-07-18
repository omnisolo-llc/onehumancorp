import { render, screen } from '@testing-library/react';
import { GrowBusinessCard } from './GrowBusinessCard';

describe('GrowBusinessCard', () => {
  it('renders correctly', () => {
    render(<GrowBusinessCard />);
    expect(screen.getByText('Grow Business')).toBeInTheDocument();
  });

  it('contains correct links', () => {
    render(<GrowBusinessCard />);


    const streakLink = screen.getByRole('link', { name: /Streak Widget/i });
    expect(streakLink).toHaveAttribute('href', '/viral-streak-widget');

    const promoterLink = screen.getByRole('link', { name: /Promoter Agent/i });
    expect(promoterLink).toHaveAttribute('href', '/viral-post-generator');

    const giveawayLink = screen.getByRole('link', { name: /Giveaway/i });
    expect(giveawayLink).toHaveAttribute('href', '/giveaway');

    const groupBuyLink = screen.getByRole('link', { name: /Group Buy/i });
    expect(groupBuyLink).toHaveAttribute('href', '/group-buy-widget');

    const mysteryLink = screen.getByRole('link', { name: /Mystery Discount/i });
    expect(mysteryLink).toHaveAttribute('href', '/mystery-discount-generator');

    const goalTrackerLink = screen.getByRole('link', { name: /Goal Tracker/i });
    expect(goalTrackerLink).toHaveAttribute('href', '/viral-goal-tracker');

    const giveGetLink = screen.getByRole('link', { name: /Give\/Get Widget/i });
    expect(giveGetLink).toHaveAttribute('href', '/viral-give-get-widget');

    const widgetLink = screen.getByRole('link', { name: /Viral Widget/i });
    expect(widgetLink).toHaveAttribute('href', '/viral-powered-by-ohc-widget');

    const bizCardLink = screen.getByRole('link', { name: /Digital Business Card/i });
    expect(bizCardLink).toHaveAttribute('href', '/digital-business-card');

    const setupLink = screen.getByRole('link', { name: /Review Storefront/i });
    expect(setupLink).toHaveAttribute('href', '/edge-storefront-setup');

    const eventRsvpLink = screen.getByRole('link', { name: /Event RSVP/i });
    expect(eventRsvpLink).toHaveAttribute('href', '/event-rsvp-builder');

    const beforeAfterLink = screen.getByRole('link', { name: /Before\/After Slider/i });
    expect(beforeAfterLink).toHaveAttribute('href', '/viral-before-after-slider');

    const challengeLink = screen.getByRole('link', { name: /Challenge Generator/i });
    expect(challengeLink).toHaveAttribute('href', '/viral-challenge-generator');

    const countdownLink = screen.getByRole('link', { name: /Countdown Widget/i });
    expect(countdownLink).toHaveAttribute('href', '/viral-countdown-widget');

    const couponLink = screen.getByRole('link', { name: /Coupon Unlock/i });
    expect(couponLink).toHaveAttribute('href', '/viral-coupon-unlock');

    const jobBoardLink = screen.getByRole('link', { name: /Job Board Generator/i });
    expect(jobBoardLink).toHaveAttribute('href', '/viral-job-board-generator');

    const leaderboardLink = screen.getByRole('link', { name: /Leaderboard/i });
    expect(leaderboardLink).toHaveAttribute('href', '/viral-leaderboard-generator');

    const productLink = screen.getByRole('link', { name: /Product Widget/i });
    expect(productLink).toHaveAttribute('href', '/viral-product-widget');

    const scratchOffLink = screen.getByRole('link', { name: /Scratch-Off/i });
    expect(scratchOffLink).toHaveAttribute('href', '/viral-scratch-off-generator');

    const tierListLink = screen.getByRole('link', { name: /Tier List/i });
    expect(tierListLink).toHaveAttribute('href', '/viral-tier-list-generator');

    const waitlistLink = screen.getByRole('link', { name: /Waitlist/i });
    expect(waitlistLink).toHaveAttribute('href', '/viral-waitlist-generator');

    const trustBadgeLink = screen.getByRole('link', { name: /Trust Badge Builder/i });
    expect(trustBadgeLink).toHaveAttribute('href', '/viral-trust-badge-builder');
  });
});
