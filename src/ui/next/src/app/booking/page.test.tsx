import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import BookingFlow from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

describe('BookingFlow Component', () => {
  const renderWithProvider = (component: React.ReactElement) => {
    return render(
      <TooltipProvider>
        {component}
      </TooltipProvider>
    );
  };

  it('renders initial AI message', () => {
    renderWithProvider(<BookingFlow />);
    expect(screen.getByText(/Hi there! 👋 I am the AI assistant/i)).toBeInTheDocument();
  });

  it('allows user to type and send a message', async () => {
    renderWithProvider(<BookingFlow />);

    const input = screen.getByPlaceholderText('Type your message...');
    const sendButton = screen.getAllByRole('button')[screen.getAllByRole('button').length - 1];

    fireEvent.change(input, { target: { value: 'I need help' } });
    fireEvent.click(sendButton);

    expect(screen.getByText('I need help')).toBeInTheDocument();
  });

  it('simulates AI responding with slots', async () => {
    renderWithProvider(<BookingFlow />);

    const input = screen.getByPlaceholderText('Type your message...');
    const sendButton = screen.getAllByRole('button')[screen.getAllByRole('button').length - 1];

    fireEvent.change(input, { target: { value: 'Schedule' } });
    fireEvent.click(sendButton);

    await waitFor(() => {
      expect(screen.getByText('Tomorrow at 10:00 AM')).toBeInTheDocument();
    }, { timeout: 2000 });
  });

  it('allows user to select a slot and proceeds to confirmation', async () => {
    renderWithProvider(<BookingFlow />);

    // Send a message first
    const input = screen.getByPlaceholderText('Type your message...');
    const sendButton = screen.getAllByRole('button')[screen.getAllByRole('button').length - 1];
    fireEvent.change(input, { target: { value: 'Schedule' } });
    fireEvent.click(sendButton);

    // Wait for slots
    let slotButton: HTMLElement;
    await waitFor(() => {
      slotButton = screen.getByText('Tomorrow at 10:00 AM');
      expect(slotButton).toBeInTheDocument();
    }, { timeout: 2000 });

    fireEvent.click(slotButton!);

    // Wait for confirmation
    await waitFor(() => {
      expect(screen.getByText(/To confirm the appointment, please click below to submit the \$50 deposit/i)).toBeInTheDocument();
    }, { timeout: 2000 });
  });
});
