import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { SubscriptionUpsellWidget } from './SubscriptionUpsellWidget';
import React, { useState } from 'react';

const TestWrapper = () => {
    const [isSubscription, setIsSubscription] = useState(false);
    return <SubscriptionUpsellWidget isSubscription={isSubscription} setIsSubscription={setIsSubscription} />;
};

describe('SubscriptionUpsellWidget', () => {
  it('renders correctly', () => {
    render(<TestWrapper />);
    expect(screen.getByText('Subscribe & Save 10%')).toBeDefined();
  });

  it('toggles subscription state when clicked', () => {
    render(<TestWrapper />);
    const checkbox = screen.getByRole('checkbox', { hidden: true });

    // Initially unchecked
    expect(checkbox).not.toHaveProperty('checked', true);

    // Click label to toggle
    fireEvent.click(screen.getByText('Subscribe & Save 10%'));

    // Should be checked
    expect(checkbox).toHaveProperty('checked', true);
  });
});