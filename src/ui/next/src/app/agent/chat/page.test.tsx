import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, expect, test } from 'vitest';
import AgentChatPage from './page';

afterEach(cleanup);

test('a balance question cannot produce a fabricated verified financial amount', () => {
  render(<AgentChatPage />);
  fireEvent.change(screen.getByRole('textbox'), { target: { value: 'What is my ledger balance?' } });
  fireEvent.click(screen.getByRole('button', { name: 'Send message' }));
  expect(screen.queryByText(/1500\.00|verified ledger balance/)).toBeNull();
  expect(screen.getByText(/A verified balance is unavailable here/)).toBeVisible();
  expect(screen.getByRole('link', { name: 'View ledger statement' })).toHaveAttribute('href', '/dashboard/ledger');
});
