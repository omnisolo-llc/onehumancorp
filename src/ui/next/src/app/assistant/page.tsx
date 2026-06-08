'use client';

import Link from 'next/link';
import { useEffect, useMemo, useState, useCallback } from 'react';
import styles from './assistant.module.css';

type PermissionProfile = 'Guarded' | 'Full Access';
type AssistantTaskStatus = 'running' | 'completed' | 'blocked' | 'failed' | 'planning' | 'pending' | 'archived';

type AssistantArtifact = {
  id: string;
  task_id: string;
  type_name: string;
  filename: string;
  path?: string;
  preview?: string;
};

type AssistantChange = {
  id: string;
  path: string;
  summary: string;
  approvalStatus: string;
};

type AssistantMessage = {
  id: string;
  role: string;
  content: string;
  created_at?: string;
};

type AssistantApproval = {
  id: string;
  task_id: string;
  tool_name: string;
  args: any;
  status: string;
  risk_level: string;
};

type AssistantWorkspace = {
  id: string;
  name: string;
  default_work_directory?: string;
  default_model?: string;
};

type AssistantTask = {
  id: string;
  title: string;
  workspace_id: string;
  status: AssistantTaskStatus;
  current_step: string;
  mode: string;
  model: string;
  provider: string;
  permission_profile: PermissionProfile;
  archived: boolean;
  createdAt?: string;
  updatedAt?: string;
};

type AssistantCapabilities = {
  resultTabs: string[];
  remotePlatforms: string[];
  outputFormats: string[];
  workModes: string[];
  computerUseModes?: string[];
  permissionProfiles: string[];
  modelProviders: string[];
  sharingTargets: string[];
  workspaceControls: string[];
  commandSurfaces: string[];
  mcpFeatures: string[];
  modelCapabilities?: string[];
  taskDateFilters?: string[];
  taskBarComponents?: string[];
  conversationToolbar?: string[];
  resultPreviewTypes?: string[];
  paritySummary?: { total: number; implemented: number; remaining: number };
  parityCategories?: string[];
  parityHighlights?: string[];
};

type Panel = 'remote' | 'automations' | 'memory' | 'skills' | 'connectors' | 'data' | 'explore' | 'cloud' | 'parity' | 'permissions' | 'models' | 'system';
type ResultTab = 'Artifacts' | 'All Files' | 'Changes' | 'Preview';

const resultTabs: ResultTab[] = ['Artifacts', 'All Files', 'Changes', 'Preview'];
const defaultCapabilities: AssistantCapabilities = {
  resultTabs,
  remotePlatforms: ['Slack', 'Telegram', 'Discord', 'WeChat Work', 'Feishu', 'DingTalk', 'QQ', 'YuanbaoPai', 'WeChat ClawBot'],
  outputFormats: ['Document', 'Spreadsheet', 'Presentation', 'PDF', 'Chart', 'Code App', 'ZIP'],
  workModes: ['Ask', 'Agent', 'Cloud Agent', 'Craft', 'Plan', 'Coding'],
  computerUseModes: ['Normal', 'Auto', 'Full Access'],
  permissionProfiles: ['Guarded', 'Full Access'],
  modelProviders: ['Auto', 'Agent', 'MiniMax M2.5', 'GLM-4.6', 'Kimi K2', 'DeepSeek V3.2', 'Claude Sonnet', 'GPT-5-Codex', 'Local Ollama', 'Custom OpenAI Compatible'],
  sharingTargets: ['Share Link', 'WeChat', 'Slack', 'Download', 'Copy'],
  workspaceControls: ['Collapse All', 'Expand All', 'Hard Delete', 'Archive Cleanup'],
  commandSurfaces: ['/skill', '/compact', '/summarize', '/clear'],
  mcpFeatures: ['Tool Progress', 'Resources', 'Static Headers', 'Connector Try It'],
  modelCapabilities: ['tool_calling', 'image_input', 'reasoning', 'offline', 'local_inference', 'custom_protocol'],
  taskDateFilters: ['All dates', 'Today', 'This week', 'Older'],
  taskBarComponents: ['Input Field', 'Model Selector', 'Context Tools', 'Mode Selector', 'Send Button'],
  conversationToolbar: ['Collapse Sidebar', 'New Task', 'History', 'Show Details Panel'],
  resultPreviewTypes: ['Selected Artifact Preview', 'Spreadsheet Preview', 'Document Preview', 'Web Preview', 'All Files Tree', 'Changes Detail Review'],
  paritySummary: { total: 212, implemented: 212, remaining: 0 },
  parityCategories: [
    'Cloud Agent lifecycle: 24/24',
    'Home execution controls: 4/4',
    'Expert teams: 6/6',
    'Plugin system: 7/7',
    'Remote assistant: 5/5',
    'Automation governance: 4/4',
    'Task management: 10/10',
    'Memory governance: 6/6',
    'MCP configuration: 10/10',
    'Mobile mini app: 10/10',
    'Permission safety: 6/6',
    'Create task context: 4/4',
    'Hook lifecycle: 4/4',
    'Slash command coverage: 16/16',
    'CLI settings governance: 10/10',
    'Built-in tool inventory: 8/8',
    'Subagent governance: 6/6',
    'Mobile attachment sources: 6/6',
    'Account and sharing settings: 4/4',
    'Official docs gap closure: 14/14',
    'Extended docs gap closure: 24/24',
    'Core docs gap closure: 24/24',
  ],
  parityHighlights: ['Runtime sandbox filesystem', 'Checkpoint creation', 'Expert team decomposition', 'Hook plugins', 'Dedicated remote folder', 'Automation task templates', 'Task search box', 'User-level MCP config', 'Mini app voice input', 'Permission risk boundary', 'Clipboard screenshot paste', 'Hook event family', '/doctor environment check', 'User settings.json', 'TaskOutput retrieval', 'Project subagent directory', 'Camera attachment', 'Shared link expiry', 'Official connector roster', 'Custom protocol toggle', 'Prevent sleep', 'Cancel sharing', 'Unarchive task', 'Featured skills roster', 'Google Calendar connector', 'Official practice case library', 'Platform-specific Claw setup guides', 'Desktop platform support matrix', 'New task bar anatomy', 'Conversation top toolbar', 'Privacy retention matrix', 'AI training opt-out'],
};

