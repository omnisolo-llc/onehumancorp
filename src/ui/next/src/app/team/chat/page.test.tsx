import { render, screen, fireEvent } from '@testing-library/react';
import NativeOmnichannelChat from './page';

// Mock useRouter
jest.mock('next/navigation', () => ({
  useRouter: () => ({
    push: jest.fn(),
  }),
}));

describe('NativeOmnichannelChat', () => {
  it('allows the user to send a message and simulates AI draft', async () => {
    render(<NativeOmnichannelChat />);

    const input = screen.getByTestId('team-chat-input');
    const sendButton = screen.getByTestId('team-chat-send');

    // Simulate user typing a message
    fireEvent.change(input, { target: { value: 'Hello team' } });
    fireEvent.click(sendButton);

    // Ensure the message shows up
    expect(screen.getByText('Hello team')).toBeInTheDocument();

    // AI drafted reply mock should show up
    const aiDraftText = await screen.findByText("I've drafted a reply for your approval.", {}, { timeout: 2000 });
    expect(aiDraftText).toBeInTheDocument();
  });
});
