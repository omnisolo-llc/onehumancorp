'use client';

import React, { useState, useEffect, useRef, useMemo } from 'react';
import { AppShell } from '../components/AppShell';

// Unique IDs for accessibility and testing
const PAGE_IDS = {
  globalSearch: 'ai-global-search-input',
  addTaskBtn: 'ai-add-task-btn',
  chatInput: 'ai-chat-input',
  chatSubmit: 'ai-chat-submit-btn',
  actionPrioritize: 'ai-prioritize-backlog-btn',
  timerToggle: 'ai-timer-toggle-btn',
  addNoteBtn: 'ai-add-note-btn',
  noteContent: 'ai-note-content-textarea',
  automationToggle: (id: string) => `ai-automation-toggle-${id}`,
};

type Task = {
  id: string;
  title: string;
  completed: boolean;
  priority: 'High' | 'Medium' | 'Low';
  project: string;
  dueDate: string;
};

type Project = {
  id: string;
  name: string;
  health: 'On Track' | 'At Risk' | 'Critical';
  progress: number; // percentage
  category: string;
};

type Note = {
  id: string;
  title: string;
  content: string;
  folder: string;
  updatedAt: string;
};

type Workflow = {
  id: string;
  name: string;
  description: string;
  active: boolean;
  trigger: string;
  action: string;
};

type ChatMessage = {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: string;
};

type Reminder = {
  id: string;
  text: string;
  time: string;
  active: boolean;
};

type Goal = {
  id: string;
  title: string;
  target: number;
  current: number;
  unit: string;
  category: string;
};

