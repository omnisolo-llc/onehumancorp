import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import CalendarPage from './page';

vi.mock('next/link', () => {
  return {
    default: ({ children, href }: { children: React.ReactNode, href: string }) => {
      return <a href={href}>{children}</a>;
    }
  };
});

describe('CalendarPage', () => {
  it('renders the calendar page with header', () => {
    render(<CalendarPage />);
    expect(screen.getByText('Calendar & Bookings')).toBeDefined();
    expect(screen.getByText('AI Scheduling (Zero-Setup)')).toBeDefined();
  });

  it('renders upcoming appointments section', () => {
    render(<CalendarPage />);
    expect(screen.getByText('Upcoming Appointments')).toBeDefined();
    expect(screen.getByText('Custom Cake Consultation')).toBeDefined();
    expect(screen.getByText('Pipe Fixing')).toBeDefined();
    expect(screen.getByText('Styling Session')).toBeDefined();
  });

  it('renders AI operations activity feed', () => {
    render(<CalendarPage />);
    expect(screen.getByText('Operations Agent')).toBeDefined();
    expect(screen.getByText('Real-time activity of your AI managing bookings and inquiries.')).toBeDefined();
    expect(screen.getByText('Proactively offered 3 time slots to Maya for Cake Consultation via IG DM.')).toBeDefined();
    expect(screen.getByText('Automatically followed up with Carlos regarding Pipe Fixing inquiry.')).toBeDefined();
  });
});
