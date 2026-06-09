import { render, screen, fireEvent } from '@testing-library/react';
import ActionFeed from './page';
import { expect, test } from 'vitest';

test('renders Action Feed and handles task approval', () => {
  render(<ActionFeed />);

  // Initial state assertions
  expect(screen.getByText('Action Required: 1 Pending Inquiries')).toBeDefined();
  expect(screen.getByText('Summary: 5 orders delivered today, $450 collected.')).toBeDefined();

  expect(screen.getByText('Customer A via Instagram')).toBeDefined();
  expect(screen.getByText('Suggested Reply:')).toBeDefined();

  // Approve action
  const approveButton = screen.getByText('Approve & Send');
  fireEvent.click(approveButton);

  // Post-approval state assertions
  expect(screen.queryByText('Action Required: 1 Pending Inquiries')).toBeNull();
  expect(screen.getByText('All caught up!')).toBeDefined();
});
