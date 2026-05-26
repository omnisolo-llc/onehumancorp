import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import TeamPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

global.fetch = vi.fn();

describe('TeamPage End-to-End Approval Flow', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the team page and allows 1-tap approval', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/tooltips') {
        return Promise.resolve({
          json: () => Promise.resolve({ tooltips: [] })
        });
      }
      if (url === '/api/agents/approvals') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({
            pending_approvals: [
              {
                id: 'req-1',
                tenant_id: 'tenant-1',
                department: 'customer_success',
                description: 'Draft reply | Payload: {"original_message":"Hi","generated_response":"Hello!"}',
                status: 'PENDING',
                action_risk: 'HIGH'
              }
            ]
          })
        });
      }
      if (url === '/api/agents/approvals/req-1') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ success: true })
        });
      }
      return Promise.resolve({ ok: false });
    });

    render(<TooltipProvider><TeamPage /></TooltipProvider>);

    // Wait for the department to show up with 1 item awaiting approval
    await waitFor(() => {
      expect(screen.getByText('The Ambassador')).toBeInTheDocument();
      expect(screen.getByText('1 item awaiting approval')).toBeInTheDocument();
    });

    // Click on the department card
    fireEvent.click(screen.getByText('The Ambassador'));

    // Wait for the approval inbox to render
    await waitFor(() => {
      expect(screen.getByText('Approval Inbox')).toBeInTheDocument();
    });

    // Verify the request is displayed
    expect(screen.getByText(/Draft reply/)).toBeInTheDocument();

    // Click the approve button
    const approveButton = screen.getByRole('button', { name: 'Approve' });
    fireEvent.click(approveButton);

    // Verify that fetch was called to approve
    expect(global.fetch).toHaveBeenCalledWith('/api/agents/approvals/req-1', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ approved: true })
    });

    // Verify optimistic update (the request should disappear)
    await waitFor(() => {
      expect(screen.queryByText(/Draft reply/)).not.toBeInTheDocument();
    });
  });
});
