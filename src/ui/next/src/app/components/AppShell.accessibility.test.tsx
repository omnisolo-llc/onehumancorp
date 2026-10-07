import { render, screen } from '@testing-library/react';
import { expect, it, vi } from 'vitest';
import { AppShell } from './AppShell';

vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</> }));
vi.mock('../../components/VoiceAssistant', () => ({ VoiceAssistant: () => <button type="button">Voice Assistant</button> }));
vi.mock('./Omnibox', () => ({ Omnibox: () => null }));
vi.mock('./LogoutButton', () => ({ LogoutButton: () => <button type="button">Log out</button> }));

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


it('keeps Help, logout and voice outside the async status/action wrapping area', () => {
  const { container, rerender } = render(<AppShell title="Dashboard" statusItems={[{ label: 'Stock', value: 'Loading…' }]} actions={[{ label: 'Campaigns', href: '/dashboard/campaigns' }]}>Work</AppShell>);
  const help = screen.getByRole('link', { name: 'Help Center' });
  const utilities = container.querySelector('.app-topbar-utilities');
  expect(utilities).not.toBeNull();
  expect(utilities).toContainElement(help);
  expect(utilities).toContainElement(screen.getByRole('button', { name: 'Log out' }));
  expect(utilities).toContainElement(screen.getByRole('button', { name: 'Voice Assistant' }));
  const statusArea = container.querySelector('.app-topbar-context');
  expect(statusArea).not.toBeNull();
  expect(statusArea).toContainElement(screen.getByText('Loading…'));
  expect(utilities).not.toContainElement(screen.getByText('Loading…'));
  expect(utilities).not.toContainElement(container.querySelector('.app-topbar-context a[href="/dashboard/campaigns"]'));
  expect(container.querySelector('.app-topbar-right')?.firstElementChild).toBe(utilities);
  rerender(<AppShell title="Dashboard" statusItems={[{ label: 'Stock', value: '123456789' }, { label: 'Growth', value: 'Unavailable' }]} actions={[{ label: 'Campaigns', href: '/dashboard/campaigns' }]}>Work</AppShell>);
  expect(screen.getByRole('link', { name: 'Help Center' })).toBe(help);
  expect(help).toHaveAttribute('href', '/help');
  expect(container.querySelector('.app-topbar-utilities')).toBe(utilities);
  expect(utilities).not.toContainElement(screen.getByText('123456789'));
});
