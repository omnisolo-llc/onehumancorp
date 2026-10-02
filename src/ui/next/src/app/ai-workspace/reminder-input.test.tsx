import React from 'react';
import { fireEvent, render, screen } from '@testing-library/react';
import { afterEach, expect, it, vi } from 'vitest';
import AIWorkspacePage from './page';
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }), usePathname: () => '/ai-workspace' }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</> }));
vi.mock('../../components/VoiceAssistant', () => ({ VoiceAssistant: () => <div /> }));
vi.mock('../components/Omnibox', () => ({ Omnibox: () => <div /> }));
vi.mock('../components/LogoutButton', () => ({ LogoutButton: () => <button>Logout</button> }));
afterEach(() => { vi.useRealTimers(); vi.unstubAllGlobals(); });
it.each(['', '   '])('does not offer Add Reminder for an empty local draft: %j', value => {
  render(<AIWorkspacePage />); fireEvent.change(screen.getByPlaceholderText('Add quick reminder...'), { target: { value } });
  expect(screen.getByRole('button', { name: 'Add Reminder' })).toBeDisabled();
  expect(screen.queryByText('Reminder set.')).not.toBeInTheDocument();
});
it('adds an entered reminder draft with an honest local acknowledgement', () => {
  vi.useFakeTimers(); const fetcher = vi.fn(); vi.stubGlobal('fetch', fetcher);
  render(<AIWorkspacePage />); const input = screen.getByPlaceholderText('Add quick reminder...');
  fireEvent.change(input, { target: { value: '  Review the prepared invoice  ' } });
  const submit = screen.getByRole('button', { name: 'Add Reminder' }); expect(submit).toBeEnabled(); fireEvent.click(submit);
  expect(screen.getByText('Review the prepared invoice')).toBeVisible(); expect(input).toHaveValue(''); expect(submit).toBeDisabled();
  expect(screen.getByText('Reminder draft added to this view. No notification was scheduled.')).toBeVisible();
  expect(screen.queryByText('Reminder set.')).not.toBeInTheDocument(); expect(fetcher).not.toHaveBeenCalled();
});
