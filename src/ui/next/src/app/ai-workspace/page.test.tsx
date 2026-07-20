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

  it('performs global natural language search across items', async () => {
    render(<AIWorkspacePage />);

    const searchInput = screen.getByPlaceholderText(/Natural language search/i);

    // Switch to tasks tab first
    const tasksTabBtn = screen.getByRole('button', { name: 'tasks' });
    await act(async () => {
      fireEvent.click(tasksTabBtn);
    });

    expect(screen.getByText('Integrate deep learning models into backend pipelines')).toBeDefined();
    expect(screen.getByText('Schedule weekly client feedback sessions')).toBeDefined();

    // Type search query
    await act(async () => {
      fireEvent.change(searchInput, { target: { value: 'deep learning' } });
    });

    expect(screen.getByText('Integrate deep learning models into backend pipelines')).toBeDefined();
    expect(screen.queryByText('Schedule weekly client feedback sessions')).toBeNull();
  });

  it('runs notes creation and notepad editing', async () => {
    render(<AIWorkspacePage />);

    // Switch to notes tab
    const notesTabBtn = screen.getByRole('button', { name: 'Notes & Knowledge' });
    await act(async () => {
      fireEvent.click(notesTabBtn);
    });

    const docLibraryHeader = screen.getByText('Document Library');
    expect(docLibraryHeader).toBeDefined();

    // Edit the existing note content
    const textarea = screen.getByPlaceholderText(/Start typing note details here.../i);
    await act(async () => {
      fireEvent.change(textarea, { target: { value: 'Updated note technical design specification.' } });
    });
    expect(textarea.value).toBe('Updated note technical design specification.');

    // Add a new note
    const noteNameInput = screen.getByPlaceholderText('New document name...');
    const folderInput = screen.getByPlaceholderText(/Folder/i);
    const createNoteBtn = screen.getByRole('button', { name: 'Create Note' });

    await act(async () => {
      fireEvent.change(noteNameInput, { target: { value: 'Product Roadmap Draft' } });
      fireEvent.change(folderInput, { target: { value: 'Strategy' } });
      fireEvent.click(createNoteBtn);
    });

    expect(screen.getByText('Product Roadmap Draft')).toBeDefined();
  });

  it('runs AI backlog prioritization and generates action items', async () => {
    render(<AIWorkspacePage />);

    const prioritizeBtn = screen.getByRole('button', { name: /AI Prioritize Backlog/i });

    await act(async () => {
      fireEvent.click(prioritizeBtn);
    });

    // Advance mock timers to bypass the simulation delay
    await act(async () => {
      vi.advanceTimersByTime(2000);
    });

    // Switch to tasks tab to check if the generated task appears
    const tasksTabBtn = screen.getByRole('button', { name: 'tasks' });
    await act(async () => {
      fireEvent.click(tasksTabBtn);
    });

    expect(screen.getByText(/OPTIMIZED: Deploy security patches/i)).toBeDefined();
  });

  it('runs AI Planner Assistant chat interaction', async () => {
    render(<AIWorkspacePage />);

    // Switch to assistant tab
    const assistantTabBtn = screen.getByRole('button', { name: 'AI Planner Assistant' });
    await act(async () => {
      fireEvent.click(assistantTabBtn);
    });

    const chatInput = screen.getByPlaceholderText(/Ask about project health/i);
    const sendBtn = screen.getByRole('button', { name: 'Send' });

    await act(async () => {
      fireEvent.change(chatInput, { target: { value: 'How is the project status?' } });
      fireEvent.click(sendBtn);
    });

    // Verify user message appears
    expect(screen.getByText('How is the project status?')).toBeDefined();

    // Advance timers for typing simulation delay
    await act(async () => {
      vi.advanceTimersByTime(2000);
    });

    // Verify assistant responds with status details
    expect(screen.getByText(/Your current portfolio shows 3 active projects/i)).toBeDefined();
  });

  it('supports creating and completing reminders', async () => {
    render(<AIWorkspacePage />);

    const reminderInput = screen.getByPlaceholderText('Add quick reminder...');
    const addReminderBtn = screen.getByRole('button', { name: 'Add Reminder' });

    // Add reminder
    await act(async () => {
      fireEvent.change(reminderInput, { target: { value: 'Check network proxy bounds' } });
      fireEvent.click(addReminderBtn);
    });

    const newReminder = screen.getByText('Check network proxy bounds');
    expect(newReminder).toBeDefined();

    // Toggle reminder active status (strikes out)
    await act(async () => {
      fireEvent.click(newReminder);
    });
  });
});
