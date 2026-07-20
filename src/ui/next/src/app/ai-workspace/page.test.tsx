import React from 'react';
import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import AIWorkspacePage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
  usePathname: () => '/ai-workspace',
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

vi.mock('../../components/VoiceAssistant', () => ({
  VoiceAssistant: () => <div data-testid="voice-assistant" />,
}));

vi.mock('../components/Omnibox', () => ({
  Omnibox: () => <div data-testid="omnibox" />,
}));

vi.mock('../components/LogoutButton', () => ({
  LogoutButton: () => <button data-testid="logout-btn">Logout</button>,
}));

describe('AIWorkspacePage', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  it('renders the AI Workspace page with main overview tab active', () => {
    render(<AIWorkspacePage />);
    
    // Overview tab content checks
    expect(screen.getByText('AI Workspace Platform')).toBeDefined();
    expect(screen.getByText('AI Communication Summary & Digests')).toBeDefined();
    expect(screen.getByText('Workspace Data Analysis')).toBeDefined();
    expect(screen.getByText('Active Goals (OKRs)')).toBeDefined();
  });

  it('switches between workspace tabs', async () => {
    render(<AIWorkspacePage />);
    
    // Switch to Tasks Tab
    const tasksTabBtn = screen.getByRole('button', { name: 'tasks' });
    await act(async () => {
      fireEvent.click(tasksTabBtn);
    });
    expect(screen.getByText('Task Management')).toBeDefined();
    expect(screen.getByText('Create AI-Assisted Task')).toBeDefined();

    // Switch to Calendar Tab
    const calendarTabBtn = screen.getByRole('button', { name: 'calendar' });
    await act(async () => {
      fireEvent.click(calendarTabBtn);
    });
    expect(screen.getByText('Calendar & Schedules')).toBeDefined();
    expect(screen.getByText('Schedule an Event')).toBeDefined();

    // Switch to Notes Tab
    const notesTabBtn = screen.getByRole('button', { name: 'Notes & Knowledge' });
    await act(async () => {
      fireEvent.click(notesTabBtn);
    });
    expect(screen.getByText('Document Library')).toBeDefined();

    // Switch to Automation Tab
    const automationTabBtn = screen.getByRole('button', { name: 'automation' });
    await act(async () => {
      fireEvent.click(automationTabBtn);
    });
    expect(screen.getByText('Business Process Automation')).toBeDefined();

    // Switch to Assistant Tab
    const assistantTabBtn = screen.getByRole('button', { name: 'AI Planner Assistant' });
    await act(async () => {
      fireEvent.click(assistantTabBtn);
    });
    expect(screen.getByText('AI Planning Assistant')).toBeDefined();
  });

  it('allows user to toggle automation active state', async () => {
    render(<AIWorkspacePage />);
    
    // Switch to automation tab
    const automationTabBtn = screen.getByRole('button', { name: 'automation' });
    await act(async () => {
      fireEvent.click(automationTabBtn);
    });
    
    // Toggle the switch
    const switchBtn = screen.getByRole('switch', { name: /Intelligent Reminder Dispatcher/i });
    expect(switchBtn.getAttribute('aria-checked')).toBe('true');
    
    await act(async () => {
      fireEvent.click(switchBtn);
    });
    expect(switchBtn.getAttribute('aria-checked')).toBe('false');
  });

  it('runs Focus Timer controls', async () => {
    render(<AIWorkspacePage />);
    
    const startBtn = screen.getByText('Start Focus');
    expect(screen.getByText('25:00')).toBeDefined();
    
    await act(async () => {
      fireEvent.click(startBtn);
    });
    
    // Advance timers by 10 seconds
    await act(async () => {
      vi.advanceTimersByTime(10000);
    });
    
    expect(screen.getByText('24:50')).toBeDefined();
  });

  it('adds a task successfully', async () => {
    render(<AIWorkspacePage />);
    
    // Switch to tasks tab
    const tasksTabBtn = screen.getByRole('button', { name: 'tasks' });
    await act(async () => {
      fireEvent.click(tasksTabBtn);
    });

    const taskInput = screen.getByPlaceholderText('Task description...');
    const submitBtn = screen.getByText('Add Task');

    await act(async () => {
      fireEvent.change(taskInput, { target: { value: 'New Test Technical Sprint Task' } });
      fireEvent.click(submitBtn);
    });

    expect(screen.getByText('New Test Technical Sprint Task')).toBeDefined();
  });

  it('toggles calendar synchronization states', async () => {
    render(<AIWorkspacePage />);
    
    // Switch to calendar tab
    const calendarTabBtn = screen.getByRole('button', { name: 'calendar' });
    await act(async () => {
      fireEvent.click(calendarTabBtn);
    });

    const googleConnectBtn = screen.getAllByRole('button', { name: 'Connect' })[0]; // Google Calendar Connect
    expect(screen.getByText('Google Calendar')).toBeDefined();
    
    await act(async () => {
      fireEvent.click(googleConnectBtn);
    });

    // Check if connected and state updated
    expect(screen.getByText('Connected & Synced')).toBeDefined();
    expect(screen.getByText('Disconnect')).toBeDefined();
  });
});
