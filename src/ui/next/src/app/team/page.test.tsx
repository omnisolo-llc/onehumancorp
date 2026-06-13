import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import TeamPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

vi.mock('../components/GrowthReferralWidget', () => ({
  default: () => <div data-testid="growth-referral-widget">Growth Referral Widget</div>
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
  TooltipProvider: ({ children }: { children: React.ReactNode }) => <>{children}</>
}));

// Mock fetch for the approvals endpoint
global.fetch = vi.fn();

describe('TeamPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({ pending_approvals: [] }),
    });
  });

  it('renders the team page headers and widget', async () => {
    render(
      <TooltipProvider>
        <TeamPage />
      </TooltipProvider>
    );
    expect(screen.getByText('Your Team')).toBeDefined();
    expect(screen.getByTestId('growth-referral-widget')).toBeDefined();

    // Check if the departments render
    await waitFor(() => {
        expect(screen.getByText('AI Departments')).toBeDefined();
        expect(screen.getByText('The Manager')).toBeDefined();
        expect(screen.getByText('The Promoter')).toBeDefined();
    });
  });
});
