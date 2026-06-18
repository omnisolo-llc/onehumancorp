import React from 'react';
import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import TestimonialCollectorPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

// Mock Next.js router
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
  usePathname: () => '/testimonial-collector',
}));

describe('TestimonialCollectorPage', () => {
  let originalWindowLocation: any;
  beforeEach(() => {
    vi.clearAllMocks();

    // Mock navigator.clipboard
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockImplementation(() => Promise.resolve()),
      },
    });

    // Mock window.open
    window.open = vi.fn();

    // Mock window.location
    originalWindowLocation = window.location;
    delete (window as any).location;
    window.location = {
      ...originalWindowLocation,
      origin: 'https://ohc.app',
    } as Location;

    // Setup localStorage
    const store: Record<string, string> = { 'tenant_id': 'test-tenant' };
    const mockStorage = {
      getItem: vi.fn((key) => store[key] || null),
      setItem: vi.fn((key, value) => { store[key] = value.toString(); }),
      removeItem: vi.fn((key) => { delete store[key]; }),
      clear: vi.fn(() => { for (const key in store) delete store[key]; }),
      length: 1,
      key: vi.fn((idx) => Object.keys(store)[idx] || null),
    };
    Object.defineProperty(window, 'localStorage', {
      value: mockStorage,
      writable: true,
    });
  });

  afterEach(() => {
    window.location = originalWindowLocation;
  });

  const renderWithProviders = (component: React.ReactNode) => {
    return render(
      <TooltipProvider>
        {component}
      </TooltipProvider>
    );
  };

  it('renders correctly and generates links based on tenantId', async () => {
    renderWithProviders(<TestimonialCollectorPage />);

    expect(screen.getByText('Testimonial Collector')).toBeDefined();
    expect(screen.getByText('Gather reviews and earn referrals.')).toBeDefined();
    expect(screen.getByText('Your Collection Link')).toBeDefined();

    const input = await screen.findByDisplayValue('https://ohc.app/embed/testimonial?tenant=test-tenant');
    expect(input).toBeDefined();

    const poweredByLink = screen.getByText('Powered by OHC');
    expect(poweredByLink.closest('a')?.href).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=test-tenant');
  });

  it('copies the collection link to clipboard', async () => {
    renderWithProviders(<TestimonialCollectorPage />);

    const copyButton = await screen.findByText('Copy');
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith('https://ohc.app/embed/testimonial?tenant=test-tenant');
    expect(screen.getByText('Copied!')).toBeDefined();
  });

  it('opens WhatsApp with the correct message', async () => {
    renderWithProviders(<TestimonialCollectorPage />);

    const whatsappBtn = await screen.findByText('Send via WhatsApp');
    fireEvent.click(whatsappBtn);

    const expectedUrl = `https://wa.me/?text=${encodeURIComponent("Hi! Thanks for choosing us. We'd love it if you could leave a quick testimonial here: https://ohc.app/embed/testimonial?tenant=test-tenant")}`;
    expect(window.open).toHaveBeenCalledWith(expectedUrl, '_blank');
  });

  it('opens Twitter with the correct message', async () => {
    renderWithProviders(<TestimonialCollectorPage />);

    const twitterBtn = await screen.findByText('Share on X (Twitter)');
    fireEvent.click(twitterBtn);

    const expectedUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent("Help us grow! If you enjoyed our service, please leave a quick review: https://ohc.app/embed/testimonial?tenant=test-tenant \n\n⚡ Powered by OHC")}`;
    expect(window.open).toHaveBeenCalledWith(expectedUrl, '_blank');
  });
});
