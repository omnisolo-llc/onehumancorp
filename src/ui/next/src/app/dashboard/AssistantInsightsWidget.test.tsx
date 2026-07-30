import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { AssistantInsightsWidget } from './AssistantInsightsWidget';
import '@testing-library/jest-dom';

// Mock fetch
global.fetch = vi.fn();

describe('AssistantInsightsWidget', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders loading state initially', () => {
    // Unresolved promise to keep it in loading state
    (global.fetch as any).mockImplementationOnce(() => new Promise(() => {}));

    render(<AssistantInsightsWidget />);
    // The loading skeleton doesn't have text, but we can verify it renders by checking the DOM structure if needed.
    // Assuming the component renders successfully without throwing.
    expect(document.querySelector('.animate-pulse')).toBeInTheDocument();
  });

  it('renders empty state when no insights are returned', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ insights: [] }),
    });

    render(<AssistantInsightsWidget />);

    await waitFor(() => {
      expect(screen.getByText('All caught up!')).toBeInTheDocument();
    });
  });

  it('renders insights and allows taking action', async () => {
    const mockInsights = [
      {
        id: "action-1",
        title: "Draft quote for Carlos",
        description: "Carlos requested a quote.",
        actionLabel: "Approve & Send",
        urgency: "high"
      },
      {
        id: "action-2",
        title: "Follow up on abandoned cart",
        description: "Priya left items in cart.",
        actionLabel: "Send Reminder",
        urgency: "medium"
      }
    ];

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ insights: mockInsights }),
    });

    render(<AssistantInsightsWidget />);

    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Draft quote for Carlos')).toBeInTheDocument();
    });

    // Check second item
    expect(screen.getByText('Follow up on abandoned cart')).toBeInTheDocument();

    // Verify badges
    expect(screen.getByText('2 Actions')).toBeInTheDocument();

    // Click action button for the first insight
    const actionButton = screen.getByText('Approve & Send');
    fireEvent.click(actionButton);

    // Verify it was removed
    await waitFor(() => {
      expect(screen.queryByText('Draft quote for Carlos')).not.toBeInTheDocument();
    });

    // Counter should update to 1 Action
    expect(screen.getByText('1 Actions')).toBeInTheDocument();
  });
});
