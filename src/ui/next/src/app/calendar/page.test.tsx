import React from 'react';
import { render, screen, act } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import CalendarPage from './page';

vi.mock('next/link', () => {
  return {
    default: ({ children, href }: { children: React.ReactNode, href: string }) => {
      return <a href={href}>{children}</a>;
    }
  };
});

import { beforeEach, afterEach } from "vitest";

describe('CalendarPage', () => {

beforeEach(() => {
  global.fetch = vi.fn().mockImplementation(() => Promise.resolve({
    ok: true,
    json: () => Promise.resolve([])
  }));
});
afterEach(() => {
  vi.clearAllMocks();
});

  it('renders the calendar page with header', async () => {
    await act(async () => { render(<CalendarPage />); });
    expect(screen.getByText('Calendar & Bookings')).toBeDefined();
    expect(screen.getByText('AI Scheduling')).toBeDefined();
  });

  it('renders upcoming appointments section', async () => {
    await act(async () => { render(<CalendarPage />); });
    expect(screen.getByText('Today')).toBeDefined();
    expect(screen.getByText('No upcoming appointments.')).toBeDefined();
  });

  it('renders AI operations activity feed', async () => {
    await act(async () => { render(<CalendarPage />); });
    expect(screen.getByText('Appointment Details')).toBeDefined();
    expect(screen.getByText('Select an appointment to view details.')).toBeDefined();
  });
});