export default function AIWorkspacePage() {
  // Navigation & View State
  const [activeTab, setActiveTab] = useState<'overview' | 'tasks' | 'calendar' | 'notes' | 'automation' | 'assistant'>('overview');
  const [searchQuery, setSearchQuery] = useState('');
  const [toast, setToast] = useState<{ message: string; type: 'success' | 'info' | 'warning' } | null>(null);

  // 1. Task Management State
  const [tasks, setTasks] = useState<Task[]>([
    { id: 't1', title: 'Integrate deep learning models into backend pipelines', completed: false, priority: 'High', project: 'AI Framework Integration', dueDate: '2026-07-22' },
    { id: 't2', title: 'Schedule weekly client feedback sessions', completed: true, priority: 'Medium', project: 'Business Operations', dueDate: '2026-07-19' },
    { id: 't3', title: 'Design user onboarding tutorials for SaaS clients', completed: false, priority: 'Low', project: 'Product Launch MVP', dueDate: '2026-07-25' },
    { id: 't4', title: 'Resolve memory leak in real-time notification listener', completed: false, priority: 'High', project: 'AI Framework Integration', dueDate: '2026-07-21' },
    { id: 't5', title: 'Draft legal terms and compliance parameters', completed: true, priority: 'Low', project: 'Business Operations', dueDate: '2026-07-15' },
  ]);
  const [newTaskTitle, setNewTaskTitle] = useState('');
  const [newTaskPriority, setNewTaskPriority] = useState<'High' | 'Medium' | 'Low'>('Medium');
  const [newTaskProject, setNewTaskProject] = useState('AI Framework Integration');
  const [taskFilter, setTaskFilter] = useState<'All' | 'Active' | 'Completed'>('All');

  // 2. Project Management State
  const [projects] = useState<Project[]>([
    { id: 'p1', name: 'AI Framework Integration', health: 'On Track', progress: 68, category: 'Engineering' },
    { id: 'p2', name: 'Product Launch MVP', health: 'At Risk', progress: 42, category: 'Product' },
    { id: 'p3', name: 'Business Operations', health: 'On Track', progress: 90, category: 'Operations' },
  ]);
  const [selectedProjectId, setSelectedProjectId] = useState<string>('p1');

  // 3. Workflow & Business Process Automation State
  const [workflows, setWorkflows] = useState<Workflow[]>([
    { id: 'w1', name: 'Intelligent Reminder Dispatcher', description: 'Triggers when a task due date is < 24 hours, dispatches instant dashboard reminders.', active: true, trigger: 'Due date near', action: 'Send UI & Email Alert' },
    { id: 'w2', name: 'Lead Ingestion & Routing Agent', description: 'Automatically routes incoming business queries to the correct expert team department.', active: false, trigger: 'Inbound customer inquiry', action: 'Categorize & Assign' },
    { id: 'w3', name: 'Automated Daily Summary Report', description: 'Aggregates completed actions and tasks, compiles brief digest for team slack channels.', active: true, trigger: 'Time reaches 18:00', action: 'Summarize & Post Digest' },
  ]);

  // 4. Scheduling & Calendar State
  const [events, setEvents] = useState([
    { id: 'e1', title: 'AI Planning Standup', date: '2026-07-20', time: '10:00 AM', desc: 'Sync priorities and run action prioritize algorithm.' },
    { id: 'e2', title: 'Project Health Review', date: '2026-07-22', time: '2:30 PM', desc: 'Review critical milestones for Launch MVP.' },
    { id: 'e3', title: 'Client Sync Session', date: '2026-07-25', time: '11:00 AM', desc: 'Present deep learning integration architecture.' },
  ]);
  const [newEventTitle, setNewEventTitle] = useState('');
  const [newEventTime, setNewEventTime] = useState('09:00 AM');
  const [selectedDay, setSelectedDay] = useState<string>('2026-07-20');
  const [calendarSyncStatus, setCalendarSyncStatus] = useState<'idle' | 'syncing' | 'synced'>('idle');
  const [isGoogleSynced, setIsGoogleSynced] = useState(false);
  const [isOutlookSynced, setIsOutlookSynced] = useState(false);
  const [lastSyncTime, setLastSyncTime] = useState<string | null>(null);

  // 5. Reminders State
  const [reminders, setReminders] = useState<Reminder[]>([
    { id: 'r1', text: 'Follow up with design team regarding SVG layout styling', time: 'Tomorrow at 9:00 AM', active: true },
    { id: 'r2', text: 'Run memory garbage collection test on server nodes', time: 'Today at 5:00 PM', active: true },
    { id: 'r3', text: 'Submit weekly cost dashboard metrics', time: 'Friday at 6:00 PM', active: false },
  ]);
  const [newReminderText, setNewReminderText] = useState('');
  const [newReminderTime, setNewReminderTime] = useState('Tomorrow at 9:00 AM');

  // 6. Goal Tracking State
  const [goals, setGoals] = useState<Goal[]>([
    { id: 'g1', title: 'Deploy AI Framework MVP', target: 10, current: 7, unit: 'Milestones', category: 'Engineering' },
    { id: 'g2', title: 'Automate Business Workflows', target: 5, current: 2, unit: 'Automations', category: 'Operations' },
    { id: 'g3', title: 'Establish Note Knowledge Base', target: 20, current: 12, unit: 'Documents', category: 'Productivity' },
  ]);

  // 7. Personal Productivity State (Focus / Pomodoro Timer)
  const [timerSeconds, setTimerSeconds] = useState(1500); // 25 minutes
  const [timerActive, setTimerActive] = useState(false);
  const [habits, setHabits] = useState([
    { id: 'h1', text: 'Check daily morning briefings', done: true },
    { id: 'h2', text: 'Perform visual workflow checks', done: false },
    { id: 'h3', text: 'Review team activity streams', done: false },
    { id: 'h4', text: 'Zero out support triage inbox', done: true },
  ]);

  // 8. Note Taking & Document Organization State
  const [notes, setNotes] = useState<Note[]>([
    { id: 'n1', title: 'Deep Learning Deployment Architecture', content: 'Our neural network routing handles real-time natural language classification. We execute low-latency pipelines via edge-storefront-setup nodes.', folder: 'Architecture', updatedAt: '2026-07-20' },
    { id: 'n2', title: 'Project Launch Marketing Checklist', content: '1. Create social-proof badges\n2. Set up flash-sale coupons\n3. Launch customer referral badge-builder widget', folder: 'Marketing', updatedAt: '2026-07-18' },
    { id: 'n3', title: 'Operations API Integration Guide', content: 'Integrate the new JSON Web Token endpoints. Secure auth tokens should remain exclusively in memory or inside secure browser session controls.', folder: 'Operations', updatedAt: '2026-07-19' },
  ]);
  const [selectedNoteId, setSelectedNoteId] = useState<string>('n1');
  const [newNoteTitle, setNewNoteTitle] = useState('');
  const [newNoteFolder, setNewNoteFolder] = useState('General');

  // 9. AI Chat Assistant State
  const [messages, setMessages] = useState<ChatMessage[]>([
    { id: 'm1', role: 'assistant', content: 'Hello! I am your AI Workspace Assistant. I can analyze your task lists, recommend optimal schedules, suggest automation workflows, or draft planning strategies for your projects. What can I do for you today?', timestamp: '9:41 AM' },
  ]);
  const [chatInput, setChatInput] = useState('');
  const [isTyping, setIsTyping] = useState(false);

  // 10. AI Action Prioritization loading
  const [isPrioritizing, setIsPrioritizing] = useState(false);

  // Timer Countdown Effect
  useEffect(() => {
    let interval: NodeJS.Timeout | null = null;
    if (timerActive && timerSeconds > 0) {
      interval = setInterval(() => {
        setTimerSeconds((prev) => prev - 1);
      }, 1000);
    } else if (timerSeconds === 0) {
      setTimerActive(false);
      showToast('Focus session complete! Time to take a short break.', 'success');
      setTimerSeconds(1500);
    }
    return () => {
      if (interval) clearInterval(interval);
    };
  }, [timerActive, timerSeconds]);

  // Helper for Toast notification
  const showToast = (message: string, type: 'success' | 'info' | 'warning') => {
    setToast({ message, type });
    setTimeout(() => {
      setToast(null);
    }, 4000);
  };

  // 11. AI Action Item Generator simulation
  const handleAIPrioritize = () => {
    setIsPrioritizing(true);
    showToast('AI is parsing project backlogs and team activity...', 'info');
    setTimeout(() => {
      // Re-sort tasks placing High priority at the top, and suggest a new prioritized item
      const suggestedTask: Task = {
        id: 'tsugg_' + Date.now(),
        title: 'OPTIMIZED: Deploy security patches to prevent potential query escalations',
        completed: false,
        priority: 'High',
        project: 'AI Framework Integration',
        dueDate: '2026-07-21',
      };
      setTasks((prev) => [suggestedTask, ...prev]);
      setIsPrioritizing(false);
      showToast('AI Prioritization complete! Generated 1 high-priority action item.', 'success');
    }, 1500);
  };

  // Adding tasks
  const handleAddTask = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTaskTitle.trim()) return;

    const newTask: Task = {
      id: 't_' + Date.now(),
      title: newTaskTitle.trim(),
      completed: false,
      priority: newTaskPriority,
      project: newTaskProject,
      dueDate: new Date(Date.now() + 86400000 * 3).toISOString().split('T')[0], // 3 days from now
    };

    setTasks((prev) => [...prev, newTask]);
    setNewTaskTitle('');
    showToast(`Task "${newTask.title.slice(0, 20)}..." created successfully.`, 'success');
  };

  // Toggle tasks
  const toggleTaskCompletion = (taskId: string) => {
    setTasks((prev) =>
      prev.map((t) => (t.id === taskId ? { ...t, completed: !t.completed } : t))
    );
  };

  // Delete tasks
  const deleteTask = (taskId: string) => {
    setTasks((prev) => prev.filter((t) => t.id !== taskId));
    showToast('Task removed.', 'info');
  };

  // Add notes
  const handleAddNote = () => {
    const title = newNoteTitle.trim() || 'Untitled Note';
    const newNote: Note = {
      id: 'n_' + Date.now(),
      title,
      content: 'Write your notes here. You can outline structures, tasks, or knowledge items.',
      folder: newNoteFolder.trim() || 'General',
      updatedAt: new Date().toISOString().split('T')[0],
    };
    setNotes((prev) => [...prev, newNote]);
    setSelectedNoteId(newNote.id);
    setNewNoteTitle('');
    showToast(`Note "${title}" added.`, 'success');
  };

  // Update selected note content
  const handleNoteContentChange = (content: string) => {
    setNotes((prev) =>
      prev.map((n) => (n.id === selectedNoteId ? { ...n, content, updatedAt: new Date().toISOString().split('T')[0] } : n))
    );
  };

  const handleNoteTitleChange = (title: string) => {
    setNotes((prev) =>
      prev.map((n) => (n.id === selectedNoteId ? { ...n, title } : n))
    );
  };

  const deleteNote = (noteId: string) => {
    const remaining = notes.filter((n) => n.id !== noteId);
    setNotes(remaining);
    if (selectedNoteId === noteId && remaining.length > 0) {
      setSelectedNoteId(remaining[0].id);
    }
    showToast('Note deleted.', 'info');
  };

  // Toggle automations
  const toggleWorkflow = (workflowId: string) => {
    setWorkflows((prev) =>
      prev.map((w) => (w.id === workflowId ? { ...w, active: !w.active } : w))
    );
    const flow = workflows.find((w) => w.id === workflowId);
    showToast(`Automation "${flow?.name}" ${!flow?.active ? 'Activated' : 'Paused'}.`, 'info');
  };

  // Add calendar event
  const handleAddEvent = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newEventTitle.trim()) return;
    const newEv = {
      id: 'e_' + Date.now(),
      title: newEventTitle.trim(),
      date: selectedDay,
      time: newEventTime,
      desc: 'Scheduled via AI Workspace calendar component.',
    };
    setEvents((prev) => [...prev, newEv]);
    setNewEventTitle('');
    showToast(`Event "${newEv.title}" scheduled on ${selectedDay}.`, 'success');
  };

  // Calendar Synchronization Handlers
  const handleToggleGoogleSync = () => {
    if (isGoogleSynced) {
      setIsGoogleSynced(false);
      setEvents((prev) => prev.filter(e => e.id !== 'esync_g1'));
      showToast('Google Calendar sync disconnected.', 'info');
    } else {
      setIsGoogleSynced(true);
      showToast('Google Calendar successfully synced!', 'success');
      runCalendarSync(true, isOutlookSynced);
    }
  };

  const handleToggleOutlookSync = () => {
    if (isOutlookSynced) {
      setIsOutlookSynced(false);
      setEvents((prev) => prev.filter(e => e.id !== 'esync_o1'));
      showToast('Outlook Calendar sync disconnected.', 'info');
    } else {
      setIsOutlookSynced(true);
      showToast('Outlook Calendar successfully synced!', 'success');
      runCalendarSync(isGoogleSynced, true);
    }
  };

  const runCalendarSync = (google: boolean, outlook: boolean) => {
    setCalendarSyncStatus('syncing');
    setTimeout(() => {
      setCalendarSyncStatus('synced');
      setLastSyncTime(new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' }));
      
      const newSyncedEvents: any[] = [];
      if (google) {
        newSyncedEvents.push({
          id: 'esync_g1',
          title: 'SYNCED: Google Dev Standup',
          date: '2026-07-20',
          time: '09:00 AM',
          desc: 'Imported from connected Google Calendar.',
        });
      }
      if (outlook) {
        newSyncedEvents.push({
          id: 'esync_o1',
          title: 'SYNCED: Microsoft Partners Briefing',
          date: '2026-07-22',
          time: '01:00 PM',
          desc: 'Imported from connected Outlook Calendar.',
        });
      }
      
      setEvents((prev) => {
        const filtered = prev.filter(e => !e.id.startsWith('esync_'));
        return [...filtered, ...newSyncedEvents];
      });
      showToast('Calendar synchronization complete.', 'success');
    }, 1200);
  };

  const handleManualSync = () => {
    if (!isGoogleSynced && !isOutlookSynced) {
      showToast('Connect at least one calendar service to sync.', 'warning');
      return;
    }
    runCalendarSync(isGoogleSynced, isOutlookSynced);
  };

  // Add reminder
  const handleAddReminder = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newReminderText.trim()) return;
    const newRem: Reminder = {
      id: 'r_' + Date.now(),
      text: newReminderText.trim(),
      time: newReminderTime,
      active: true,
    };
    setReminders((prev) => [...prev, newRem]);
    setNewReminderText('');
    showToast('Reminder set.', 'success');
  };

  // Toggle habit checkbox
  const toggleHabit = (habitId: string) => {
    setHabits((prev) =>
      prev.map((h) => (h.id === habitId ? { ...h, done: !h.done } : h))
    );
  };

  // Global Natural Language Search Logic
  const filteredTasks = useMemo(() => {
    let list = tasks;
    if (taskFilter === 'Active') list = list.filter((t) => !t.completed);
    if (taskFilter === 'Completed') list = list.filter((t) => t.completed);

    if (!searchQuery.trim()) return list;
    const q = searchQuery.toLowerCase();
    return list.filter((t) =>
      t.title.toLowerCase().includes(q) ||
      t.priority.toLowerCase().includes(q) ||
      t.project.toLowerCase().includes(q)
    );
  }, [tasks, taskFilter, searchQuery]);

  const filteredNotes = useMemo(() => {
    if (!searchQuery.trim()) return notes;
    const q = searchQuery.toLowerCase();
    return notes.filter((n) =>
      n.title.toLowerCase().includes(q) ||
      n.content.toLowerCase().includes(q) ||
      n.folder.toLowerCase().includes(q)
    );
  }, [notes, searchQuery]);

  const filteredEvents = useMemo(() => {
    if (!searchQuery.trim()) return events;
    const q = searchQuery.toLowerCase();
    return events.filter((e) =>
      e.title.toLowerCase().includes(q) ||
      e.desc.toLowerCase().includes(q)
    );
  }, [events, searchQuery]);

  // AI Chat Simulation
  const handleChatSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!chatInput.trim()) return;

    const userMsg: ChatMessage = {
      id: 'm_u_' + Date.now(),
      role: 'user',
      content: chatInput.trim(),
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
    };

    setMessages((prev) => [...prev, userMsg]);
    setChatInput('');
    setIsTyping(true);

    // AI thinking timeout simulation
    setTimeout(() => {
      let aiContent = "I've analyzed your workspace parameters. All projects look healthy. How else can I assist with your planning details?";
      const text = userMsg.content.toLowerCase();

      if (text.includes('status') || text.includes('health') || text.includes('project')) {
        const atRiskCount = projects.filter((p) => p.health === 'At Risk').length;
        aiContent = `Your current portfolio shows ${projects.length} active projects. ${projects.length - atRiskCount} are On Track, while 1 (${projects.find(p => p.health === 'At Risk')?.name}) is flagged as At Risk due to minor backlog delays. I suggest setting up a 'Lead Ingestion Agent' flow to accelerate operations.`;
      } else if (text.includes('task') || text.includes('prioritize')) {
        const highPriority = tasks.filter((t) => !t.completed && t.priority === 'High');
        aiContent = `You have ${highPriority.length} high-priority tasks pending. Your top-priority item is: "${highPriority[0]?.title || 'none'}". I recommend executing the Pomodoro Timer to tackle this item now.`;
      } else if (text.includes('schedule') || text.includes('calendar') || text.includes('time')) {
        aiContent = `Looking at your upcoming events, your next sync session is scheduled for July 22nd: "Project Health Review". I've reserved a focus-time slot on July 21st from 2:00 PM to 4:00 PM for deep engineering tasks.`;
      } else if (text.includes('automate') || text.includes('workflow')) {
        aiContent = `You currently have 2 active workflows. The "Intelligent Reminder Dispatcher" is running. I recommend toggling the "Lead Ingestion & Routing Agent" to automate incoming client communications.`;
      } else if (text.includes('note') || text.includes('document')) {
        aiContent = `I found ${notes.length} documents in your database. The note "${notes[0]?.title}" details neural network deployment configurations. You can search these dynamically using the workspace search bar.`;
      }

      const aiMsg: ChatMessage = {
        id: 'm_a_' + Date.now(),
        role: 'assistant',
        content: aiContent,
        timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      };

      setMessages((prev) => [...prev, aiMsg]);
      setIsTyping(false);
    }, 1200);
  };

  // Formatted Timer string
  const formatTimer = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  // Data Analysis Statistics Calculations
  const taskStats = useMemo(() => {
    const total = tasks.length;
    const completed = tasks.filter((t) => t.completed).length;
    const pending = total - completed;
    const rate = total > 0 ? Math.round((completed / total) * 100) : 0;
    return { total, completed, pending, rate };
  }, [tasks]);

  const activeNote = notes.find((n) => n.id === selectedNoteId) || notes[0];

  return (
    <AppShell title="AI Workspace Platform" subtitle="Trademark-aligned AI-assisted productivity and business automation console.">
      <div className="min-h-screen text-slate-800 dark:text-slate-100 flex flex-col font-sans antialiased pb-12">
        
        {/* Toast Alert */}
        {toast && (
          <div className="fixed bottom-6 right-6 z-[9999] flex items-center gap-3 px-5 py-4 rounded-xl shadow-2xl backdrop-blur-xl border border-emerald-500/20 bg-white/90 dark:bg-slate-900/90 max-w-sm">
            <span className="flex h-3 w-3 relative">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
              <span className="relative inline-flex rounded-full h-3 w-3 bg-emerald-500"></span>
            </span>
            <p className="text-sm font-medium text-slate-800 dark:text-slate-200">{toast.message}</p>
          </div>
        )}

        {/* Global Toolbar */}
        <section className="mb-6 flex flex-col md:flex-row gap-4 items-center justify-between p-4 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/45 dark:bg-slate-900/45 backdrop-blur-md shadow-sm">
          {/* Natural Language Search */}
          <div className="relative w-full md:w-96">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-slate-400">
              <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
            </div>
            <input
              id={PAGE_IDS.globalSearch}
              type="text"
              placeholder="Natural language search (e.g. High priority architecture note)..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-10 pr-4 py-2 bg-white/60 dark:bg-slate-900/60 border border-slate-200 dark:border-slate-800 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
            />
            {searchQuery && (
              <button
                onClick={() => setSearchQuery('')}
                className="absolute inset-y-0 right-0 pr-3 flex items-center text-slate-400 hover:text-slate-600"
              >
                Clear
              </button>
            )}
          </div>

          {/* AI Decision Support & Action Prioritization */}
          <div className="flex gap-2 w-full md:w-auto justify-end">
            <button
              id={PAGE_IDS.actionPrioritize}
              onClick={handleAIPrioritize}
              disabled={isPrioritizing}
              className="px-4 py-2 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 hover:to-indigo-700 text-white rounded-xl text-sm font-semibold transition-all shadow-md hover:shadow-lg disabled:opacity-50 flex items-center gap-2"
            >
              <svg className={`w-4 h-4 ${isPrioritizing ? 'animate-spin' : ''}`} fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
              </svg>
              {isPrioritizing ? 'Generating Priorities...' : 'AI Prioritize Backlog'}
            </button>
          </div>
        </section>

        {/* Workspace Hub Tabs Navigation */}
        <nav className="flex flex-wrap gap-2 mb-6 border-b border-slate-200 dark:border-slate-800 pb-2" aria-label="Workspace Tabs">
          {(['overview', 'tasks', 'calendar', 'notes', 'automation', 'assistant'] as const).map((tab) => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={`px-4 py-2.5 rounded-xl text-sm font-semibold transition-all capitalize ${
                activeTab === tab
                  ? 'bg-blue-600 text-white shadow-md'
                  : 'text-slate-600 dark:text-slate-400 hover:bg-white/50 dark:hover:bg-slate-900/50'
              }`}
            >
              {tab === 'notes' ? 'Notes & Knowledge' : tab === 'assistant' ? 'AI Planner Assistant' : tab}
            </button>
          ))}
        </nav>

        {/* Tab Contents */}
        <div className="flex-1">
          {activeTab === 'overview' && (
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              
              {/* Left Column: Metrics & Summaries */}
              <div className="lg:col-span-2 flex flex-col gap-6">
                
                {/* AI Summary and Digest */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <h2 className="text-lg font-bold mb-4 font-outfit text-slate-900 dark:text-white flex items-center gap-2">
                    <span className="w-2.5 h-2.5 rounded-full bg-blue-500"></span>
                    AI Communication Summary & Digests
                  </h2>
                  <div className="space-y-4">
                    <div className="p-4 rounded-xl bg-blue-50/50 dark:bg-blue-900/10 border border-blue-100 dark:border-blue-900/30 text-sm">
                      <p className="font-semibold text-blue-900 dark:text-blue-200 mb-1">Slack Channel Highlights (3 channels summarized)</p>
                      <ul className="list-disc pl-4 space-y-1 text-slate-700 dark:text-slate-300">
                        <li>Design team request update on the affiliate badge-builder SVG dimensions.</li>
                        <li>QA completed regression checks for visual workflow API nodes; minor latency warning logged.</li>
                        <li>Client requesting a demo on the new schedule recommendation matrix tomorrow.</li>
                      </ul>
                    </div>
                    <div className="p-4 rounded-xl bg-indigo-50/50 dark:bg-indigo-900/10 border border-indigo-100 dark:border-indigo-900/30 text-sm">
                      <p className="font-semibold text-indigo-900 dark:text-indigo-200 mb-1">AI Productivity Recommendations</p>
                      <p className="text-slate-700 dark:text-slate-300">
                        "Your peak completion hours are 10:00 AM - 12:00 PM. Schedule your core technical tasks during this focus slot. I have automated your weekly client report updates to run at 6:00 PM on Friday."
                      </p>
                    </div>
                  </div>
                </div>

                {/* Data Analysis Productivity Analytics */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <div className="flex justify-between items-center mb-6">
                    <h2 className="text-lg font-bold font-outfit text-slate-900 dark:text-white">Workspace Data Analysis</h2>
                    <span className="text-xs font-semibold px-2.5 py-1 rounded-full bg-blue-100 dark:bg-blue-950 text-blue-700 dark:text-blue-300">
                      Productivity Score: 87%
                    </span>
                  </div>

                  <div className="grid grid-cols-1 md:grid-cols-2 gap-6 items-center">
                    {/* SVG Graphic Completion Bar Chart */}
                    <div className="flex flex-col items-center">
                      <p className="text-xs text-slate-500 mb-3 font-semibold">Weekly Completion Trends</p>
                      <svg width="100%" height="160" viewBox="0 0 300 160" className="overflow-visible">
                        {/* Grid lines */}
                        <line x1="30" y1="20" x2="280" y2="20" stroke="#cbd5e1" strokeWidth="1" strokeDasharray="4 4" className="dark:stroke-slate-800" />
                        <line x1="30" y1="70" x2="280" y2="70" stroke="#cbd5e1" strokeWidth="1" strokeDasharray="4 4" className="dark:stroke-slate-800" />
                        <line x1="30" y1="120" x2="280" y2="120" stroke="#cbd5e1" strokeWidth="1" strokeDasharray="4 4" className="dark:stroke-slate-800" />
                        <line x1="30" y1="140" x2="280" y2="140" stroke="#64748b" strokeWidth="1.5" />
                        
                        {/* Bar items */}
                        {[
                          { day: 'Mon', val: 90, count: 4 },
                          { day: 'Tue', val: 120, count: 6 },
                          { day: 'Wed', val: 60, count: 3 },
                          { day: 'Thu', val: 110, count: 5 },
                          { day: 'Fri', val: 130, count: 7 },
                        ].map((item, idx) => {
                          const barWidth = 30;
                          const spacing = 48;
                          const x = 50 + idx * spacing;
                          const barHeight = item.val;
                          const y = 140 - barHeight;
                          return (
                            <g key={idx}>
                              <rect
                                x={x}
                                y={y}
                                width={barWidth}
                                height={barHeight}
                                rx="4"
                                fill="url(#blueGradient)"
                                className="transition-all duration-500 hover:opacity-85"
                              />
                              <text x={x + 15} y="155" textAnchor="middle" className="text-[10px] font-semibold fill-slate-500 dark:fill-slate-400">
                                {item.day}
                              </text>
                              <text x={x + 15} y={y - 5} textAnchor="middle" className="text-[10px] font-bold fill-blue-600 dark:fill-blue-400">
                                {item.count}
                              </text>
                            </g>
                          );
                        })}
                        <defs>
                          <linearGradient id="blueGradient" x1="0" y1="0" x2="0" y2="1">
                            <stop offset="0%" stopColor="#2563eb" />
                            <stop offset="100%" stopColor="#6366f1" />
                          </linearGradient>
                        </defs>
                      </svg>
                    </div>

                    {/* Stats List */}
                    <div className="space-y-4">
                      <div>
                        <div className="flex justify-between text-xs mb-1 font-semibold text-slate-500">
                          <span>Task Completion Ratio</span>
                          <span className="text-slate-800 dark:text-slate-200">{taskStats.completed}/{taskStats.total} ({taskStats.rate}%)</span>
                        </div>
                        <div className="w-full bg-slate-200 dark:bg-slate-800 h-2.5 rounded-full overflow-hidden">
                          <div className="bg-blue-600 h-2.5 rounded-full" style={{ width: `${taskStats.rate}%` }}></div>
                        </div>
                      </div>
                      <div>
                        <div className="flex justify-between text-xs mb-1 font-semibold text-slate-500">
                          <span>Workflows Automation Load</span>
                          <span className="text-slate-800 dark:text-slate-200">
                            {workflows.filter(w => w.active).length} of {workflows.length} active
                          </span>
                        </div>
                        <div className="w-full bg-slate-200 dark:bg-slate-800 h-2.5 rounded-full overflow-hidden">
                          <div className="bg-indigo-600 h-2.5 rounded-full" style={{ width: `${(workflows.filter(w => w.active).length / workflows.length) * 100}%` }}></div>
                        </div>
                      </div>
                      <div className="grid grid-cols-2 gap-3 pt-2">
                        <div className="p-3 bg-white/40 dark:bg-slate-900/40 rounded-xl border border-slate-200/60 dark:border-slate-800/60">
                          <p className="text-[10px] text-slate-500 font-bold uppercase">Time Saved This Week</p>
                          <p className="text-xl font-bold text-slate-800 dark:text-white">8.4 hrs</p>
                        </div>
                        <div className="p-3 bg-white/40 dark:bg-slate-900/40 rounded-xl border border-slate-200/60 dark:border-slate-800/60">
                          <p className="text-[10px] text-slate-500 font-bold uppercase">Active OKRs</p>
                          <p className="text-xl font-bold text-indigo-600 dark:text-indigo-400">3 Goals</p>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                {/* AI Planning & Decision Support Matrix */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <h2 className="text-lg font-bold mb-4 font-outfit text-slate-900 dark:text-white">AI Decision Support Matrix</h2>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div className="p-4 rounded-xl border border-slate-200/80 dark:border-slate-800 bg-white/50 dark:bg-slate-900/50">
                      <p className="text-xs font-bold text-rose-500 uppercase tracking-wider mb-2">High Impact / High Urgency</p>
                      <p className="text-sm font-semibold text-slate-800 dark:text-white">Integrate deep learning models into backend pipelines</p>
                      <p className="text-xs text-slate-500 mt-1">Resolution plan: Dedicate next Pomodoro session. Block distractions.</p>
                    </div>
                    <div className="p-4 rounded-xl border border-slate-200/80 dark:border-slate-800 bg-white/50 dark:bg-slate-900/50">
                      <p className="text-xs font-bold text-blue-500 uppercase tracking-wider mb-2">High Impact / Low Urgency</p>
                      <p className="text-sm font-semibold text-slate-800 dark:text-white">Design user onboarding tutorials for SaaS clients</p>
                      <p className="text-xs text-slate-500 mt-1">Resolution plan: Delegate outline phase to Knowledge base notes.</p>
                    </div>
                  </div>
                </div>

              </div>

              {/* Right Column: Timer, Goals, Reminders */}
              <div className="flex flex-col gap-6">

                {/* Focus / Pomodoro Timer */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-gradient-to-br from-indigo-900 to-slate-950 text-white shadow-xl text-center relative overflow-hidden">
                  <div className="absolute top-0 right-0 w-24 h-24 bg-blue-500/10 rounded-full blur-2xl"></div>
                  <h2 className="text-sm font-bold uppercase tracking-wider text-indigo-300 mb-3">Personal Productivity Timer</h2>
                  
                  <div className="text-5xl font-mono font-bold tracking-tight mb-4">
                    {formatTimer(timerSeconds)}
                  </div>

                  <div className="flex justify-center gap-2 mb-6">
                    <button
                      id={PAGE_IDS.timerToggle}
                      onClick={() => setTimerActive(!timerActive)}
                      className={`px-4 py-2 rounded-xl text-xs font-bold transition-all ${
                        timerActive ? 'bg-amber-500 text-slate-950 hover:bg-amber-600' : 'bg-blue-600 text-white hover:bg-blue-700'
                      }`}
                    >
                      {timerActive ? 'Pause Session' : 'Start Focus'}
                    </button>
                    <button
                      onClick={() => {
                        setTimerActive(false);
                        setTimerSeconds(1500);
                      }}
                      className="px-4 py-2 bg-white/10 hover:bg-white/20 text-white rounded-xl text-xs font-semibold transition-all"
                    >
                      Reset
                    </button>
                  </div>

                  {/* Habit checklist */}
                  <div className="text-left border-t border-white/10 pt-4">
                    <p className="text-xs font-bold text-indigo-200 mb-2 uppercase tracking-wide">Daily Habits checklist</p>
                    <div className="space-y-2">
                      {habits.map(h => (
                        <label key={h.id} className="flex items-center gap-2 text-xs font-medium text-slate-300 hover:text-white cursor-pointer select-none">
                          <input
                            type="checkbox"
                            checked={h.done}
                            onChange={() => toggleHabit(h.id)}
                            className="rounded border-slate-700 text-indigo-600 focus:ring-indigo-500 bg-slate-900"
                          />
                          <span className={h.done ? 'line-through text-slate-500' : ''}>{h.text}</span>
                        </label>
                      ))}
                    </div>
                  </div>
                </div>

                {/* OKR / Goal Tracking Progress */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <h2 className="text-lg font-bold mb-4 font-outfit text-slate-900 dark:text-white">Active Goals (OKRs)</h2>
                  <div className="space-y-4">
                    {goals.map((g) => {
                      const pct = Math.round((g.current / g.target) * 100);
                      return (
                        <div key={g.id} className="text-sm">
                          <div className="flex justify-between font-semibold mb-1">
                            <span className="text-slate-800 dark:text-slate-200">{g.title}</span>
                            <span className="text-blue-600 dark:text-blue-400">{g.current}/{g.target} {g.unit}</span>
                          </div>
                          <div className="w-full bg-slate-100 dark:bg-slate-800 h-2 rounded-full overflow-hidden">
                            <div className="bg-gradient-to-r from-blue-500 to-indigo-600 h-2 rounded-full" style={{ width: `${pct}%` }}></div>
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </div>

                {/* Reminders Widgets */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <h2 className="text-lg font-bold mb-3 font-outfit text-slate-900 dark:text-white">Reminders</h2>
                  
                  <div className="space-y-2 mb-4">
                    {reminders.map((rem) => (
                      <div
                        key={rem.id}
                        onClick={() => {
                          setReminders(prev => prev.map(r => r.id === rem.id ? { ...r, active: !r.active } : r));
                        }}
                        className={`p-3 rounded-xl border text-xs font-semibold cursor-pointer transition-all ${
                          rem.active
                            ? 'border-blue-100 dark:border-blue-900/30 bg-blue-50/20 dark:bg-blue-900/10 text-slate-800 dark:text-slate-200'
                            : 'border-slate-200/60 dark:border-slate-800/60 text-slate-400 dark:text-slate-500 line-through'
                        }`}
                      >
                        <p>{rem.text}</p>
                        <p className="text-[10px] text-slate-400 mt-1">{rem.time}</p>
                      </div>
                    ))}
                  </div>

                  <form onSubmit={handleAddReminder} className="space-y-2">
                    <input
                      type="text"
                      placeholder="Add quick reminder..."
                      value={newReminderText}
                      onChange={(e) => setNewReminderText(e.target.value)}
                      className="w-full px-3 py-1.5 bg-white/60 dark:bg-slate-900/60 border border-slate-200 dark:border-slate-800 rounded-lg text-xs"
                    />
                    <button type="submit" className="w-full py-1.5 bg-slate-200 dark:bg-slate-800 hover:bg-slate-300 dark:hover:bg-slate-700 rounded-lg text-xs font-bold transition-all">
                      Add Reminder
                    </button>
                  </form>
                </div>

              </div>

            </div>
          )}

          {activeTab === 'tasks' && (
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              
              {/* Tasks List Column */}
              <div className="lg:col-span-2 p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                
                {/* Task Controls */}
                <div className="flex justify-between items-center mb-6">
                  <h2 className="text-xl font-bold font-outfit text-slate-900 dark:text-white">Task Management</h2>
                  <div className="flex gap-1.5 bg-slate-100 dark:bg-slate-800/60 p-1 rounded-xl">
                    {(['All', 'Active', 'Completed'] as const).map((filter) => (
                      <button
                        key={filter}
                        onClick={() => setTaskFilter(filter)}
                        className={`px-3 py-1 rounded-lg text-xs font-bold transition-all ${
                          taskFilter === filter
                            ? 'bg-white dark:bg-slate-700 text-slate-900 dark:text-white shadow-sm'
                            : 'text-slate-500 hover:text-slate-800 dark:hover:text-slate-200'
                        }`}
                      >
                        {filter}
                      </button>
                    ))}
                  </div>
                </div>

                {/* Tasks Stack */}
                <div className="space-y-3 mb-6">
                  {filteredTasks.length === 0 ? (
                    <div className="p-8 text-center text-slate-400 text-sm">
                      No tasks found matching current filters.
                    </div>
                  ) : (
                    filteredTasks.map((t) => (
                      <div
                        key={t.id}
                        className={`p-4 rounded-xl border transition-all flex items-center justify-between gap-4 ${
                          t.completed
                            ? 'border-slate-100 dark:border-slate-800/40 bg-slate-50/40 dark:bg-slate-950/20 opacity-70'
                            : 'border-slate-200/80 dark:border-slate-800/60 bg-white/50 dark:bg-slate-900/50 hover:border-slate-300 dark:hover:border-slate-700'
                        }`}
                      >
                        <div className="flex items-center gap-3 min-w-0">
                          <input
                            type="checkbox"
                            checked={t.completed}
                            onChange={() => toggleTaskCompletion(t.id)}
                            className="h-4.5 w-4.5 rounded border-slate-300 text-blue-600 focus:ring-blue-500"
                          />
                          <div className="min-w-0">
                            <h3 className={`font-semibold text-sm ${t.completed ? 'line-through text-slate-400' : 'text-slate-800 dark:text-slate-100'}`}>
                              {t.title}
                            </h3>
                            <div className="flex flex-wrap items-center gap-2 mt-1.5">
                              <span className="text-[10px] font-semibold text-slate-400">
                                {t.project}
                              </span>
                              <span className="text-[10px] text-slate-400">•</span>
                              <span className="text-[10px] font-semibold text-slate-400">
                                Due {t.dueDate}
                              </span>
                            </div>
                          </div>
                        </div>

                        <div className="flex items-center gap-2 shrink-0">
                          <span className={`px-2 py-0.5 rounded-full text-[10px] font-bold ${
                            t.priority === 'High'
                              ? 'bg-rose-100 dark:bg-rose-950/50 text-rose-700 dark:text-rose-300'
                              : t.priority === 'Medium'
                                ? 'bg-amber-100 dark:bg-amber-950/50 text-amber-700 dark:text-amber-300'
                                : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400'
                          }`}>
                            {t.priority}
                          </span>
                          <button
                            onClick={() => deleteTask(t.id)}
                            className="p-1 text-slate-400 hover:text-rose-500 rounded-lg transition-all"
                            aria-label="Delete Task"
                          >
                            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                            </svg>
                          </button>
                        </div>
                      </div>
                    ))
                  )}
                </div>

                {/* Create Task Form */}
                <form onSubmit={handleAddTask} className="p-4 rounded-xl border border-slate-200/80 dark:border-slate-800 bg-slate-50/50 dark:bg-slate-900/50 space-y-3">
                  <h3 className="text-sm font-bold text-slate-800 dark:text-white">Create AI-Assisted Task</h3>
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
                    <input
                      type="text"
                      placeholder="Task description..."
                      value={newTaskTitle}
                      onChange={(e) => setNewTaskTitle(e.target.value)}
                      className="w-full md:col-span-2 px-3.5 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl text-sm focus:ring-2 focus:ring-blue-500"
                    />
                    <select
                      value={newTaskPriority}
                      onChange={(e) => setNewTaskPriority(e.target.value as any)}
                      className="px-3 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl text-sm focus:ring-2 focus:ring-blue-500"
                    >
                      <option value="Low">Low Priority</option>
                      <option value="Medium">Medium Priority</option>
                      <option value="High">High Priority</option>
                    </select>
                  </div>
                  <div className="flex justify-between items-center pt-2">
                    <select
                      value={newTaskProject}
                      onChange={(e) => setNewTaskProject(e.target.value)}
                      className="px-3 py-1.5 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl text-xs"
                    >
                      {projects.map((p) => (
                        <option key={p.id} value={p.name}>{p.name}</option>
                      ))}
                    </select>
                    <button
                      id={PAGE_IDS.addTaskBtn}
                      type="submit"
                      className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded-xl text-xs transition-all shadow-md"
                    >
                      Add Task
                    </button>
                  </div>
                </form>
              </div>

              {/* Projects Management Column */}
              <div className="flex flex-col gap-6">
                
                {/* Project Portfolio */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <h2 className="text-lg font-bold mb-4 font-outfit text-slate-900 dark:text-white">Project Portfolios</h2>
                  <div className="space-y-4">
                    {projects.map((proj) => (
                      <div
                        key={proj.id}
                        onClick={() => setSelectedProjectId(proj.id)}
                        className={`p-4 rounded-xl border cursor-pointer transition-all ${
                          selectedProjectId === proj.id
                            ? 'border-blue-500 bg-blue-50/20 dark:bg-blue-900/20'
                            : 'border-slate-200/60 dark:border-slate-800/60 hover:border-slate-300 dark:hover:hover:border-slate-700'
                        }`}
                      >
                        <div className="flex justify-between items-center mb-2">
                          <h3 className="font-semibold text-sm text-slate-950 dark:text-slate-50">{proj.name}</h3>
                          <span className={`px-2 py-0.5 rounded-full text-[9px] font-bold ${
                            proj.health === 'On Track'
                              ? 'bg-emerald-100 dark:bg-emerald-950 text-emerald-800 dark:text-emerald-300'
                              : 'bg-rose-100 dark:bg-rose-950 text-rose-800 dark:text-rose-300'
                          }`}>
                            {proj.health}
                          </span>
                        </div>
                        <div className="flex justify-between items-center text-xs text-slate-500 mt-2">
                          <span>Progress</span>
                          <span className="font-semibold">{proj.progress}%</span>
                        </div>
                        <div className="w-full bg-slate-100 dark:bg-slate-800 h-1.5 rounded-full overflow-hidden mt-1">
                          <div className="bg-blue-600 h-1.5 rounded-full" style={{ width: `${proj.progress}%` }}></div>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>

                {/* Milestone Gantt Timeline Grid */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <h2 className="text-lg font-bold mb-3 font-outfit text-slate-900 dark:text-white">Project Timeline</h2>
                  <p className="text-xs text-slate-400 mb-4">Milestone progress map for {projects.find(p => p.id === selectedProjectId)?.name}</p>
                  
                  <div className="space-y-4">
                    <div className="text-xs">
                      <div className="flex justify-between font-semibold mb-1 text-slate-700 dark:text-slate-300">
                        <span>1. Alpha Infrastructure Architecture</span>
                        <span className="text-emerald-500">Completed</span>
                      </div>
                      <div className="w-full bg-slate-100 dark:bg-slate-800 h-2 rounded-full overflow-hidden">
                        <div className="bg-emerald-500 h-2 rounded-full" style={{ width: '100%' }}></div>
                      </div>
                    </div>
                    <div className="text-xs">
                      <div className="flex justify-between font-semibold mb-1 text-slate-700 dark:text-slate-300">
                        <span>2. Deep Learning Pipelines Execution</span>
                        <span className="text-blue-500">Active</span>
                      </div>
                      <div className="w-full bg-slate-100 dark:bg-slate-800 h-2 rounded-full overflow-hidden">
                        <div className="bg-blue-500 h-2 rounded-full" style={{ width: '60%' }}></div>
                      </div>
                    </div>
                    <div className="text-xs">
                      <div className="flex justify-between font-semibold mb-1 text-slate-700 dark:text-slate-300">
                        <span>3. Beta Security Audits Integration</span>
                        <span className="text-slate-400">Scheduled</span>
                      </div>
                      <div className="w-full bg-slate-100 dark:bg-slate-800 h-2 rounded-full overflow-hidden">
                        <div className="bg-slate-300 dark:bg-slate-700 h-2 rounded-full" style={{ width: '0%' }}></div>
                      </div>
                    </div>
                  </div>
                </div>

              </div>

            </div>
          )}

          {activeTab === 'calendar' && (
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              
              {/* Calendar Grid Section */}
              <div className="lg:col-span-2 p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                <div className="flex justify-between items-center mb-6">
                  <h2 className="text-xl font-bold font-outfit text-slate-900 dark:text-white">Calendar & Schedules</h2>
                  <span className="text-sm font-semibold text-slate-500">July 2026</span>
                </div>

                {/* Calendar monthly grid mockup */}
                <div className="grid grid-cols-7 gap-2 text-center text-xs font-semibold mb-2 text-slate-400">
                  <span>Sun</span><span>Mon</span><span>Tue</span><span>Wed</span><span>Thu</span><span>Fri</span><span>Sat</span>
                </div>
                <div className="grid grid-cols-7 gap-2">
                  {/* Empty cells before month starts */}
                  {Array.from({ length: 3 }).map((_, idx) => (
                    <div key={'empty_' + idx} className="aspect-square bg-slate-50/20 dark:bg-slate-950/5 rounded-xl border border-transparent"></div>
                  ))}
                  {/* Calendar Days */}
                  {Array.from({ length: 28 }).map((_, idx) => {
                    const dayNum = idx + 1;
                    const dateStr = `2026-07-${dayNum.toString().padStart(2, '0')}`;
                    const hasEvent = events.some((e) => e.date === dateStr);
                    const isSelected = selectedDay === dateStr;
                    return (
                      <button
                        key={'day_' + idx}
                        onClick={() => setSelectedDay(dateStr)}
                        className={`aspect-square rounded-xl border text-xs font-bold transition-all relative flex flex-col justify-between p-1.5 ${
                          isSelected
                            ? 'border-blue-600 bg-blue-50/40 dark:bg-blue-900/30 text-blue-600'
                            : 'border-slate-200/40 dark:border-slate-800/40 hover:border-slate-300 dark:hover:border-slate-700 bg-white/30 dark:bg-slate-900/30'
                        }`}
                      >
                        <span>{dayNum}</span>
                        {hasEvent && (
                          <span className="h-1.5 w-1.5 rounded-full bg-indigo-600 dark:bg-indigo-400 mx-auto mb-1"></span>
                        )}
                      </button>
                    );
                  })}
                </div>

                {/* Scheduling Events List */}
                <div className="mt-6 border-t border-slate-200 dark:border-slate-800 pt-6">
                  <h3 className="text-sm font-bold mb-3 text-slate-800 dark:text-white">Events for {selectedDay}</h3>
                  <div className="space-y-3">
                    {events.filter((e) => e.date === selectedDay).length === 0 ? (
                      <p className="text-xs text-slate-400 font-semibold italic">No events scheduled. AI suggests: "Perfect slot for focused deep technical execution!"</p>
                    ) : (
                      events.filter((e) => e.date === selectedDay).map((e) => (
                        <div key={e.id} className="p-3.5 rounded-xl border border-slate-200/80 dark:border-slate-800 bg-white/50 dark:bg-slate-900/50 flex justify-between items-start gap-4">
                          <div>
                            <h4 className="font-semibold text-sm text-slate-900 dark:text-white">{e.title}</h4>
                            <p className="text-xs text-slate-500 mt-0.5">{e.desc}</p>
                          </div>
                          <span className="text-[10px] font-bold px-2 py-0.5 rounded-md bg-indigo-100 dark:bg-indigo-950 text-indigo-700 dark:text-indigo-300 shrink-0">
                            {e.time}
                          </span>
                        </div>
                      ))
                    )}
                  </div>
                </div>
              </div>

              {/* Book Event Form Column */}
              <div className="flex flex-col gap-6">
                
                {/* Book Session Event */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <h2 className="text-lg font-bold mb-4 font-outfit text-slate-900 dark:text-white font-outfit">Schedule an Event</h2>
                  
                  <form onSubmit={handleAddEvent} className="space-y-4">
                    <div>
                      <label className="block text-xs font-bold text-slate-500 uppercase mb-1">Event Name</label>
                      <input
                        type="text"
                        placeholder="e.g. Code Sync Session..."
                        value={newEventTitle}
                        onChange={(e) => setNewEventTitle(e.target.value)}
                        className="w-full px-3 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl text-sm"
                      />
                    </div>

                    <div className="grid grid-cols-2 gap-3">
                      <div>
                        <label className="block text-xs font-bold text-slate-500 uppercase mb-1">Date</label>
                        <input
                          type="text"
                          value={selectedDay}
                          disabled
                          className="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800/40 border border-slate-200 dark:border-slate-800 rounded-xl text-sm text-slate-500"
                        />
                      </div>
                      <div>
                        <label className="block text-xs font-bold text-slate-500 uppercase mb-1">Time</label>
                        <select
                          value={newEventTime}
                          onChange={(e) => setNewEventTime(e.target.value)}
                          className="w-full px-3 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl text-sm"
                        >
                          <option value="09:00 AM">09:00 AM</option>
                          <option value="10:00 AM">10:00 AM</option>
                          <option value="11:30 AM">11:30 AM</option>
                          <option value="02:00 PM">02:00 PM</option>
                          <option value="04:30 PM">04:30 PM</option>
                        </select>
                      </div>
                    </div>

                    <button type="submit" className="w-full py-2 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded-xl text-sm transition-all shadow-md">
                      Add to Calendar
                    </button>
                  </form>
                </div>

                {/* Calendar Sync Integrations */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <h2 className="text-lg font-bold mb-3 font-outfit text-slate-900 dark:text-white">Calendar Synchronization</h2>
                  <p className="text-xs text-slate-500 mb-4">Sync external agendas with your AI-assisted calendar scheduler.</p>
                  
                  <div className="space-y-3 mb-4">
                    <div className="flex items-center justify-between p-3 rounded-xl border border-slate-200 dark:border-slate-800 bg-white/40 dark:bg-slate-900/40 text-xs">
                      <div>
                        <p className="font-bold text-slate-950 dark:text-slate-50">Google Calendar</p>
                        <p className="text-[10px] text-slate-400">{isGoogleSynced ? 'Connected & Synced' : 'Disconnected'}</p>
                      </div>
                      <button
                        type="button"
                        onClick={handleToggleGoogleSync}
                        className={`px-3 py-1.5 rounded-lg text-[10px] font-bold transition-all ${
                          isGoogleSynced
                            ? 'bg-rose-100 dark:bg-rose-950/45 text-rose-700 dark:text-rose-300 hover:bg-rose-200'
                            : 'bg-blue-100 dark:bg-blue-950/45 text-blue-700 dark:text-blue-300 hover:bg-blue-200'
                        }`}
                      >
                        {isGoogleSynced ? 'Disconnect' : 'Connect'}
                      </button>
                    </div>

                    <div className="flex items-center justify-between p-3 rounded-xl border border-slate-200 dark:border-slate-800 bg-white/40 dark:bg-slate-900/40 text-xs">
                      <div>
                        <p className="font-bold text-slate-950 dark:text-slate-50">Microsoft Outlook</p>
                        <p className="text-[10px] text-slate-400">{isOutlookSynced ? 'Connected & Synced' : 'Disconnected'}</p>
                      </div>
                      <button
                        type="button"
                        onClick={handleToggleOutlookSync}
                        className={`px-3 py-1.5 rounded-lg text-[10px] font-bold transition-all ${
                          isOutlookSynced
                            ? 'bg-rose-100 dark:bg-rose-950/45 text-rose-700 dark:text-rose-300 hover:bg-rose-200'
                            : 'bg-blue-100 dark:bg-blue-950/45 text-blue-700 dark:text-blue-300 hover:bg-blue-200'
                        }`}
                      >
                        {isOutlookSynced ? 'Disconnect' : 'Connect'}
                      </button>
                    </div>
                  </div>

                  <div className="pt-2 border-t border-slate-200 dark:border-slate-850 flex items-center justify-between gap-2 text-xs">
                    <span className="text-[10px] text-slate-400">
                      {lastSyncTime ? `Last sync: ${lastSyncTime}` : 'Not synced yet'}
                    </span>
                    <button
                      type="button"
                      onClick={handleManualSync}
                      disabled={calendarSyncStatus === 'syncing'}
                      className="px-3.5 py-2 bg-slate-200 dark:bg-slate-800 hover:bg-slate-300 dark:hover:bg-slate-700 rounded-lg text-[10px] font-bold transition-all disabled:opacity-50"
                    >
                      {calendarSyncStatus === 'syncing' ? 'Syncing...' : 'Sync Now'}
                    </button>
                  </div>
                </div>

                {/* AI Schedule Optimization */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <h2 className="text-lg font-bold mb-3 font-outfit text-slate-900 dark:text-white">AI Schedule Advice</h2>
                  <div className="space-y-3 text-xs font-semibold text-slate-600 dark:text-slate-300">
                    <div className="p-3 rounded-lg bg-emerald-500/10 border border-emerald-500/20 text-emerald-800 dark:text-emerald-300">
                      "July 21st shows zero meetings. Highly recommend blocking 9:00 AM - 1:00 PM for focused deep learning architecture programming."
                    </div>
                    <div className="p-3 rounded-lg bg-amber-500/10 border border-amber-500/20 text-amber-800 dark:text-amber-300">
                      "You have back-to-back operations discussions on July 23rd. I suggest scheduling 15-minute breaks after each."
                    </div>
                  </div>
                </div>

              </div>

            </div>
          )}

          {activeTab === 'notes' && (
            <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
              
              {/* Notes sidebar / documents list */}
              <div className="lg:col-span-1 p-4 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm flex flex-col gap-4">
                <div className="flex justify-between items-center">
                  <h3 className="font-bold text-slate-900 dark:text-white font-outfit">Document Library</h3>
                </div>

                <div className="space-y-2 max-h-[400px] overflow-y-auto">
                  {filteredNotes.map((note) => (
                    <div
                      key={note.id}
                      onClick={() => setSelectedNoteId(note.id)}
                      className={`p-3 rounded-xl border text-xs font-semibold cursor-pointer transition-all relative ${
                        selectedNoteId === note.id
                          ? 'border-blue-500 bg-blue-50/20 dark:bg-blue-900/20'
                          : 'border-slate-200/40 dark:border-slate-800/40 hover:border-slate-300 dark:hover:border-slate-700 bg-white/40 dark:bg-slate-900/40'
                      }`}
                    >
                      <div className="flex justify-between items-center mb-1">
                        <span className="truncate pr-2 text-slate-900 dark:text-white">{note.title}</span>
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            deleteNote(note.id);
                          }}
                          className="text-slate-400 hover:text-rose-500"
                        >
                          ×
                        </button>
                      </div>
                      <div className="flex justify-between text-[10px] text-slate-400 mt-1.5">
                        <span>{note.folder}</span>
                        <span>{note.updatedAt}</span>
                      </div>
                    </div>
                  ))}
                </div>

                {/* Add Quick Document Folder details */}
                <div className="border-t border-slate-200 dark:border-slate-800 pt-4 space-y-2">
                  <input
                    type="text"
                    placeholder="New document name..."
                    value={newNoteTitle}
                    onChange={(e) => setNewNoteTitle(e.target.value)}
                    className="w-full px-3 py-1.5 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-xs"
                  />
                  <input
                    type="text"
                    placeholder="Folder (e.g. Planning)..."
                    value={newNoteFolder}
                    onChange={(e) => setNewNoteFolder(e.target.value)}
                    className="w-full px-3 py-1.5 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-xs"
                  />
                  <button
                    id={PAGE_IDS.addNoteBtn}
                    onClick={handleAddNote}
                    className="w-full py-2 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded-lg text-xs transition-all"
                  >
                    Create Note
                  </button>
                </div>
              </div>

              {/* Editor Workspace Column */}
              <div className="lg:col-span-3 p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm flex flex-col min-h-[450px]">
                {activeNote ? (
                  <div className="flex-1 flex flex-col">
                    <div className="flex justify-between items-center mb-4 border-b border-slate-200 dark:border-slate-800 pb-3">
                      <input
                        type="text"
                        value={activeNote.title}
                        onChange={(e) => handleNoteTitleChange(e.target.value)}
                        className="text-lg font-bold font-outfit bg-transparent border-none outline-none text-slate-900 dark:text-white w-full focus:ring-0"
                      />
                      <span className="text-xs font-semibold px-2 py-0.5 rounded-full bg-slate-100 dark:bg-slate-800 text-slate-500">
                        {activeNote.folder}
                      </span>
                    </div>

                    <textarea
                      id={PAGE_IDS.noteContent}
                      value={activeNote.content}
                      onChange={(e) => handleNoteContentChange(e.target.value)}
                      className="flex-1 w-full bg-transparent border-none outline-none resize-none text-sm text-slate-700 dark:text-slate-300 font-sans focus:ring-0 leading-relaxed"
                      placeholder="Start typing note details here..."
                    />
                  </div>
                ) : (
                  <div className="flex-1 flex items-center justify-center text-slate-400">
                    Create or select a document to start writing.
                  </div>
                )}
              </div>

            </div>
          )}

          {activeTab === 'automation' && (
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              
              {/* Active Workflows list */}
              <div className="lg:col-span-2 p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                <h2 className="text-xl font-bold mb-6 font-outfit text-slate-900 dark:text-white">Business Process Automation</h2>
                
                <div className="space-y-4">
                  {workflows.map((flow) => (
                    <div
                      key={flow.id}
                      className="p-5 rounded-xl border border-slate-200/80 dark:border-slate-800 bg-white/50 dark:bg-slate-900/50 flex items-start justify-between gap-4 transition-all"
                    >
                      <div className="space-y-2">
                        <div className="flex items-center gap-2">
                          <span className={`h-2.5 w-2.5 rounded-full ${flow.active ? 'bg-emerald-500 animate-pulse' : 'bg-slate-400'}`}></span>
                          <h3 className="font-bold text-sm text-slate-900 dark:text-white">{flow.name}</h3>
                        </div>
                        <p className="text-xs text-slate-500 leading-relaxed">{flow.description}</p>
                        
                        {/* Visualization trigger action */}
                        <div className="flex items-center gap-2 text-[10px] font-bold text-indigo-600 dark:text-indigo-400 bg-indigo-500/5 px-2.5 py-1 rounded-md w-fit mt-1 border border-indigo-500/10">
                          <span>{flow.trigger}</span>
                          <svg className="w-3 h-3 text-indigo-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M14 5l7 7m0 0l-7 7m7-7H3" />
                          </svg>
                          <span>{flow.action}</span>
                        </div>
                      </div>

                      <div className="flex items-center">
                        <button
                          id={PAGE_IDS.automationToggle(flow.id)}
                          onClick={() => toggleWorkflow(flow.id)}
                          className={`relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none ${
                            flow.active ? 'bg-blue-600' : 'bg-slate-200 dark:bg-slate-700'
                          }`}
                          role="switch"
                          aria-label={flow.name}
                          aria-checked={flow.active}
                        >
                          <span
                            className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                              flow.active ? 'translate-x-5' : 'translate-x-0'
                            }`}
                          />
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Graphic Connector Panel */}
              <div className="flex flex-col gap-6">
                
                {/* Visual connection indicator */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm text-center">
                  <h2 className="text-lg font-bold mb-4 font-outfit text-slate-900 dark:text-white">Active Integrations</h2>
                  
                  <div className="space-y-4">
                    <div className="p-3.5 rounded-xl border border-slate-200 dark:border-slate-800 bg-slate-50/50 dark:bg-slate-900/50 flex items-center justify-between">
                      <span className="text-xs font-semibold text-slate-700 dark:text-slate-300">Stripe Terminal API</span>
                      <span className="text-[10px] font-bold px-2 py-0.5 rounded-md bg-emerald-100 dark:bg-emerald-950 text-emerald-800 dark:text-emerald-300">Active</span>
                    </div>
                    <div className="p-3.5 rounded-xl border border-slate-200 dark:border-slate-800 bg-slate-50/50 dark:bg-slate-900/50 flex items-center justify-between">
                      <span className="text-xs font-semibold text-slate-700 dark:text-slate-300">Slack Webhooks</span>
                      <span className="text-[10px] font-bold px-2 py-0.5 rounded-md bg-emerald-100 dark:bg-emerald-950 text-emerald-800 dark:text-emerald-300">Active</span>
                    </div>
                    <div className="p-3.5 rounded-xl border border-slate-200 dark:border-slate-800 bg-slate-50/50 dark:bg-slate-900/50 flex items-center justify-between">
                      <span className="text-xs font-semibold text-slate-700 dark:text-slate-300">GitHub Webhooks</span>
                      <span className="text-[10px] font-bold px-2 py-0.5 rounded-md bg-slate-100 dark:bg-slate-800 text-slate-400 dark:text-slate-500">Idle</span>
                    </div>
                  </div>
                </div>

                {/* Workflow Templates */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <h2 className="text-lg font-bold mb-3 font-outfit text-slate-900 dark:text-white">Workflow Templates</h2>
                  <div className="space-y-2 text-xs font-semibold text-slate-600 dark:text-slate-300">
                    <button
                      onClick={() => showToast('Template loaded: Setup Abandoned Cart Dispatcher workflow.', 'success')}
                      className="w-full text-left p-3.5 rounded-xl border border-slate-200 hover:border-blue-500 transition-all dark:border-slate-800 bg-white/40 dark:bg-slate-900/40"
                    >
                      Abandoned Cart Automatic Recovery
                    </button>
                    <button
                      onClick={() => showToast('Template loaded: Setup Shift Reassignment automated notification workflow.', 'success')}
                      className="w-full text-left p-3.5 rounded-xl border border-slate-200 hover:border-blue-500 transition-all dark:border-slate-800 bg-white/40 dark:bg-slate-900/40"
                    >
                      Shift Reassignment Notifications
                    </button>
                  </div>
                </div>

              </div>

            </div>
          )}

          {activeTab === 'assistant' && (
            <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
              
              {/* Chat history main console */}
              <div className="lg:col-span-3 p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm flex flex-col min-h-[480px]">
                <h3 className="font-bold text-slate-900 dark:text-white font-outfit mb-4 border-b border-slate-200 dark:border-slate-800 pb-3">
                  AI Planning Assistant
                </h3>

                {/* Chat window */}
                <div className="flex-1 overflow-y-auto space-y-4 mb-4 max-h-[350px]">
                  {messages.map((m) => (
                    <div
                      key={m.id}
                      className={`flex gap-3 max-w-[80%] ${m.role === 'user' ? 'ml-auto flex-row-reverse' : ''}`}
                    >
                      <div className={`p-3.5 rounded-2xl text-sm leading-relaxed ${
                        m.role === 'user'
                          ? 'bg-blue-600 text-white rounded-tr-none'
                          : 'bg-slate-100 dark:bg-slate-800/80 text-slate-800 dark:text-slate-200 rounded-tl-none'
                      }`}>
                        <p>{m.content}</p>
                        <span className="block text-[9px] text-slate-400 mt-1 text-right">{m.timestamp}</span>
                      </div>
                    </div>
                  ))}
                  {isTyping && (
                    <div className="flex gap-2 items-center text-xs text-slate-400 font-semibold italic">
                      <span className="h-1.5 w-1.5 rounded-full bg-slate-400 animate-bounce"></span>
                      <span className="h-1.5 w-1.5 rounded-full bg-slate-400 animate-bounce delay-75"></span>
                      <span className="h-1.5 w-1.5 rounded-full bg-slate-400 animate-bounce delay-150"></span>
                      AI Planning helper is thinking...
                    </div>
                  )}
                </div>

                {/* Chat input box */}
                <form onSubmit={handleChatSubmit} className="flex gap-2 items-center">
                  <input
                    id={PAGE_IDS.chatInput}
                    type="text"
                    placeholder="Ask about project health, tasks optimization, or schedule recommendations..."
                    value={chatInput}
                    onChange={(e) => setChatInput(e.target.value)}
                    className="flex-1 px-4 py-2 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                  <button
                    id={PAGE_IDS.chatSubmit}
                    type="submit"
                    className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-xl text-sm font-bold transition-all shadow-md"
                  >
                    Send
                  </button>
                </form>
              </div>

              {/* Chat context and actions panel */}
              <div className="flex flex-col gap-6">
                
                {/* Team presence list */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm">
                  <h3 className="font-bold text-slate-900 dark:text-white font-outfit mb-3">Team Presence</h3>
                  <div className="space-y-3">
                    <div className="flex items-center gap-3 text-xs">
                      <div className="relative">
                        <div className="w-8 h-8 rounded-full bg-slate-200 flex items-center justify-center font-bold">AL</div>
                        <span className="absolute bottom-0 right-0 h-2.5 w-2.5 rounded-full bg-emerald-500 border-2 border-white"></span>
                      </div>
                      <div>
                        <p className="font-bold text-slate-800 dark:text-white">Alex Larson</p>
                        <p className="text-[10px] text-slate-400">Engineering Manager</p>
                      </div>
                    </div>
                    <div className="flex items-center gap-3 text-xs">
                      <div className="relative">
                        <div className="w-8 h-8 rounded-full bg-slate-200 flex items-center justify-center font-bold">SM</div>
                        <span className="absolute bottom-0 right-0 h-2.5 w-2.5 rounded-full bg-emerald-500 border-2 border-white"></span>
                      </div>
                      <div>
                        <p className="font-bold text-slate-800 dark:text-white">Sarah Miller</p>
                        <p className="text-[10px] text-slate-400">Product Designer</p>
                      </div>
                    </div>
                    <div className="flex items-center gap-3 text-xs">
                      <div className="relative">
                        <div className="w-8 h-8 rounded-full bg-slate-200 flex items-center justify-center font-bold">JH</div>
                        <span className="absolute bottom-0 right-0 h-2.5 w-2.5 rounded-full bg-slate-300 border-2 border-white"></span>
                      </div>
                      <div>
                        <p className="font-bold text-slate-800 dark:text-white">John Hanson</p>
                        <p className="text-[10px] text-slate-400">Operations Specialist</p>
                      </div>
                    </div>
                  </div>
                </div>

                {/* Team discussion comments box */}
                <div className="p-6 rounded-2xl border border-white/20 dark:border-slate-800/40 bg-white/60 dark:bg-slate-900/60 backdrop-blur-md shadow-sm text-xs">
                  <h3 className="font-bold text-slate-900 dark:text-white font-outfit mb-3">Activity Stream Feed</h3>
                  <div className="space-y-3">
                    <div className="p-3 bg-slate-50/50 dark:bg-slate-900/50 rounded-lg">
                      <p className="font-bold">Alex Larson <span className="font-normal text-slate-400">commented</span></p>
                      <p className="text-[11px] text-slate-600 dark:text-slate-400 mt-1">"The deep learning architecture note matches our latest sprint plans."</p>
                    </div>
                    <div className="p-3 bg-slate-50/50 dark:bg-slate-900/50 rounded-lg">
                      <p className="font-bold">Sarah Miller <span className="font-normal text-slate-400">completed task</span></p>
                      <p className="text-[11px] text-slate-600 dark:text-slate-400 mt-1">"Schedule weekly client feedback sessions."</p>
                    </div>
                  </div>
                </div>

              </div>

            </div>
          )}
        </div>

      </div>
    </AppShell>
  );
}
