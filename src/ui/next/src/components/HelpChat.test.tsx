import React from 'react';
import { render, screen, act, fireEvent } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { HelpChat } from './HelpChat';
import '@testing-library/jest-dom';

describe('HelpChat Component', () => {
  it('renders correctly', () => {
    render(<HelpChat />);
    expect(screen.getByRole('button', { name: /open help chat/i })).toBeInTheDocument();
  });

  it('adds a user message when submitted', async () => {
    const user = userEvent.setup();
    render(<HelpChat />);

    const openButton = screen.getByRole('button', { name: /open help chat/i });
    await act(async () => {
        await user.click(openButton);
    });

    const input = screen.getByPlaceholderText('Ask anything...');
    await act(async () => {
      await user.type(input, 'Hello World{enter}');
    });

    expect(screen.getByText('Hello World')).toBeInTheDocument();
  });

  it('clears the chat when the clear button is clicked', async () => {
    const user = userEvent.setup();
    render(<HelpChat />);

    const openButton = screen.getByRole('button', { name: /open help chat/i });
    await act(async () => {
        await user.click(openButton);
    });

    const input = screen.getByPlaceholderText('Ask anything...');
    await act(async () => {
      await user.type(input, 'Hello World{enter}');
    });

    expect(screen.getByText('Hello World')).toBeInTheDocument();

    const clearButton = screen.getByRole('button', { name: /clear/i });
    await act(async () => {
      await user.click(clearButton);
    });

    expect(screen.queryByText('Hello World')).not.toBeInTheDocument();
  });
});
