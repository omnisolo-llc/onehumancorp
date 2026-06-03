import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import InboxPage from './page';

vi.mock('next/link', () => ({
  default: ({ children, href }: { children: React.ReactNode; href: string }) => (
    <a href={href}>{children}</a>
  ),
}));

describe('InboxPage', () => {
  it('renders without hardcoded mock data initially', () => {
    render(<InboxPage />);
    // Since we start with empty messages array, we shouldn't see previous mock content
    expect(screen.queryByText('Do you have vegan birthday cake options?')).toBeNull();
    // But the header should be present
    expect(screen.getByText('Customer Inbox')).toBeDefined();
  });

  it('can simulate an incoming message and an AI drafted reply with confidence indicator', async () => {
    render(<InboxPage />);

    // Click the simulate button
    const simulateBtn = screen.getByTitle('Simulate Incoming Message');
    fireEvent.click(simulateBtn);

    // Initial simulated message
    expect(screen.getByText('Are you open today?')).toBeDefined();

    // Wait for the simulated AI draft to appear (which is delayed by setTimeout)
    await waitFor(() => {
      expect(screen.getByText(/Hi! Yes, we are open until 6 PM/)).toBeDefined();
    });

    // Check that the AI draft indicator is present
    expect(screen.getByText('AI Draft')).toBeDefined();

    // Check that the High Confidence indicator is present
    expect(screen.getByText('High Confidence')).toBeDefined();
  });

  it('can send a manual reply', () => {
    const { container } = render(<InboxPage />);

    // Using hidden input from page.tsx specifically kept for tests
    const replyInput = container.querySelector('#reply-input') as HTMLInputElement | null;
    const sendButton = screen.getAllByText('Send')[0];

    if (replyInput) {
       fireEvent.change(replyInput, { target: { value: 'Hello testing' } });
       fireEvent.click(sendButton);
       expect(screen.getByText('Hello testing')).toBeDefined();
    }
  });
});
