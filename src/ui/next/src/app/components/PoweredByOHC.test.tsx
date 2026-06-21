/**
 * @vitest-environment jsdom
 */

import React from 'react';
import { render, screen, fireEvent, cleanup } from '@testing-library/react';
import { PoweredByOHC } from './PoweredByOHC';
import { describe, it, expect, afterEach } from 'vitest';

describe('PoweredByOHC Component', () => {
  afterEach(() => {
    cleanup();
  });
  const testTenantId = 'test-org-123';

  it('renders the base button correctly', () => {
    render(<PoweredByOHC tenantId={testTenantId} />);

    // The main base link should be rendered
    const linkElements = screen.getAllByRole('link');
    // Find the one that has the text Powered by OHC
    const baseLink = linkElements.find(el => el.textContent?.includes('Powered by OHC'));

    expect(baseLink).toBeDefined();
    expect(baseLink?.getAttribute('href')).toBe(`/onboarding?ref=${testTenantId}&source=footer_widget`);
  });

  it('shows the popover when hovered', async () => {
    render(<PoweredByOHC tenantId={testTenantId} />);

    // The popover content shouldn't be there initially
    expect(screen.queryByText(/Built with OneHumanCorp/i)).toBeNull();

    // Find the container wrapper and trigger hover
    const wrapper = screen.getAllByRole('link')[0].parentElement;
    if (!wrapper) throw new Error("Wrapper not found");

    fireEvent.mouseEnter(wrapper);

    // Wait for the popover content to appear
    expect(screen.getByText(/Built with OneHumanCorp/i)).toBeDefined();

    // There should now be two links to the onboarding URL (one in base, one in popover CTA)
    const links = screen.getAllByRole('link');
    expect(links.length).toBe(2);
    expect(links[0].getAttribute('href')).toBe(`/onboarding?ref=${testTenantId}&source=footer_widget`);
    expect(links[1].getAttribute('href')).toBe(`/onboarding?ref=${testTenantId}&source=footer_widget`);

    // CTA button text should be present
    expect(screen.getByText(/Create Your Own/i)).toBeDefined();
  });
});