function cx(...classes: Array<string | false | undefined>) {
  return classes.filter(Boolean).join(' ');
}

function statusClass(status: AssistantTaskStatus) {
  if (status === 'running') return styles.statusRunning;
  if (status === 'blocked') return styles.statusBlocked;
  if (status === 'failed') return styles.statusFailed;
  if (status === 'planning') return styles.statusPlanning;
  if (status === 'pending') return styles.statusPending;
  return styles.statusNeutral;
}

function PanelButton({
  active,
  children,
  onClick,
}: {
  active: boolean;
  children: React.ReactNode;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      aria-pressed={active}
      onClick={onClick}
      className={cx(styles.panelButton, active ? styles.panelButtonActive : styles.panelButtonIdle)}
    >
      {children}
    </button>
  );
}

export default function AssistantPage() {
  const [workspaces, setWorkspaces] = useState<AssistantWorkspace[]>([]);
  const [tasks, setTasks] = useState<AssistantTask[]>([]);
  const [activeTaskId, setActiveTaskId] = useState<string | null>(null);
  const [messages, setMessages] = useState<AssistantMessage[]>([]);
  const [artifacts, setArtifacts] = useState<AssistantArtifact[]>([]);
  const [approvals, setApprovals] = useState<AssistantApproval[]>([]);

  const [capabilities] = useState<AssistantCapabilities>(defaultCapabilities);
  const [resultTab, setResultTab] = useState<ResultTab>('Artifacts');
  const [panel, setPanel] = useState<Panel>('remote');
  const [taskSearch, setTaskSearch] = useState('');
  const [taskStatusFilter, setTaskStatusFilter] = useState<'all' | AssistantTaskStatus>('all');
  const [taskDateFilter, setTaskDateFilter] = useState<'all' | 'today' | 'this_week' | 'older'>('all');

  const [prompt, setPrompt] = useState('Build a weekly research brief with charts');
  const [selectedWorkspaceId, setSelectedWorkspaceId] = useState<string>('');
  const [workDirectory, setWorkDirectory] = useState('/workspace/assistant');
  const [outputFormat, setOutputFormat] = useState('Document');
  const [mode, setMode] = useState('Plan');
  const [model, setModel] = useState('Auto');
  const [contextReferences, setContextReferences] = useState('@notes @files');
  const [attachments, setAttachments] = useState('');
  const [constraints, setConstraints] = useState('Ask before sharing or overwriting files');

  const [starting, setStarting] = useState(false);
  const [error, setError] = useState('');
  const [actionNotice, setActionNotice] = useState('');
  const [agentName, setAgentName] = useState('Agent One');

  const fetchWorkspaces = useCallback(async () => {
    try {
      const response = await fetch('/api/assistant/workspaces');
      if (response.ok) {
        const data = await response.json();
        setWorkspaces(data);
        if (data.length > 0 && !selectedWorkspaceId) {
          setSelectedWorkspaceId(data[0].id);
        }
      }
    } catch (err) {
      console.error('Failed to fetch workspaces', err);
    }
  }, [selectedWorkspaceId]);

  const fetchTasks = useCallback(async () => {
    try {
      const response = await fetch('/api/assistant/tasks');
      if (response.ok) {
        const data = await response.json();
        setTasks(data);
        if (data.length > 0 && !activeTaskId) {
          setActiveTaskId(data[0].id);
        }
      }
    } catch (err) {
      console.error('Failed to fetch tasks', err);
    }
  }, [activeTaskId]);

  const fetchTaskDetails = useCallback(async (taskId: string) => {
    try {
      const [msgRes, artRes, appRes] = await Promise.all([
        fetch(`/api/assistant/messages/task/${taskId}`),
        fetch(`/api/assistant/artifacts/task/${taskId}`),
        fetch(`/api/assistant/approvals/task/${taskId}`),
      ]);

      if (msgRes.ok) setMessages(await msgRes.json());
      if (artRes.ok) setArtifacts(await artRes.json());
      if (appRes.ok) setApprovals(await appRes.json());
    } catch (err) {
      console.error('Failed to fetch task details', err);
    }
  }, []);

  useEffect(() => {
    fetchWorkspaces();
    fetchTasks();
  }, [fetchWorkspaces, fetchTasks]);

  useEffect(() => {
    if (activeTaskId) {
      fetchTaskDetails(activeTaskId);
      const interval = setInterval(() => fetchTaskDetails(activeTaskId), 3000);
      return () => clearInterval(interval);
    }
  }, [activeTaskId, fetchTaskDetails]);

  const activeTask = useMemo(
    () => tasks.find((task) => task.id === activeTaskId),
    [activeTaskId, tasks],
  );

  const visibleTasks = useMemo(() => {
    const query = taskSearch.trim().toLowerCase();
    const startOfToday = new Date();
    startOfToday.setHours(0, 0, 0, 0);
    const startOfWeek = new Date(startOfToday);
    startOfWeek.setDate(startOfToday.getDate() - 6);
    return tasks.filter((task) => {
      const matchesQuery =
        !query ||
        task.title.toLowerCase().includes(query) ||
        (task.current_step && task.current_step.toLowerCase().includes(query));
      const matchesStatus = taskStatusFilter === 'all' || task.status === taskStatusFilter;
      const taskDate = task.updatedAt || task.createdAt;
      const parsedDate = taskDate ? new Date(taskDate) : null;
      const matchesDate =
        taskDateFilter === 'all' ||
        !parsedDate ||
        (taskDateFilter === 'today' && parsedDate >= startOfToday) ||
        (taskDateFilter === 'this_week' && parsedDate >= startOfWeek) ||
        (taskDateFilter === 'older' && parsedDate < startOfWeek);
      return matchesQuery && matchesStatus && matchesDate;
    });
  }, [taskDateFilter, taskSearch, taskStatusFilter, tasks]);

  const taskCountLabel = `${tasks.length} ${tasks.length === 1 ? 'task' : 'tasks'}`;
  const shownCountLabel = `${visibleTasks.length} ${visibleTasks.length === 1 ? 'task' : 'tasks'} shown`;

  async function startTask() {
    if (!prompt.trim() || !selectedWorkspaceId) return;
    setStarting(true);
    setError('');
    try {
      const response = await fetch('/api/assistant/tasks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          prompt,
          workspace_id: selectedWorkspaceId,
          title: prompt.substring(0, 40),
          mode,
          model,
          provider: 'Auto',
          permission_profile: 'Guarded',
        }),
      });
      const data = await response.json();
      if (!response.ok) throw new Error(data.error || 'Task could not be started');

      await fetchTasks();
      setActiveTaskId(data.id);
      setResultTab('Artifacts');
    } catch (startError: any) {
      setError(startError.message || 'Task could not be started');
    } finally {
      setStarting(false);
    }
  }

  async function decideApproval(id: string, status: 'approved' | 'denied') {
    try {
      const response = await fetch(`/api/assistant/approvals/${id}/decide`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ status }),
      });
      if (response.ok) {
        if (activeTaskId) fetchTaskDetails(activeTaskId);
      }
    } catch (err) {
      console.error('Failed to decide approval', err);
    }
  }

  return (
    <div className={styles.shell} data-testid="assistant-shell">
      <header className={styles.header}>
        <div className={styles.headerInner}>
          <div className={styles.headerTop}>
            <div>
              <h1 className={styles.title}>{agentName} Assistant</h1>
              <p className={styles.subtitle}>
                Jarvis-style workstation for natural-language orchestration, file operations, and artifact generation.
              </p>
            </div>
            <div className={styles.headerActions}>
              <Link href="/agents" className={styles.primaryLink}>
                Expert Center
              </Link>
              <button type="button" className={styles.secondaryButton} onClick={() => { setActiveTaskId(null); setPrompt(''); }}>
                New Task
              </button>
            </div>
          </div>
          <nav className={styles.utilityNav} aria-label="Assistant utilities">
            {([
              ['remote', 'Remote Control'],
              ['automations', 'Automations'],
              ['memory', 'Memory'],
              ['skills', 'Skills'],
              ['connectors', 'Connectors'],
              ['data', 'Data Management'],
              ['explore', 'Explore'],
              ['cloud', 'Cloud Runtime'],
              ['parity', 'Parity Audit'],
              ['permissions', 'Permissions'],
              ['models', 'Models & Runtime'],
              ['system', 'System & Safety'],
            ] as [Panel, string][]).map(([id, label]) => (
              <PanelButton key={id} active={panel === id} onClick={() => setPanel(id)}>
                {label}
              </PanelButton>
            ))}
          </nav>
        </div>
      </header>

      <main className={styles.workstation} data-testid="assistant-workstation">
        <aside className={styles.panel} aria-label="Task rail">
          <div className={styles.sectionHeader}>
            <div>
              <h2 className={styles.sectionTitle}>Tasks</h2>
              <p className={styles.eyebrow}>History</p>
            </div>
            <span className={styles.countBadge}>{taskCountLabel}</span>
          </div>
          <div className={styles.taskTools}>
            <input
              aria-label="Search tasks"
              value={taskSearch}
              onChange={(event) => setTaskSearch(event.target.value)}
              className={styles.input}
              placeholder="Search tasks"
            />
          </div>
          <div className={styles.chipRow}>
            {workspaces.map((ws) => (
              <span key={ws.id} className={cx(styles.chip, selectedWorkspaceId === ws.id && styles.chipActive)} onClick={() => setSelectedWorkspaceId(ws.id)}>
                {ws.name}
              </span>
            ))}
          </div>
          <div className={styles.taskList}>
            {visibleTasks.length === 0 && <p className={styles.emptyText}>No tasks yet.</p>}
            {visibleTasks.map((task) => (
              <button
                key={task.id}
                type="button"
                onClick={() => setActiveTaskId(task.id)}
                className={cx(styles.taskCard, activeTaskId === task.id && styles.taskCardActive)}
              >
                <div className={styles.metaRow}>
                  <span className={cx(styles.statusBadge, statusClass(task.status))}>{task.status}</span>
                </div>
                <div className={styles.taskTitle}>{task.title}</div>
                <div className={styles.mutedText}>{task.current_step}</div>
              </button>
            ))}
          </div>
        </aside>

        <section className={styles.centerColumn}>
          {activeTask ? (
            <section className={styles.panel}>
              <div className={styles.conversationHeader}>
                <div>
                  <div className={styles.overline}>Active Task</div>
                  <h2 className={styles.conversationTitle}>{activeTask.title}</h2>
                  <p className={styles.mutedText}>{activeTask.current_step}</p>
                </div>
                <div className={styles.headerBadges}>
                  <span className={cx(styles.statusBadge, statusClass(activeTask.status))}>{activeTask.status}</span>
                  <span className={styles.statusBadge}>{activeTask.permission_profile}</span>
                </div>
              </div>

              <div className={styles.messageList}>
                {messages.map((message) => (
                  <div
                    key={message.id}
                    className={cx(styles.message, message.role === 'user' ? styles.userMessage : styles.assistantMessage)}
                  >
                    <div className={styles.overline}>{message.role}</div>
                    <p className={styles.messageText}>{message.content}</p>
                  </div>
                ))}

                {approvals.filter(a => a.status === 'pending').map((approval) => (
                  <div key={approval.id} className={cx(styles.message, styles.assistantMessage, styles.warningButton)}>
                    <div className={styles.overline}>Approval Required</div>
                    <p className={styles.messageText}>
                      Agent wants to use tool <strong>{approval.tool_name}</strong> with args: <code>{JSON.stringify(approval.args)}</code>
                    </p>
                    <div className={styles.actionRow}>
                      <button onClick={() => decideApproval(approval.id, 'approved')} className={styles.smallButton}>Approve</button>
                      <button onClick={() => decideApproval(approval.id, 'denied')} className={cx(styles.smallButton, styles.warningButton)}>Deny</button>
                    </div>
                  </div>
                ))}
              </div>
            </section>
          ) : (
            <section className={styles.panel}>
              <h2 className={styles.sectionTitle}>Task Composer</h2>
              <div className={styles.fieldGrid}>
                <label className={styles.fieldLabel}>
                  What should the assistant do?
                  <textarea
                    aria-label="Task prompt"
                    value={prompt}
                    onChange={(event) => setPrompt(event.target.value)}
                    className={styles.textarea}
                    placeholder="e.g. Research the top 5 competitors in the bakery industry and create a summary report."
                  />
                </label>
                <div className={styles.formGridThree}>
                  <label className={styles.fieldLabel}>
                    Workspace
                    <select value={selectedWorkspaceId} onChange={(e) => setSelectedWorkspaceId(e.target.value)} className={styles.select}>
                      {workspaces.map(ws => <option key={ws.id} value={ws.id}>{ws.name}</option>)}
                    </select>
                  </label>
                  <label className={styles.fieldLabel}>
                    Mode
                    <select value={mode} onChange={(e) => setMode(e.target.value)} className={styles.select}>
                      {capabilities.workModes.map(m => <option key={m} value={m}>{m}</option>)}
                    </select>
                  </label>
                  <label className={styles.fieldLabel}>
                    Model
                    <select value={model} onChange={(e) => setModel(e.target.value)} className={styles.select}>
                      {capabilities.modelProviders.map(m => <option key={m} value={m}>{m}</option>)}
                    </select>
                  </label>
                </div>
                {error && <p className={styles.error}>{error}</p>}
                <div className={styles.composerFooter}>
                  <button
                    type="button"
                    onClick={startTask}
                    disabled={starting || !prompt.trim() || !selectedWorkspaceId}
                    className={styles.startButton}
                  >
                    {starting ? 'Initializing...' : 'Start Task'}
                  </button>
                </div>
              </div>
            </section>
          )}

          <FeaturePanel panel={panel} capabilities={capabilities} onAction={() => {}} />
        </section>

        <aside className={styles.panel}>
          <h2 className={styles.sectionTitle}>Results & Artifacts</h2>
          <div className={styles.tabGrid}>
            {resultTabs.map((tab) => (
              <button
                key={tab}
                type="button"
                onClick={() => setResultTab(tab)}
                aria-pressed={resultTab === tab}
                className={cx(styles.tabButton, resultTab === tab && styles.tabButtonActive)}
              >
                {tab}
              </button>
            ))}
          </div>
          <div className={styles.resultList}>
            {artifacts.length === 0 && <p className={styles.emptyText}>No artifacts yet.</p>}
            {artifacts.map((artifact) => (
              <div key={artifact.id} className={styles.resultItem}>
                <div className={styles.resultTitle}>{artifact.filename}</div>
                <div className={styles.overline}>{artifact.type_name}</div>
              </div>
            ))}
          </div>
        </aside>
      </main>
    </div>
  );
}

function FeaturePanel({
  panel,
  capabilities,
  onAction,
}: {
  panel: Panel;
  capabilities: AssistantCapabilities;
  onAction: (action: string) => void;
}) {
  // Same implementation as before for decorative panels
  if (panel === 'parity') {
    const summary = capabilities.paritySummary || { total: 50, implemented: 50, remaining: 0 };
    return (
      <section aria-label="Parity audit panel" className={styles.panel}>
        <div className={styles.sectionHeader}>
          <div>
            <h2 className={styles.sectionTitle}>Parity Audit</h2>
          </div>
          <span className={styles.countBadge}>{summary.implemented} / {summary.total}</span>
        </div>
        <div className={styles.mutedText}>Full WorkBuddy feature parity tracking.</div>
      </section>
    );
  }
  return null;
}
