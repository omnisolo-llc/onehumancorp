import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { PostPurchaseShareWidget } from './PostPurchaseShareWidget';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import '@testing-library/jest-dom';

describe('PostPurchaseShareWidget', () => {
  beforeEach(() => {
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockImplementation(() => Promise.resolve()),
      },
    });
    vi.spyOn(window, 'open').mockImplementation(() => null);
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly with default props', () => {
    render(<PostPurchaseShareWidget tenantId="test-tenant" />);
    expect(screen.getByText('Share & Save')).toBeInTheDocument();
    expect(screen.getByText('🎁 Give 10%, Get 10%')).toBeInTheDocument();
  });

  it('generates the correct referral link', () => {
    render(<PostPurchaseShareWidget tenantId="test-tenant" orderId="123" />);
    const input = screen.getByRole('textbox') as HTMLInputElement;
    expect(input.value).toBe('https://ohc.app/shop/test-tenant?ref=post_purchase_123');
  });

  it('copies link to clipboard', async () => {
    render(<PostPurchaseShareWidget tenantId="test-tenant" />);
    const copyButton = screen.getByText('Copy');

    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith('https://ohc.app/shop/test-tenant?ref=post_purchase_default');
    expect(screen.getByText('Copied!')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Copy')).toBeInTheDocument();
    }, { timeout: 2100 });
  });

  it('shares to WhatsApp', () => {
    render(<PostPurchaseShareWidget tenantId="test-tenant" storeName="Cool Store" />);
    const whatsappButton = screen.getByText('Share on WhatsApp');
    fireEvent.click(whatsappButton);

    expect(window.open).toHaveBeenCalled();
    const urlArg = (window.open as any).mock.calls[0][0];
    expect(urlArg).toContain('wa.me');
    expect(urlArg).toContain('Cool%20Store');
  });

  it('shares to Twitter/X with Powered by OHC branding', () => {
    render(<PostPurchaseShareWidget tenantId="test-tenant" storeName="Cool Store" />);
    const twitterButton = screen.getByText('Share on X (Twitter)');
    fireEvent.click(twitterButton);

    expect(window.open).toHaveBeenCalled();
    const urlArg = (window.open as any).mock.calls[0][0];
    expect(urlArg).toContain('twitter.com');
    expect(urlArg).toContain('Cool%20Store');
    expect(urlArg).toContain('Powered%20by%20OHC'); // Verifying loop branding
  });
});
