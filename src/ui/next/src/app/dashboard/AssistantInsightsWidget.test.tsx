import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { AssistantInsightsWidget } from './AssistantInsightsWidget';

// Mock fetch
const originalFetch = global.fetch;

beforeEach(() => {
  global.fetch = vi.fn();
});

afterEach(() => {
  global.fetch = originalFetch;
  vi.resetAllMocks();
});

describe('AssistantInsightsWidget', () => {
  it('renders loading state initially', () => {
    (global.fetch as any).mockImplementation(() => new Promise(() => {})); // Never resolves to keep loading

    render(<AssistantInsightsWidget tenant="test-tenant" />);

    // We can just verify it doesn't crash and maybe look for the pulse class if we need to,
    // but React Testing Library is better with text.
    // Wait for the pulse div to be in document.
    const loadingDiv = document.querySelector('.animate-pulse');
    expect(loadingDiv).toBeInTheDocument();
  });

  it('renders nothing when there are no insights', async () => {
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ([]), // Empty items
    });

    const { container } = render(<AssistantInsightsWidget tenant="test-tenant" />);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/v1/ui/triage?tenant_id=test-tenant');
    });

    // Should render null
    expect(container.firstChild).toBeNull();
  });

  it('renders insights and handles approve click', async () => {
    const mockTriageItems = [
      {
        id: 'item-1',
        tenant_id: 'test-tenant',
        source: 'Decision Assistant',
        priority: 'High',
        context: 'Follow up on abandoned cart for Priya',
        status: 'pending',
        created_at: '2023-01-01T00:00:00Z',
      },
      {
        id: 'item-2',
        tenant_id: 'test-tenant',
        source: 'Customer Success Agent',
        context: 'Draft quote for Carlos',
        status: 'pending',
        created_at: '2023-01-01T01:00:00Z',
      }
    ];

    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('/api/v1/ui/triage?')) {
        return Promise.resolve({
          ok: true,
          json: async () => mockTriageItems,
        });
      }
      if (url.includes('/api/v1/ui/triage/action')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ success: true }),
        });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<AssistantInsightsWidget tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Assistant Insights')).toBeInTheDocument();
      expect(screen.getByText('Follow up on abandoned cart for Priya')).toBeInTheDocument();
      expect(screen.getByText('Draft quote for Carlos')).toBeInTheDocument();
    });

    const user = userEvent.setup();
    const approveBtn1 = screen.getByTestId('approve-action-item-1');

    await user.click(approveBtn1);

    await waitFor(() => {
      // Expect fetch to be called with POST and correct payload
      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/v1/ui/triage/action'),
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ triage_item_id: 'item-1', approved: true })
        })
      );
    });

    // Expect item 1 to be removed from the UI
    await waitFor(() => {
      expect(screen.queryByText('Follow up on abandoned cart for Priya')).not.toBeInTheDocument();
      expect(screen.getByText('Draft quote for Carlos')).toBeInTheDocument(); // Item 2 should still be there
    });
  });

  it('handles API errors gracefully', async () => {
    (global.fetch as any).mockRejectedValue(new Error('Network error'));

    // Silence console.error for this expected failure
    const originalError = console.error;
    console.error = vi.fn();

    const { container } = render(<AssistantInsightsWidget tenant="test-tenant" />);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalled();
    });

    // Should render null on error (since actions will be empty)
    expect(container.firstChild).toBeNull();

    console.error = originalError;
  });
});