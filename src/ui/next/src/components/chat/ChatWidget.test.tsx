import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { ChatWidget } from './ChatWidget';

describe('ChatWidget', () => {
  it('renders correctly and can be opened', () => {
    render(<ChatWidget />);

    // Should be closed initially
    expect(screen.queryByText('Chat with us')).not.toBeInTheDocument();

    // Click toggle
    fireEvent.click(screen.getByLabelText('Toggle chat'));

    // Should be open
    expect(screen.getByText('Chat with us')).toBeInTheDocument();
  });
});
