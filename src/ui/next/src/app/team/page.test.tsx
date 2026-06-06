import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import TeamPage from './page';

// Mock the components
vi.mock('./components/DepartmentCard', () => {
  return {
    default: ({ name, pendingCount, onClick }: any) => {
      // name is "The Manager", "The Promoter", etc.
      return (
        <div data-testid={`dept-card-${name}`} onClick={onClick}>
          {name}
        </div>
      );
    }
  };
});

vi.mock('./components/ApprovalInbox', () => {
  return {
    default: ({ departmentName, onBack }: any) => (
      <div data-testid="approval-inbox">
        <h1>Inbox for {departmentName}</h1>
        <button onClick={onBack}>Back</button>
      </div>
    )
  };
});

// Mock fetch globally
global.fetch = vi.fn(() =>
  Promise.resolve({
    json: () => Promise.resolve([]),
  })
) as any;

describe('TeamPage', () => {
  it('shows the SEO metric card when The Promoter (marketing) is selected', async () => {
    render(<TeamPage />);

    // Wait for loading to finish
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    const promoterCard = screen.getByTestId('dept-card-The Promoter');
    fireEvent.click(promoterCard);

    expect(screen.getByText('Storefront SEO & Speed')).toBeInTheDocument();
    expect(screen.getByText('0.8s')).toBeInTheDocument();
  });
});
