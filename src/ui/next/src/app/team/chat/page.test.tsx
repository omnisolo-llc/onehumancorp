import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import TeamChatPage from './page';
import { expect, vi } from 'vitest';
import '@testing-library/jest-dom';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

global.fetch = vi.fn();

describe('TeamChatPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it('sends a message and renders the response card', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        department_assigned: 'Marketing & Advertising',
        description: 'Drafted a new email campaign',
        approval_id: 'test-approval-id'
      }),
    });

    render(<TeamChatPage />);

    const input = screen.getByTestId('team-chat-input');
    const sendBtn = screen.getByTestId('team-chat-send');

    fireEvent.change(input, { target: { value: 'Write an email to customers' } });
    fireEvent.click(sendBtn);

    expect(screen.getByText('Write an email to customers')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByTestId('action-card')).toBeInTheDocument();
    });

    expect(screen.getByText('Marketing & Advertising')).toBeInTheDocument();
    expect(screen.getByText('Drafted a new email campaign')).toBeInTheDocument();
    expect(screen.getByText('Needs Approval')).toBeInTheDocument();
    expect(global.fetch).toHaveBeenCalledWith('/api/agents/chat', expect.any(Object));
  });

  it('approves an action card successfully', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        department_assigned: 'The Manager',
        description: 'Test action',
        approval_id: 'test-approval-id'
      }),
    });

    render(<TeamChatPage />);

    const input = screen.getByTestId('team-chat-input');
    const sendBtn = screen.getByTestId('team-chat-send');

    fireEvent.change(input, { target: { value: 'test' } });
    fireEvent.click(sendBtn);

    await waitFor(() => {
      expect(screen.getByTestId('action-card')).toBeInTheDocument();
    });

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ success: true }),
    });

    const approveBtn = screen.getByTestId('approve-action-btn');
    fireEvent.click(approveBtn);

    await waitFor(() => {
      expect(screen.getByText('Approved')).toBeInTheDocument();
    });

    expect(global.fetch).toHaveBeenCalledWith('/api/agents/approvals/test-approval-id', expect.objectContaining({
      method: 'POST',
      body: JSON.stringify({ approved: true })
    }));
  });
});
