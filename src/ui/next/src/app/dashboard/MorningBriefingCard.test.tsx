/** @jsxImportSource react */
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { MorningBriefingCard } from './MorningBriefingCard';
import userEvent from '@testing-library/user-event';
import React from 'react';

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: vi.fn(({ children }: { children: React.ReactNode }) => children)
}));

describe('MorningBriefingCard', () => {
  beforeEach(() => {
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders loading state initially and then fetches data successfully', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ briefing: 'Test briefing' })
        });
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([
            { id: '1', source: 'Decision Assistant', context: 'Test triage item', action_type: 'Test Action' }
          ])
        });
      }
      return Promise.reject(new Error('not found'));
    });

    const { container } = render(<MorningBriefingCard tenant="test-tenant" />);

    expect(screen.getByText('Morning Briefing')).toBeTruthy();

    await waitFor(() => {
      expect(screen.getByText('Test briefing')).toBeTruthy();
      expect(screen.getByText('Test triage item')).toBeTruthy();
    });

    // Visual styles check
    const wrapper = container.firstChild as HTMLElement;
    expect(wrapper).toHaveClass('backdrop-blur-[30px]');
    expect(wrapper).toHaveClass('backdrop-saturate-[2.1]');
    expect(wrapper).toHaveClass('bg-white/65');
  });

  it('handles fetch failure for briefing gracefully', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: false });
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Unable to load Morning Briefing.')).toBeTruthy();
    });
  });

  it('handles empty briefing gracefully', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Good morning. No new insights at this time.')).toBeTruthy();
    });
  });

  it('handles fetch exception for briefing gracefully', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.reject(new Error('network error'));
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Unable to load Morning Briefing.')).toBeTruthy();
    });
  });

  it('handles triage item actions correctly', async () => {
    const mockTriageItems = [
      { id: '1', source: 'Decision Assistant', context: 'Approve Item', action_type: 'Approve' },
      { id: '2', source: 'Decision Assistant', context: 'Dismiss Item', action_type: 'Dismiss' }
    ];

    (global.fetch as any).mockImplementation((url: string, options: any) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/triage/action')) {
        return Promise.resolve({ ok: true });
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve(mockTriageItems) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Approve Item')).toBeTruthy();
      expect(screen.getByText('Dismiss Item')).toBeTruthy();
    });

    const user = userEvent.setup();
    const approveBtn = screen.getByTestId('action-card-approve-1');
    await user.click(approveBtn);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/ui/triage/action'),
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ triage_item_id: '1', approved: true })
        })
      );
      expect(screen.queryByText('Approve Item')).not.toBeTruthy();
    });

    const dismissBtn = screen.getByTestId('action-card-dismiss-2');
    await user.click(dismissBtn);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/ui/triage/action'),
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ triage_item_id: '2', approved: false })
        })
      );
      expect(screen.queryByText('Dismiss Item')).not.toBeTruthy();
    });
  });

  it('handles triage item actions failure gracefully', async () => {
    const mockTriageItems = [
      { id: '1', source: 'Decision Assistant', context: 'Approve Item', action_type: 'Approve' }
    ];

    (global.fetch as any).mockImplementation((url: string, options: any) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/triage/action')) {
        return Promise.resolve({ ok: false }); // Failure
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve(mockTriageItems) });
      }
      return Promise.reject(new Error('not found'));
    });

    const consoleErrorMock = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Approve Item')).toBeTruthy();
    });

    const user = userEvent.setup();
    const approveBtn = screen.getByTestId('action-card-approve-1');
    await user.click(approveBtn);

    await waitFor(() => {
      expect(consoleErrorMock).toHaveBeenCalled();
      // Item shouldn't be removed
      expect(screen.getByText('Approve Item')).toBeTruthy();
    });

    consoleErrorMock.mockRestore();
  });

  it('handles insight chat correctly', async () => {
    (global.fetch as any).mockImplementation((url: string, options: any) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/dashboard/analytics/chat')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ reply: 'Test chat reply' }) });
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Briefing')).toBeTruthy();
    });

    const user = userEvent.setup();
    const input = screen.getByTestId('insight-chat-input');
    const submitBtn = screen.getByTestId('insight-chat-submit');

    await user.type(input, 'Hello agent');

    // Test the button click
    await user.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByText('Hello agent')).toBeTruthy();
      expect(screen.getByText('Test chat reply')).toBeTruthy();
      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/ui/dashboard/analytics/chat'),
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ message: 'Hello agent' })
        })
      );
    });
  });

  it('handles insight chat failures gracefully', async () => {
    (global.fetch as any).mockImplementation((url: string, options: any) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/dashboard/analytics/chat')) {
        return Promise.resolve({ ok: false }); // Non-ok response
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Briefing')).toBeTruthy();
    });

    const user = userEvent.setup();
    const input = screen.getByTestId('insight-chat-input');
    const submitBtn = screen.getByTestId('insight-chat-submit');

    await user.type(input, 'Hello agent error');
    await user.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByText('Hello agent error')).toBeTruthy();
      expect(screen.getByText('I encountered an error retrieving that information.')).toBeTruthy();
    });
  });

  it('handles insight chat exception gracefully', async () => {
    (global.fetch as any).mockImplementation((url: string, options: any) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/dashboard/analytics/chat')) {
        return Promise.reject(new Error('Network error'));
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Briefing')).toBeTruthy();
    });

    const user = userEvent.setup();
    const input = screen.getByTestId('insight-chat-input');
    const submitBtn = screen.getByTestId('insight-chat-submit');

    await user.type(input, 'Hello agent exception');
    await user.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByText('Hello agent exception')).toBeTruthy();
      expect(screen.getByText('I encountered an error retrieving that information.')).toBeTruthy();
    });
  });

  it('does not send empty chat message', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Briefing')).toBeTruthy();
    });

    const submitBtn = screen.getByTestId('insight-chat-submit');
    expect(submitBtn).toBeDisabled();

    // Simulate form submission directly
    const user = userEvent.setup();
    const input = screen.getByTestId('insight-chat-input');
    await user.type(input, ' ');
    await user.type(input, '{enter}');

    // Check it wasn't called (the input only has spaces which gets trimmed)
    expect(global.fetch).not.toHaveBeenCalledWith(
        expect.stringContaining('/api/ui/dashboard/analytics/chat'),
        expect.anything()
    );
  });


  it('handles resTriage ok branch', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ items: [{ id: '1', source: 'Decision Assistant', context: 'Test triage item', action_type: 'Test Action' }] }) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Test Action')).toBeTruthy();
    });
  });

  it('handles empty action_type branch', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve([{ id: '1', source: 'Decision Assistant', context: 'Test triage item', action_type: '' }]) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Execute Action')).toBeTruthy();
    });
  });

  it('handles chat fallback reply correctly', async () => {
    (global.fetch as any).mockImplementation((url: string, options: any) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/dashboard/analytics/chat')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Briefing')).toBeTruthy();
    });

    const user = userEvent.setup();
    const input = screen.getByTestId('insight-chat-input');
    const submitBtn = screen.getByTestId('insight-chat-submit');

    await user.type(input, 'Hello agent');
    await user.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByText('I encountered an error retrieving that information.')).toBeTruthy();
    });
  });

  it('handles chat empty or loading branch', async () => {
    let resolveChatPromise: any;
    const chatPromise = new Promise((resolve) => { resolveChatPromise = resolve; });
    (global.fetch as any).mockImplementation((url: string, options: any) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/dashboard/analytics/chat')) {
        return chatPromise;
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Briefing')).toBeTruthy();
    });

    const submitBtn = screen.getByTestId('insight-chat-submit');
    const input = screen.getByTestId('insight-chat-input');

    expect(submitBtn).toBeDisabled();

    const user = userEvent.setup();
    await user.type(input, 'test');

    expect(submitBtn).not.toBeDisabled();

    // click will stay pending because the fetch won't resolve yet
    user.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByText("Thinking...")).toBeTruthy();
    });

    // Resolve the hanging promise to finish test
    resolveChatPromise({ ok: true, json: () => Promise.resolve({ reply: 'Test chat reply' }) });

    await waitFor(() => {
      expect(screen.queryByText("Thinking...")).not.toBeTruthy();
    });
  });

  it('handles resTriage root items object fallback branch', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ items: undefined }) }); // Fallback
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Briefing')).toBeTruthy();
    });
  });

  it('handles resTriage failure branch', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('/api/ui/dashboard/analytics/briefing')) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ briefing: 'Briefing' }) });
      }
      if (url.includes('/api/ui/triage')) {
        return Promise.resolve({ ok: false });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<MorningBriefingCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Briefing')).toBeTruthy();
    });
  });
});
