import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import MilestoneSharePage, { generateMetadata } from './page';

describe('MilestoneSharePage', () => {
  it('renders the milestone share landing page with correct CTA', () => {
    render(<MilestoneSharePage searchParams={{ tenant: 'test-tenant', milestone: 'first_sale' }} />);

    // Check if image is rendered with correct src
    const image = screen.getByAltText('Milestone') as HTMLImageElement;
    expect(image.src).toContain('/api/v1/growth/milestone/card?milestone_id=first_sale&tenant=test-tenant');

    // Check if CTA exists and points to correct onboarding link
    const cta = screen.getByText(/Start your own business on OHC/i);
    expect(cta).toBeDefined();
    expect((cta as HTMLAnchorElement).href).toContain('/onboarding?ref=test-tenant');
  });

  it('generateMetadata returns correct OG tags', async () => {
    const metadata = await generateMetadata({
      searchParams: {
        tenant: 'test-tenant',
        milestone: '100_orders',
        title: 'Awesome Title',
        description: 'Awesome Description'
      }
    }, {} as any);

    expect(metadata.title).toBe('Awesome Title');
    expect(metadata.description).toBe('Awesome Description');
    expect(metadata.openGraph?.images?.[0]).toContain('/api/v1/growth/milestone/card?milestone_id=100_orders&tenant=test-tenant');
  });
});
