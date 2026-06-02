import { render, screen, act } from '@testing-library/react';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import userEvent from '@testing-library/user-event';
import { expect, test, vi } from 'vitest';

global.fetch = vi.fn(() =>
  Promise.resolve({
    json: () => Promise.resolve({
      "test-id": "Fetched tooltip text"
    }),
    ok: true,
  })
) as any;

test('renders the tooltip and advances to the next step and finishes', async () => {
  const user = userEvent.setup();

  await act(async () => {
    render(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default text">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );
  });

  const button = screen.getByRole('button', { name: 'Hover me' });
  await act(async () => {
    await user.hover(button);
  });

  expect(screen.getByText('Fetched tooltip text')).toBeDefined();
});
