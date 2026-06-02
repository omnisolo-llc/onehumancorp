import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import MarketDynamicsReport from './page';

// Mock Next.js Link component to avoid router issues in tests
vi.mock('next/link', () => {
  return {
    __esModule: true,
    default: ({ children, href, className }: any) => {
      return (
        <a href={href} className={className}>
          {children}
        </a>
      );
    },
  };
});

describe('MarketDynamicsReport', () => {
  it('renders the header correctly', () => {
    render(<MarketDynamicsReport />);

    // Check main title
    expect(screen.getByText('OHC Market Dynamics & Competitor Deep-Dive')).toBeInTheDocument();

    // Check subtitle
    expect(screen.getByText(/Comprehensive research detailing the SMB platform market/)).toBeInTheDocument();

    // Check navigation link
    expect(screen.getByText('Dashboard')).toBeInTheDocument();
    expect(screen.getByText('Dashboard').closest('a')).toHaveAttribute('href', '/dashboard');
  });

  it('renders all tab buttons', () => {
    render(<MarketDynamicsReport />);

    expect(screen.getByRole('button', { name: /Executive Summary/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Market Mapping/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Gap Analysis/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Agentic Solutions/i })).toBeInTheDocument();
  });

  it('displays the Executive Summary tab content by default', () => {
    render(<MarketDynamicsReport />);

    // There are multiple "Executive Summary" texts (tab and section header), so we use getAllByText
    expect(screen.getAllByText('Executive Summary').length).toBeGreaterThan(0);
    expect(screen.getByText('Invisible AI')).toBeInTheDocument();
    expect(screen.getByText('< 10 Mins')).toBeInTheDocument();
    expect(screen.getByText('Mobile-First')).toBeInTheDocument();
    expect(screen.getByText('Actionable Recommendations for Engineering Swarm')).toBeInTheDocument();
  });

  it('switches to Market Mapping tab correctly', () => {
    render(<MarketDynamicsReport />);

    const mappingButton = screen.getByRole('button', { name: /Market Mapping/i });
    fireEvent.click(mappingButton);

    expect(screen.getByText('Top 10 Traditional Platforms')).toBeInTheDocument();
    expect(screen.getByText('Shopify')).toBeInTheDocument();
    expect(screen.getByText('Top 10 AI-Native Tools')).toBeInTheDocument();
    expect(screen.getByText('Dora AI')).toBeInTheDocument();
    expect(screen.getByText('Deep-Dive Audit: Shopify')).toBeInTheDocument();
  });

  it('switches to Gap Analysis tab correctly', () => {
    render(<MarketDynamicsReport />);

    const gapButton = screen.getByRole('button', { name: /Gap Analysis/i });
    fireEvent.click(gapButton);

    expect(screen.getByText('Gap Matrix: Shopify vs. OHC')).toBeInTheDocument();
    expect(screen.getByText('Unresolved Market Pain Points')).toBeInTheDocument();
    expect(screen.getByText('Omnichannel Sync Nightmare')).toBeInTheDocument();
  });

  it('switches to Agentic Solutions tab correctly', () => {
    render(<MarketDynamicsReport />);

    const solutionsButton = screen.getByRole('button', { name: /Agentic Solutions/i });
    fireEvent.click(solutionsButton);

    expect(screen.getByText('Agentic Solutions for Market Gaps')).toBeInTheDocument();
    expect(screen.getByText('Invisible Local Delivery & Inventory Mesh')).toBeInTheDocument();
    expect(screen.getByText('Omnichannel AI Inbox (The Ambassador)')).toBeInTheDocument();
    expect(screen.getByText('Plain-Language Daily Briefing')).toBeInTheDocument();
  });

  it('triggers window.print when Export PDF is clicked', () => {
    const printSpy = vi.spyOn(window, 'print').mockImplementation(() => {});

    render(<MarketDynamicsReport />);

    const printButton = screen.getByRole('button', { name: /Export PDF/i });
    fireEvent.click(printButton);

    expect(printSpy).toHaveBeenCalled();
    printSpy.mockRestore();
  });
});
