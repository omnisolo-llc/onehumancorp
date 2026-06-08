import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import PostCheckoutShare from './PostCheckoutShare';
import { vi, describe, it, expect, beforeAll, afterEach } from 'vitest';

describe('PostCheckoutShare', () => {
  const mockReferralLink = 'https://ohc.app/ref/123';
  const mockWriteText = vi.fn();

  beforeAll(() => {
    Object.assign(navigator, {
      clipboard: {
        writeText: mockWriteText,
      },
    });
    window.open = vi.fn();
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly for a normal order', () => {
    render(<PostCheckoutShare referralLink={mockReferralLink} isSubscription={false} />);
    expect(screen.getByText(/Your order is confirmed/i)).toBeDefined();
    expect(screen.getByDisplayValue(mockReferralLink)).toBeDefined();
  });

  it('renders correctly for a subscription', () => {
    render(<PostCheckoutShare referralLink={mockReferralLink} isSubscription={true} />);
    expect(screen.getByText(/You're in! We'll text you a magic link/i)).toBeDefined();
    expect(screen.getByDisplayValue(mockReferralLink)).toBeDefined();
  });

  it('copies the link when the copy button is clicked', () => {
    render(<PostCheckoutShare referralLink={mockReferralLink} isSubscription={false} />);
    const copyButton = screen.getByText('Copy');
    fireEvent.click(copyButton);
    expect(mockWriteText).toHaveBeenCalledWith(mockReferralLink);
    expect(screen.getByText('Copied!')).toBeDefined();
  });

  it('opens WhatsApp share dialog when WhatsApp button is clicked', () => {
    render(<PostCheckoutShare referralLink={mockReferralLink} isSubscription={false} />);
    const whatsappButton = screen.getByText('WhatsApp');
    fireEvent.click(whatsappButton);
    expect(window.open).toHaveBeenCalled();
  });

  it('opens Twitter share dialog when X button is clicked', () => {
    render(<PostCheckoutShare referralLink={mockReferralLink} isSubscription={false} />);
    const xButton = screen.getByText('X (Twitter)');
    fireEvent.click(xButton);
    expect(window.open).toHaveBeenCalled();
  });

  it('opens Facebook share dialog when Facebook button is clicked', () => {
    render(<PostCheckoutShare referralLink={mockReferralLink} isSubscription={false} />);
    const facebookButton = screen.getByText('Share on Facebook');
    fireEvent.click(facebookButton);
    expect(window.open).toHaveBeenCalled();
  });
});
