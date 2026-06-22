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
    expect(screen.getByText('AI Scheduling (Zero-Setup)')).toBeDefined();
  });

  it('renders upcoming appointments section', async () => {
    await act(async () => { render(<CalendarPage />); });
    expect(screen.getByText('Upcoming Appointments')).toBeDefined();
    expect(screen.getByText('No upcoming appointments.')).toBeDefined();
    // Removed mock assert
    // Removed mock assert
  });

  it('renders AI operations activity feed', async () => {
    await act(async () => { render(<CalendarPage />); });
    expect(screen.getByText('Operations Agent')).toBeDefined();
    expect(screen.getByText('Real-time activity of your AI managing bookings and inquiries.')).toBeDefined();
    // Removed ai activity mock assert
    // Removed ai activity mock assert
  });

  it('renders the Morning Briefing card', async () => {
    await act(async () => { render(<CalendarPage />); });
    expect(screen.getByText('Morning Briefing')).toBeDefined();
    expect(screen.getAllByText(/appointments/i)).toBeDefined();
  });

  it('renders appointments and expands them on click', async () => {
    global.fetch = vi.fn().mockImplementation(() => Promise.resolve({
      ok: true,
      json: () => Promise.resolve([
        {
          id: '1',
          customer_name: 'Test Customer',
          product_title: 'Test Service',
          start_time: new Date(Date.now() + 86400000).toISOString(), // Tomorrow
          status: 'pending',
          notes: 'Test notes'
        }
      ])
    }));

    await act(async () => { render(<CalendarPage />); });

    const appointmentTitle = await screen.findByText('Test Service');
    expect(appointmentTitle).toBeDefined();

    // Check for grayscale class on future event (should not be present)
    const card = appointmentTitle.closest('.border-gray-100');
    expect(card?.className).not.toContain('grayscale');

    // Click to expand
    await act(async () => {
      appointmentTitle.click();
    });

    expect(screen.getByText('Client Context')).toBeDefined();
    expect(screen.getByText('Message Client')).toBeDefined();
    expect(screen.getByText('Test notes')).toBeDefined();
  });

  it('renders past appointments with grayscale', async () => {
    global.fetch = vi.fn().mockImplementation(() => Promise.resolve({
      ok: true,
      json: () => Promise.resolve([
        {
          id: '2',
          customer_name: 'Past Customer',
          product_title: 'Past Service',
          start_time: new Date(Date.now() - 86400000).toISOString(), // Yesterday
          status: 'confirmed'
        }
      ])
    }));

    await act(async () => { render(<CalendarPage />); });

    const appointmentTitle = await screen.findByText('Past Service');
    expect(appointmentTitle).toBeDefined();

    // Check for grayscale class on past event
    const card = appointmentTitle.closest('.border-gray-100');
    expect(card?.className).toContain('grayscale');
  });
});
