import React from 'react';
import { render, screen } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import InboxPage from './page';

test('renders inbox list and conversation view correctly', () => {
  const { container } = render(<InboxPage />);

  expect(screen.getByText('Work Feed / Inbox')).toBeInTheDocument();
  expect(screen.getByText('Select a conversation to view')).toBeInTheDocument();
});
