import { render, screen } from '@testing-library/react';
import { expect, it, vi } from 'vitest';
import { AppShell } from './AppShell';

vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</> }));
vi.mock('../../components/VoiceAssistant', () => ({ VoiceAssistant: () => null }));
vi.mock('./Omnibox', () => ({ Omnibox: () => null }));
vi.mock('./LogoutButton', () => ({ LogoutButton: () => null }));

it('keeps every navigation link named when mobile styles hide its text', () => {
  const { container } = render(<AppShell title="Work Triage">Review work</AppShell>);
  const links = container.querySelectorAll<HTMLAnchorElement>('.app-nav-link');
  expect(links.length).toBeGreaterThan(0);
  for (const link of links) {
    const label = link.textContent!;
    link.querySelectorAll('span').forEach(span => { span.style.display = 'none'; });
    expect(link).toHaveAccessibleName(label);
  }
  expect(screen.getByRole('link', { name: 'Triage' })).toHaveAttribute('href', '/triage');
});
