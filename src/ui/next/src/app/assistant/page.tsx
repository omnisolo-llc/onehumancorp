'use client';

import Link from 'next/link';
import { useEffect, useMemo, useState } from 'react';
import styles from './assistant.module.css';

type PermissionProfile = 'Guarded' | 'Full Access';
type AssistantTaskStatus = 'running' | 'completed' | 'blocked' | 'failed' | 'planning' | 'pending' | 'archived';

type AssistantArtifact = {
  id: string;
  type: string;
  filename: string;
  preview: string;
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
};

type AssistantAction = {
  id: string;
  label: string;
  kind: string;
  approvalRequired: boolean;
};

type AssistantTask = {
  id: string;
  title: string;
  workspace: string;
  status: AssistantTaskStatus;
  currentStep: string;
  mode: string;
  model: string;
  provider: string;
  permissionProfile: PermissionProfile;
  riskSummary: string[];
  artifacts: AssistantArtifact[];
  changes: AssistantChange[];
  messages: AssistantMessage[];
  actions?: AssistantAction[];
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

type Panel = 'remote' | 'automations' | 'memory' | 'skills' | 'connectors' | 'data' | 'explore' | 'cloud' | 'parity' | 'permissions' | 'models' | 'system' | 'billing';
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

const fallbackTasks: AssistantTask[] = [
  {
    id: 'fallback-task',
    title: 'Create a personal briefing',
    workspace: 'Personal OS',
    status: 'running',
    currentStep: 'Ready to plan',
    mode: 'Ask',
    model: 'Auto',
    provider: 'Auto',
    permissionProfile: 'Guarded',
    riskSummary: ['Guarded mode is active'],
    artifacts: [],
    changes: [],
    messages: [
      {
        id: 'fallback-message',
        role: 'assistant',
        content: 'Agent is ready.',
      },
    ],
    actions: [
      { id: 'fallback-action-stop', label: 'Stop', kind: 'control', approvalRequired: false },
      { id: 'fallback-action-preview', label: 'Open Preview', kind: 'preview', approvalRequired: false },
      { id: 'fallback-action-run', label: 'Run Locally', kind: 'execute', approvalRequired: true },
    ],
  },
];

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
  const [tasks, setTasks] = useState<AssistantTask[]>(fallbackTasks);
  const [capabilities, setCapabilities] = useState<AssistantCapabilities>(defaultCapabilities);
  const [activeTaskId, setActiveTaskId] = useState(fallbackTasks[0].id);
  const [resultTab, setResultTab] = useState<ResultTab>('Artifacts');
  const [panel, setPanel] = useState<Panel>('remote');
  const [taskSearch, setTaskSearch] = useState('');
  const [taskStatusFilter, setTaskStatusFilter] = useState<'all' | AssistantTaskStatus>('all');
  const [taskDateFilter, setTaskDateFilter] = useState<'all' | 'today' | 'this_week' | 'older'>('all');
  const [prompt, setPrompt] = useState('Build a weekly research brief with charts');
  const [workspace, setWorkspace] = useState('Personal OS');
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
  const [billing, setBilling] = useState<any>({});


  useEffect(() => {
    let mounted = true;

    async function loadTasks() {
      try {
        const response = await fetch('/api/assistant/tasks');
        if (!response.ok) throw new Error('Assistant tasks unavailable');
        const data = await response.json();
        const loadedTasks: AssistantTask[] = data.tasks?.length ? data.tasks : fallbackTasks;
        if (!mounted) return;
        setCapabilities(data.capabilities || defaultCapabilities);
        setTasks(loadedTasks);
        setActiveTaskId(loadedTasks[0]?.id || '');
      } catch (loadError: any) {
        if (!mounted) return;
        setError(loadError.message || 'Assistant tasks unavailable');
        setTasks(fallbackTasks);
        setActiveTaskId(fallbackTasks[0].id);
      }
    }

    async function loadSettings() {
      try {
        const response = await fetch('/api/assistant/settings');
        if (response.ok) {
          const data = await response.json();
          if (data?.settings?.agentName) {
            setAgentName(data.settings.agentName);
          }
        }
      } catch (err) {
        console.error(err);
      }
    }

    async function loadBilling() {
      try {
        const response = await fetch('/api/assistant/billing');
        if (response.ok) {
          const data = await response.json();
          if (mounted) {
            setBilling(data);
          }
        }
      } catch (err) {
        console.error(err);
      }
    }

    Promise.all([loadTasks(), loadSettings(), loadBilling()]);
    return () => {
      mounted = false;
    };
  }, []);

  const activeTask = useMemo(
    () => tasks.find((task) => task.id === activeTaskId) || tasks[0] || fallbackTasks[0],
    [activeTaskId, tasks],
  );

  const workspaces = useMemo(() => Array.from(new Set(tasks.map((task) => task.workspace))), [tasks]);
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
        task.workspace.toLowerCase().includes(query) ||
        task.currentStep.toLowerCase().includes(query);
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
    if (!prompt.trim()) return;
    setStarting(true);
    setError('');
    try {
      const response = await fetch('/api/assistant/tasks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          prompt,
          workspace,
          mode,
          model,
          provider: 'Auto',
          workDirectory,
          outputFormat,
          constraints,
          contextReferences,
          attachments: attachments.split(',').map((item) => item.trim()).filter(Boolean),
          skills: ['Web Research', 'Document Writer', 'Chart Builder'],
          connectors: ['Google Drive', 'Slack'],
          permissionProfile: 'Guarded',
        }),
      });
      const data = await response.json();
      if (!response.ok) throw new Error(data.error || 'Task could not be started');
      setTasks((current) => [data.task, ...current.filter((task) => task.id !== data.task.id)]);
      setActiveTaskId(data.task.id);
      setResultTab('Artifacts');
    } catch (startError: any) {
      setError(startError.message || 'Task could not be started');
    } finally {
      setStarting(false);
    }
  }

  async function runApiAction(path: string, method: 'POST' | 'PATCH', body: unknown, successMessage: string) {
    setError('');
    setActionNotice('');
    try {
      const response = await fetch(path, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      });
      const data = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(data.error || 'Action failed');
      setActionNotice(successMessage);
      return data;
    } catch (actionError: any) {
      setError(actionError.message || 'Action failed');
      return null;
    }
  }

  async function runResultAction(action: string) {
    const artifact = activeTask.artifacts[0];
    if (action.startsWith('Share') && artifact) {
      const target = action.replace('Share to ', '').replace('Share Link', 'Share Link');
      await runApiAction('/api/assistant/share', 'POST', {
        taskId: activeTask.id,
        artifactId: artifact.id,
        target,
      }, `${action} queued`);
      return;
    }
    if (action === 'Open External Preview' && artifact) {
      await runApiAction('/api/assistant/previews', 'PATCH', {
        action: 'open_external',
        artifactId: artifact.id,
      }, 'Preview opened externally');
      return;
    }
    if (action.startsWith('Export ') && artifact) {
      await runApiAction('/api/assistant/artifacts', 'POST', {
        taskId: activeTask.id,
        outputFormat: action.replace('Export ', ''),
        title: activeTask.title,
      }, `${action} created`);
      return;
    }
    setActionNotice(`${action} ready`);
  }

  async function runFeatureAction(action: string) {
    if (action === 'upload_remote_file') {
      await runApiAction('/api/assistant/uploads', 'POST', {
        platform: 'WeChat ClawBot',
        userId: 'agent-user',
        filename: 'remote-upload.png',
        mimeType: 'image/png',
        sizeBytes: 2048,
        previewText: 'Remote image upload',
      }, 'Remote upload added');
    } else if (action === 'summon_expert') {
      await runApiAction('/api/assistant/experts', 'PATCH', {
        action: 'summon',
        id: 'expert-research-strategist',
        taskId: activeTask.id,
      }, 'Expert summoned');
    } else if (action === 'summarize') {
      await runApiAction('/api/assistant/commands', 'POST', {
        command: '/summarize',
        taskId: activeTask.id,
      }, 'Summary command complete');
    } else if (action === 'try_mcp') {
      await runApiAction('/api/assistant/mcp', 'PATCH', {
        action: 'try_tool',
        name: 'MCP Endpoint',
        tool: 'read_resource',
      }, 'MCP tool completed');
    } else if (action === 'collapse_workspaces') {
      await runApiAction('/api/assistant/workspaces', 'PATCH', {
        action: 'collapse_all',
      }, 'Workspaces collapsed');
    } else if (action === 'copy_share_link') {
      const artifact = activeTask.artifacts[0];
      if (!artifact) throw new Error('No artifact available to share');
      const data = await runApiAction('/api/assistant/share', 'POST', {
        taskId: activeTask.id,
        artifactId: artifact.id,
        target: 'Share Link',
      }, 'Share link created');
      if (data?.share?.id) {
        await runApiAction('/api/assistant/share', 'PATCH', {
          action: 'copy_link',
          id: data.share.id,
        }, 'Share link copied');
      }
    } else if (action === 'download_shared_file') {
      const artifact = activeTask.artifacts[0];
      if (!artifact) throw new Error('No artifact available to download');
      await runApiAction('/api/assistant/share', 'POST', {
        taskId: activeTask.id,
        artifactId: artifact.id,
        target: 'Download',
      }, 'Download prepared');
    } else if (action === 'cancel_sharing') {
      const artifact = activeTask.artifacts[0];
      if (!artifact) throw new Error('No artifact available to revoke');
      const data = await runApiAction('/api/assistant/share', 'POST', {
        taskId: activeTask.id,
        artifactId: artifact.id,
        target: 'Share Link',
      }, 'Share link staged');
      if (data?.share?.id) {
        await runApiAction('/api/assistant/share', 'PATCH', {
          action: 'revoke',
          id: data.share.id,
        }, 'Sharing canceled');
      }
    } else if (action === 'unarchive_task') {
      await runApiAction(`/api/assistant/tasks/${activeTask.id}`, 'PATCH', {
        action: activeTask.status === 'archived' ? 'unarchive' : 'archive',
      }, activeTask.status === 'archived' ? 'Task unarchived' : 'Task archived');
    } else if (action === 'save_custom_model') {
      await runApiAction('/api/assistant/models', 'PATCH', {
        action: 'upsert',
        provider: 'Custom OpenAI Compatible',
        modelId: 'agent-custom',
        endpoint: 'https://models.example.test/v1',
        parameters: { temperature: 0.2, reasoningEffort: 'medium' },
        skipChatCompletions: true,
        customProtocol: true,
        capabilities: ['tool_calling', 'reasoning'],
      }, 'Custom model saved');
    } else if (action === 'remix_template') {
      await runApiAction('/api/assistant/explore', 'POST', {
        templateId: 'explore-investor-update',
        workspace,
        ownerGoal: 'Turn this community workflow into my Agent agent.',
      }, 'Template remixed');
    } else if (action === 'share_exploration') {
      await runApiAction('/api/assistant/explore', 'PATCH', {
        action: 'share',
        remixId: 'remix-1',
        target: 'Share Link',
      }, 'Exploration shared');
    } else if (action === 'start_cloud_session') {
      await runApiAction('/api/assistant/cloud', 'POST', {
        prompt,
        workspace,
        model,
        files: attachments.split(',').map((item) => item.trim()).filter(Boolean),
        screenshot: 'assistant-screenshot.png',
      }, 'Cloud session started');
    } else if (action === 'pause_cloud_session') {
      await runApiAction('/api/assistant/cloud', 'PATCH', {
        action: 'pause',
        id: 'cloud-1',
      }, 'Cloud session paused');
    } else if (action === 'install_plugin') {
      await runApiAction('/api/assistant/plugins', 'PATCH', {
        action: 'install',
        id: 'plugin-office-suite',
      }, 'Office Suite installed');
    } else if (action === 'connect_claw') {
      await runApiAction('/api/assistant/claw', 'PATCH', {
        action: 'connect',
        platform: 'Slack',
        credentials: { appId: 'A123', botToken: 'xoxb-token' },
      }, 'Slack Claw connected');
    } else if (action === 'create_approval') {
      await runApiAction('/api/assistant/approvals', 'POST', {
        taskId: activeTask.id,
        action: 'external_send',
        summary: 'Send artifact outside Agent',
        riskLevel: 'high',
      }, 'High-risk approval created');
    } else if (action === 'save_ui_settings') {
      await runApiAction('/api/assistant/settings', 'PATCH', {
        fontSize: 'large',
        systemLanguage: 'en-US',
        contentFilter: 'hide_filtered_answer',
        compactMode: false,
        preventSleep: true,
      }, 'UI settings saved');
    } else if (action === 'upload_logs') {
      await runApiAction('/api/assistant/support', 'POST', {
        kind: 'upload_logs',
        message: 'User feedback from assistant surface',
        includeLogs: true,
        screenshot: 'assistant-feedback.png',
      }, 'Logs uploaded');
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
                Natural-language workstation for tasks, files, artifacts, remote control, automations, memory, skills, and connectors.
              </p>
            </div>
            <div className={styles.headerActions}>
              <Link href="/agents" className={styles.primaryLink}>
                Expert Center
              </Link>
              <button type="button" className={styles.secondaryButton}>
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
              ['billing', 'My Plan'],
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
              <h2 className={styles.sectionTitle}>Task List</h2>
              <p className={styles.eyebrow}>Workspaces</p>
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
            <select
              aria-label="Task status filter"
              value={taskStatusFilter}
              onChange={(event) => setTaskStatusFilter(event.target.value as 'all' | AssistantTaskStatus)}
              className={styles.select}
            >
              <option value="all">All statuses</option>
              <option value="running">Running</option>
              <option value="completed">Completed</option>
              <option value="blocked">Blocked</option>
              <option value="failed">Failed</option>
              <option value="planning">Planning</option>
              <option value="pending">Pending</option>
              <option value="archived">Archived</option>
            </select>
            <select
              aria-label="Task date filter"
              value={taskDateFilter}
              onChange={(event) => setTaskDateFilter(event.target.value as 'all' | 'today' | 'this_week' | 'older')}
              className={styles.select}
            >
              <option value="all">All dates</option>
              <option value="today">Today</option>
              <option value="this_week">This week</option>
              <option value="older">Older</option>
            </select>
          </div>
          <div className={styles.chipRow}>
            {workspaces.map((name) => (
              <span key={name} className={styles.chip}>
                {name}
              </span>
            ))}
          </div>
          <div className={styles.filterMeta}>
            <span>{shownCountLabel}</span>
            <button
              type="button"
              onClick={() => {
                setTaskSearch('');
                setTaskStatusFilter('all');
                setTaskDateFilter('all');
              }}
              className={styles.inlineButton}
            >
              Reset task filters
            </button>
          </div>
          <div className={styles.taskList}>
            {visibleTasks.length === 0 && <p className={styles.emptyText}>No matching tasks.</p>}
            {visibleTasks.map((task) => (
              <button
                key={task.id}
                type="button"
                onClick={() => setActiveTaskId(task.id)}
                className={cx(styles.taskCard, activeTask.id === task.id && styles.taskCardActive)}
              >
                <div className={styles.metaRow}>
                  <span className={styles.overline}>{task.workspace}</span>
                  <span className={cx(styles.statusBadge, statusClass(task.status))}>{task.status}</span>
                </div>
                <div className={styles.taskTitle}>{task.title}</div>
                <div className={styles.mutedText}>{task.currentStep}</div>
              </button>
            ))}
          </div>
        </aside>

        <section className={styles.centerColumn}>
          <section className={styles.panel}>
            <div className={styles.conversationHeader}>
              <div>
                <div className={styles.overline}>Conversation</div>
                <h2 className={styles.conversationTitle}>{activeTask.title}</h2>
                <p className={styles.mutedText}>{activeTask.currentStep}</p>
              </div>
              <div className={styles.headerBadges}>
                <span className={cx(styles.statusBadge, statusClass(activeTask.status))}>{activeTask.status}</span>
                <span className={styles.statusBadge}>
                  {activeTask.permissionProfile}
                </span>
              </div>
            </div>
            <div className={styles.featureGridThree}>
              {[
                ['Collapse Sidebar', 'Hide the task rail for a cleaner conversation view.'],
                ['History', 'Jump through previous messages and execution checkpoints.'],
                ['Show Details Panel', 'Open artifacts, files, changes, and browser preview beside the chat.'],
                ['File & Image Upload', 'Paste screenshots, drag files, or upload context into the thread.'],
                ['Execution Progress', 'Show stage descriptions, intermediate steps, and result summaries.'],
                ['Interrupt & Resume', 'Stop a running task, add instructions, and continue with context.'],
              ].map(([title, detail]) => (
                <div key={title} className={styles.resultItem}>
                  <div className={styles.resultTitle}>{title}</div>
                  <div className={styles.cardDetail}>{detail}</div>
                </div>
              ))}
            </div>
            <div className={styles.messageList}>
              {activeTask.messages.map((message) => (
                <div
                  key={message.id}
                  className={cx(styles.message, message.role === 'user' ? styles.userMessage : styles.assistantMessage)}
                >
                  <div className={styles.overline}>{message.role}</div>
                  <p className={styles.messageText}>{message.content}</p>
                </div>
              ))}
            </div>
            <div className={styles.actionRow}>
              {(activeTask.actions || []).map((action) => (
                <button
                  key={action.id}
                  type="button"
                  className={cx(styles.smallButton, action.approvalRequired ? styles.warningButton : styles.neutralButton)}
                >
                  {action.label}
                </button>
              ))}
            </div>
          </section>

          <section className={styles.panel}>
            <h2 className={styles.sectionTitle}>Task Composer</h2>
            <div className={styles.featureGridThree}>
              {[
                ['Input Field', 'Natural-language task description.'],
                ['Model Selector', 'Choose from available AI models.'],
                ['Context Tools', '@ references, files, screenshots, and details.'],
                ['Mode Selector', 'Switch between work and coding task modes.'],
                ['Send Button', 'Launch the task or press Enter.'],
                ['One-sentence Assignment', 'Start with a concise description and let Agent plan.'],
                ['Default Directory', 'Use the default task output folder when none is selected.'],
                ['Parallel Work', 'Create more tasks or follow-ups while execution continues.'],
              ].map(([title, detail]) => (
                <div key={title} className={styles.resultItem}>
                  <div className={styles.resultTitle}>{title}</div>
                  <div className={styles.cardDetail}>{detail}</div>
                </div>
              ))}
            </div>
            <div className={styles.fieldGrid}>
              <label className={styles.fieldLabel}>
                Task prompt
                <textarea
                  aria-label="Task prompt"
                  value={prompt}
                  onChange={(event) => setPrompt(event.target.value)}
                  className={styles.textarea}
                />
              </label>
              <div className={styles.formGridThree}>
                <label className={styles.fieldLabel}>
                  Workspace
                  <input
                    aria-label="Workspace"
                    value={workspace}
                    onChange={(event) => setWorkspace(event.target.value)}
                    className={styles.input}
                  />
                </label>
                <label className={styles.fieldLabel}>
                  Work directory
                  <input
                    aria-label="Work directory"
                    value={workDirectory}
                    onChange={(event) => setWorkDirectory(event.target.value)}
                    className={styles.input}
                  />
                </label>
                <label className={styles.fieldLabel}>
                  Output format
                  <select
                    aria-label="Output format"
                    value={outputFormat}
                    onChange={(event) => setOutputFormat(event.target.value)}
                    className={styles.select}
                  >
                    {capabilities.outputFormats.map((option) => (
                      <option key={option}>{option}</option>
                    ))}
                  </select>
                </label>
              </div>
              <div className={styles.formGridFour}>
                <label className={styles.fieldLabel}>
                  Mode
                  <select aria-label="Mode" value={mode} onChange={(event) => setMode(event.target.value)} className={styles.select}>
                    {capabilities.workModes.map((option) => (
                      <option key={option}>{option}</option>
                    ))}
                  </select>
                </label>
                <label className={styles.fieldLabel}>
                  Model
                  <select aria-label="Model" value={model} onChange={(event) => setModel(event.target.value)} className={styles.select}>
                    {capabilities.modelProviders.map((option) => (
                      <option key={option}>{option}</option>
                    ))}
                  </select>
                </label>
                <label className={styles.fieldLabel}>
                  Context
                  <input value={contextReferences} onChange={(event) => setContextReferences(event.target.value)} className={styles.input} />
                </label>
                <label className={styles.fieldLabel}>
                  Attachments
                  <input value={attachments} onChange={(event) => setAttachments(event.target.value)} className={styles.input} />
                </label>
              </div>
              <label className={styles.fieldLabel}>
                Constraints
                <input value={constraints} onChange={(event) => setConstraints(event.target.value)} className={styles.input} />
              </label>
              {error && <p className={styles.error}>{error}</p>}
              <div className={styles.composerFooter}>
                <div className={styles.quickChips}>
                  {['@ files', 'Screenshot', 'Guarded', 'Parallel tasks'].map((item) => (
                    <span key={item} className={styles.formatChip}>{item}</span>
                  ))}
                  <button type="button" className={styles.smallButton} aria-label="Clipboard screenshot paste" title="Paste screenshot from clipboard">
                    📋 Paste Image
                  </button>
                  {outputFormat === 'Code App' && (
                    <>
                      <button type="button" className={styles.smallButton}>
                        Open Preview
                      </button>
                      <button type="button" className={cx(styles.smallButton, styles.warningButton)}>
                        Run Locally
                      </button>
                    </>
                  )}
                </div>
                <button
                  type="button"
                  onClick={startTask}
                  disabled={starting || !prompt.trim()}
                  className={styles.startButton}
                >
                  {starting ? 'Starting...' : 'Start Task'}
                </button>
              </div>
            </div>
          </section>

          {actionNotice && <div className={styles.resultItem} role="status">{actionNotice}</div>}
          <FeaturePanel panel={panel} capabilities={capabilities} onAction={runFeatureAction} billing={billing} />
        </section>

        <aside className={styles.panel}>
          <h2 className={styles.sectionTitle}>Results Panel</h2>
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
          <ResultContent task={activeTask} tab={resultTab} />
          <div className={styles.resultList}>
            <div className={styles.resultItem}>
              <div className={styles.resultTitle}>Preview Auto Refresh</div>
              <div className={styles.cardDetail}>Generated files and web previews refresh when artifacts change.</div>
            </div>
            {[
              ['Selected Artifact Preview', 'Artifact list and preview area stay side by side for review.'],
              ['Spreadsheet Preview', 'Inspect headers, data completeness, and formatting before downloading.'],
              ['Document Preview', 'Review headings, layout, and body text before delivery.'],
              ['Web Preview Controls', 'Track URL, refresh, and rendered page content for local apps.'],
              ['All Files Tree', 'Browse workspace files with open-file tabs and content panes.'],
              ['Changes Detail Review', 'Inspect modified files and change contents before accepting.'],
            ].map(([title, detail]) => (
              <div key={title} className={styles.resultItem}>
                <div className={styles.resultTitle}>{title}</div>
                <div className={styles.cardDetail}>{detail}</div>
              </div>
            ))}
          </div>
          <div className={styles.resultActions}>
            {['Share Link', 'Share to WeChat', 'Share to Slack', 'Open External Preview', 'Download', 'Copy', 'Archive', 'Export DOCX', 'Export XLSX', 'Export PPTX', 'Export PDF', 'Export ZIP'].map((action) => (
              <button key={action} type="button" onClick={() => runResultAction(action)} className={styles.smallButton}>
                {action}
              </button>
            ))}
          </div>
        </aside>
      </main>
    </div>
  );
}

function ResultContent({ task, tab }: { task: AssistantTask; tab: ResultTab }) {
  if (tab === 'Artifacts') {
    return (
      <div className={styles.resultList}>
        {task.artifacts.length === 0 && <p className={styles.emptyText}>No artifacts yet.</p>}
        {task.artifacts.map((artifact) => (
          <div key={artifact.id} className={styles.resultItem}>
            <div className={styles.resultTitle}>{artifact.filename}</div>
            <div className={styles.overline}>{artifact.type}</div>
          </div>
        ))}
      </div>
    );
  }

  if (tab === 'All Files') {
    return (
      <div className={styles.resultList}>
        {task.changes.map((change) => (
          <div key={change.id} className={styles.resultItem}>{change.path}</div>
        ))}
        {task.artifacts.map((artifact) => (
          <div key={artifact.id} className={styles.resultItem}>{artifact.filename}</div>
        ))}
      </div>
    );
  }

  if (tab === 'Changes') {
    return (
      <div className={styles.resultList}>
        {task.changes.length === 0 && <p className={styles.emptyText}>No file changes yet.</p>}
        {task.changes.map((change) => (
          <div key={change.id} className={styles.resultItem}>
            <div className={styles.resultTitle}>{change.summary}</div>
            <div className={cx(styles.statusBadge, styles.warningButton)}>{change.approvalStatus}</div>
          </div>
        ))}
      </div>
    );
  }

  return (
    <div className={styles.resultList}>
      {task.artifacts.length === 0 && <p className={styles.emptyText}>Preview appears after the first artifact.</p>}
      {task.artifacts.map((artifact) => (
        <div key={artifact.id} className={styles.resultItem}>
          {artifact.preview}
        </div>
      ))}
    </div>
  );
}

function FeaturePanel({
  panel,
  capabilities,
  onAction,
  billing,
}: {
  panel: Panel;
  capabilities: AssistantCapabilities;
  onAction: (action: string) => void;
  billing?: any;
}) {
  if (panel === 'remote') {
    return (
      <section className={styles.panel}>
        <h2 className={styles.sectionTitle}>Remote Control</h2>
        <div className={styles.featureGridThree}>
          {[...capabilities.remotePlatforms, 'File/Image Upload'].map((platform) => (
            <div key={platform} className={styles.featureCard}>
              <div className={styles.cardTitle}>{platform}</div>
              <div className={styles.cardDetail}>
                {platform === 'File/Image Upload' ? 'Remote bots can attach files and images to tasks.' : 'Connected task intake'}
              </div>
            </div>
          ))}
        </div>
        <div className={styles.featureGridThree}>
          {[
            ['Socket Mode', 'Slack app connection with bot and app tokens.'],
            ['WebSocket Long Connection', 'DingTalk long-lived remote control channel.'],
            ['URL Callback', 'DingTalk callback mode with AES key and token validation.'],
            ['Pairing Code', 'Bind Slack and chat channels by sending a one-time code.'],
            ['QR Code Linking', 'Bind WeChat ClawBot by scanning the local setup QR code.'],
            ['Credential Fields', 'Show the exact token, secret, intent, AES, and callback fields required by each platform.'],
            ['Troubleshooting Catalog', 'Surface platform-specific reconnect, token, permission, and callback fixes.'],
          ].map(([title, detail]) => (
            <div key={title} className={cx(styles.featureCard, styles.featureCardAccent)}>
              <div className={styles.cardTitle}>{title}</div>
              <div className={styles.cardDetail}>{detail}</div>
            </div>
          ))}
        </div>
        <div className={styles.actionRow}>
          <button type="button" onClick={() => onAction('upload_remote_file')} className={styles.smallButton}>
            Upload Remote File
          </button>
        </div>
      </section>
    );
  }

  if (panel === 'automations') {
    return (
      <section className={styles.panel}>
        <h2 className={styles.sectionTitle}>Automations</h2>
        <div className={styles.featureGridTwo}>
          <div className={styles.featureCard}>
            <div className={styles.cardTitle}>Weekly research brief</div>
            <div className={styles.cardDetail}>Every Monday 09:00</div>
          </div>
          {[
            ['Execution history', 'Runs, approvals, outputs, and notifications'],
            ['One-time task', 'Schedule a single future run without binding a permanent workspace.'],
            ['List mode', 'Scan active, paused, and completed automations in a compact list.'],
          ].map(([title, detail]) => (
            <div key={title} className={styles.featureCard}>
              <div className={styles.cardTitle}>{title}</div>
              <div className={styles.cardDetail}>{detail}</div>
            </div>
          ))}
        </div>
        <div className={styles.actionRow}>
          <button type="button" onClick={() => onAction('create_one_time_task')} className={styles.smallButton}>
            Schedule One-time Task
          </button>
        </div>
      </section>
    );
  }

  if (panel === 'memory') {
    return (
      <section className={styles.panel}>
        <h2 className={styles.sectionTitle}>Memory</h2>
        <div className={styles.resultList}>
          <div className={styles.resultItem}>Prefer concise technical summaries with citations.</div>
          <div className={styles.resultItem}>Ask before sending messages or modifying original files.</div>
        </div>
        <button type="button" className={styles.smallButton}>
          Import Memory
        </button>
      </section>
    );
  }

  if (panel === 'skills') {
    return (
      <section className={styles.panel}>
        <h2 className={styles.sectionTitle}>Skill Marketplace</h2>
        <div className={styles.featureGridThree}>
          {[
            'Web Research',
            'Document Writer',
            'Chart Builder',
            'Expert Ranking',
            'Custom Expert Builder',
            'Slash Command Runner',
            'Agent Browser',
            'Google Calendar',
            'Google Drive',
            'Google Search',
            'Office Document Suite',
            'Local Whisper',
            'yt-dlp Downloader',
            'Obsidian',
            'Frontend Design',
            'Batch Skill Updates',
            'Generated Skill Package',
          ].map((skill) => (
            <div key={skill} className={styles.featureCard}>
              <div className={styles.cardTitle}>{skill}</div>
              <div className={styles.cardDetail}>
                {skill.includes('Batch') ? 'Update all marketplace skills at once.' : skill.includes('Generated') ? 'skill.yml, README, and implementation files.' : skill.includes('Expert') ? 'Expert Center' : 'Marketplace'}
              </div>
            </div>
          ))}
        </div>
        <div className={styles.actionRow}>
          <button type="button" onClick={() => onAction('summon_expert')} className={styles.smallButton}>
            Summon Expert
          </button>
          <button type="button" onClick={() => onAction('summarize')} className={styles.smallButton}>
            Run /summarize
          </button>
        </div>
      </section>
    );
  }

  if (panel === 'connectors') {
    return (
      <section aria-label="Connector panel" className={styles.panel}>
        <h2 className={styles.sectionTitle}>Connectors</h2>
        <div className={styles.featureGridThree}>
          {[
            'GitHub',
            'GitLab',
            'Jira',
            'Confluence',
            'Google Calendar',
            'Google Drive',
            'Gmail',
            'Notion',
            'Slack',
            'MCP Endpoint',
            'Tencent Docs',
            'Tencent Meeting',
            'WeCom Docs',
            'QQ Mail',
          ].map((connector) => (
            <div key={connector} className={styles.featureCard}>
              <div className={styles.cardTitle}>{connector}</div>
              <div className={styles.cardDetail}>Available</div>
            </div>
          ))}
        </div>
        <div className={styles.featureGridThree}>
          {[
            ['OAuth Flow', 'Google Calendar, Drive, Gmail, and workspace connectors support OAuth setup.'],
            ['Calendar Events', 'Read meeting context and scheduling metadata for tasks.'],
            ['Drive Context', 'Select files and folders as assistant task references.'],
          ].map(([title, detail]) => (
            <div key={title} className={cx(styles.featureCard, styles.featureCardAccent)}>
              <div className={styles.cardTitle}>{title}</div>
              <div className={styles.cardDetail}>{detail}</div>
            </div>
          ))}
        </div>
        <div className={styles.featureGridThree}>
          {capabilities.mcpFeatures.map((feature) => (
            <div key={feature} className={cx(styles.featureCard, styles.featureCardAccent)}>
              <div className={styles.cardTitle}>{feature}</div>
              <div className={styles.cardDetail}>MCP connector support</div>
            </div>
          ))}
        </div>
        <div className={styles.actionRow}>
          <button type="button" onClick={() => onAction('try_mcp')} className={styles.smallButton}>
            Try MCP Tool
          </button>
        </div>
      </section>
    );
  }

  if (panel === 'data') {
    return (
      <section aria-label="Data management panel" className={styles.panel}>
        <h2 className={styles.sectionTitle}>Data Management</h2>
        <div className={styles.featureGridThree}>
          {[
            ['Shared Files', 'Review files currently available to tasks.'],
            ['Archived Tasks', 'Restore or permanently clean completed work.'],
            ['Unshare Queue', 'Revoke task and remote-channel file access.'],
            ['Copy Share Link', 'Copy the current artifact share URL after review.'],
            ['Download Shared File', 'Prepare a downloadable file URL for the current artifact.'],
            ['Cancel Sharing', 'Remove a shared file from public access and revoke its link.'],
            ['Unarchive Task', 'Restore an archived task to the active task list.'],
            ['Pinned Tasks', 'Keep important tasks at the top of the sidebar for quick access.'],
            ['Workspace Management', 'Search, filter, collapse, pin, archive, and organize tasks by workspace.'],
            ['Feedback Product Team Route', 'Send issues through Help & Feedback with screenshots and logs.'],
            ['Collapse All', 'Fold every workspace and task group.'],
            ['Expand All', 'Open every workspace and task group.'],
            ['Hard Delete', 'Permanently remove selected workspace or task records.'],
          ].map(([title, detail]) => (
            <div key={title} className={styles.featureCard}>
              <div className={styles.cardTitle}>{title}</div>
              <div className={styles.cardDetail}>{detail}</div>
            </div>
          ))}
        </div>
        <div className={styles.featureGridThree}>
          {[
            ['Batch Convert', 'Plan format conversion before writing files.'],
            ['Rename Files', 'Preview filename changes before applying them.'],
            ['Merge PDFs', 'Combine selected PDFs into a tracked artifact.'],
          ].map(([title, detail]) => (
            <div key={title} className={cx(styles.featureCard, styles.featureCardAccent)}>
              <div className={styles.cardTitle}>{title}</div>
              <div className={styles.cardDetail}>{detail}</div>
            </div>
          ))}
        </div>
        <div className={styles.actionRow}>
          <button type="button" onClick={() => onAction('copy_share_link')} className={styles.smallButton}>
            Copy Current Link
          </button>
          <button type="button" onClick={() => onAction('download_shared_file')} className={styles.smallButton}>
            Prepare Download
          </button>
          <button type="button" onClick={() => onAction('cancel_sharing')} className={styles.smallButton}>
            Revoke Share
          </button>
          <button type="button" onClick={() => onAction('unarchive_task')} className={styles.smallButton}>
            Restore Task
          </button>
          <button type="button" onClick={() => onAction('collapse_workspaces')} className={styles.smallButton}>
            Collapse All Workspaces
          </button>
        </div>
      </section>
    );
  }

  if (panel === 'explore') {
    return (
      <section aria-label="Explore panel" className={styles.panel}>
        <h2 className={styles.sectionTitle}>Explore</h2>
        <div className={styles.featureGridThree}>
          {[
            ['Community Templates', 'Browse shared task patterns and try them against your own workspace.'],
            ['Investor Update Agent', 'Remix metrics, notes, and tasks into a private Agent agent.'],
            ['Local File Cleanup Agent', 'Adapt batch rename, conversion, and PDF merge workflows safely.'],
            ['Research Deck Agent', 'Turn public research workflows into cited decks and charts.'],
            ['Remix as Agent Agent', 'Copy a useful workflow into your own workspace with attribution.'],
            ['Review & Share', 'Share your remixed result only after review.'],
            ['File Content Recognition', 'Recognize and summarize uploaded local files.'],
            ['Document Generation & Editing', 'Draft and revise documents from task context.'],
            ['Data Analysis & Visualization', 'Analyze tables and produce charts or forecasts.'],
            ['Social Media Content Creation', 'Create content variants for Twitter/X, LinkedIn, YouTube, and Medium.'],
            ['Automated Daily News Briefing', 'Build recurring briefings from search and remote channels.'],
            ['Remote Control via Slack', 'Operate and confirm assistant tasks from Slack.'],
            ['Google Calendar & Drive Integration', 'Combine meeting and file context through Google OAuth.'],
            ['Zero-Code Local Application Development', 'Generate runnable local apps from natural language.'],
            ['Creating Custom Skills', 'Create skill.yml, README, and implementation files.'],
            ['AI Self-Driven Workflows', 'Let an agent plan, execute, inspect, and continue autonomously.'],
            ['Make My Version', 'Personalize an official practice case into your own Agent workflow.'],
          ].map(([title, detail]) => (
            <div key={title} className={styles.featureCard}>
              <div className={styles.cardTitle}>{title}</div>
              <div className={styles.cardDetail}>{detail}</div>
            </div>
          ))}
        </div>
        <div className={styles.actionRow}>
          <button type="button" onClick={() => onAction('remix_template')} className={styles.smallButton}>
            Remix Template
          </button>
          <button type="button" onClick={() => onAction('share_exploration')} className={styles.smallButton}>
            Share Exploration
          </button>
        </div>
      </section>
    );
  }

  if (panel === 'cloud') {
    return (
      <section aria-label="Cloud runtime panel" className={styles.panel}>
        <h2 className={styles.sectionTitle}>Cloud Runtime</h2>
        <div className={styles.featureGridThree}>
          {[
            ['Cloud Agent', 'Run long tasks asynchronously with the same Agent task contract.'],
            ['Background Session', 'Keep research, generation, and file prep active while you leave the page.'],
            ['Uploaded Files', 'Attach local files, workspace files, and screenshots to cloud runs.'],
            ['Pause/Resume', 'Pause or resume background execution without deleting the task.'],
            ['Task Search', 'Find cloud and local tasks from the same workspace list.'],
            ['Runtime Manifest', 'Track isolation, files, model, and session state before actions.'],
          ].map(([title, detail]) => (
            <div key={title} className={styles.featureCard}>
              <div className={styles.cardTitle}>{title}</div>
              <div className={styles.cardDetail}>{detail}</div>
            </div>
          ))}
        </div>
        <div className={styles.actionRow}>
          <button type="button" onClick={() => onAction('start_cloud_session')} className={styles.smallButton}>
            Start Cloud Session
          </button>
          <button type="button" onClick={() => onAction('pause_cloud_session')} className={cx(styles.smallButton, styles.warningButton)}>
            Pause Cloud Session
          </button>
        </div>
      </section>
    );
  }

  if (panel === 'billing') {
    return (
      <section aria-label="Cost transparency dashboard" className={styles.panel}>
        <div className={styles.sectionHeader}>
          <div>
            <h2 className={styles.sectionTitle}>My Plan</h2>
            <p className={styles.eyebrow}>Cost Transparency Dashboard</p>
          </div>
          <button type="button" onClick={() => onAction('upgrade_plan')} className={styles.panelHeaderButton}>
            Upgrade
          </button>
        </div>
        <div className={styles.featureGridTwo}>
          <div className={styles.featureCard}>
            <div className={styles.cardTitle}>Current plan</div>
            <div className={styles.cardDetail}>{billing?.plan || 'Growth'}</div>
          </div>
          <div className={styles.featureCard}>
            <div className={styles.cardTitle}>Estimated next bill</div>
            <div className={styles.cardDetail}>${billing?.estimatedNextBill?.toFixed(2) || '29.00'}</div>
          </div>
          <div className={styles.featureCard}>
            <div className={styles.cardTitle}>AI actions used this month</div>
            <div className={styles.cardDetail}>{billing?.aiActionsUsed || 0} / {billing?.aiActionsLimit || 500}</div>
          </div>
          <div className={styles.featureCard}>
            <div className={styles.cardTitle}>Storage used</div>
            <div className={styles.cardDetail}>{billing?.storageUsedGB?.toFixed(1) || '0.0'}GB / {billing?.storageLimitGB || 50}GB</div>
          </div>
        </div>
      </section>
    );
  }

  if (panel === 'parity') {
    const summary = capabilities.paritySummary || { total: 50, implemented: 50, remaining: 0 };
    const categories = capabilities.parityCategories || [];
    const highlights = capabilities.parityHighlights || [];
    return (
      <section aria-label="Parity audit panel" className={styles.panel}>
        <div className={styles.sectionHeader}>
          <div>
            <h2 className={styles.sectionTitle}>Parity Audit</h2>
            <p className={styles.eyebrow}>Agent gap coverage</p>
          </div>
          <span className={styles.countBadge}>{summary.implemented} / {summary.total} implemented</span>
        </div>
        <div className={styles.featureGridThree}>
          {categories.map((category) => (
            <div key={category} className={styles.featureCard}>
              <div className={styles.cardTitle}>{category}</div>
              <div className={styles.cardDetail}>Implemented in Agent assistant surfaces and API contracts.</div>
            </div>
          ))}
        </div>
        <div className={styles.featureGridThree}>
          {highlights.map((highlight) => (
            <div key={highlight} className={cx(styles.featureCard, styles.featureCardAccent)}>
              <div className={styles.cardTitle}>{highlight}</div>
              <div className={styles.cardDetail}>Gap closed and tracked in the parity registry.</div>
            </div>
          ))}
        </div>
        <div className={styles.resultItem}>
          {summary.remaining} remaining gaps
        </div>
      </section>
    );
  }

  if (panel === 'models') {
    return (
      <section aria-label="Models and runtime panel" className={styles.panel}>
        <h2 className={styles.sectionTitle}>Models & Runtime</h2>
        <div className={styles.featureGridThree}>
          {capabilities.modelProviders.map((provider) => (
            <div key={provider} className={styles.featureCard}>
              <div className={styles.cardTitle}>{provider}</div>
              <div className={styles.cardDetail}>Selectable per task or automation.</div>
            </div>
          ))}
        </div>
        <div className={styles.featureGridThree}>
          {[
            ['Provider Presets', 'Auto-fill model URL, list, and capability flags for standard providers.'],
            ['Custom Model UI', 'Configure API keys, endpoints, headers, and model parameters visually.'],
            ['Model Capability Flags', `Track ${capabilities.modelCapabilities?.join(', ') || 'tool calling, image input, reasoning, and local inference'}.`],
            ['Custom Protocol', 'Send requests directly to a non-standard endpoint URL when advanced settings require it.'],
            ['Runtime Detection', 'Detect Python and Node.js availability before running local skills.'],
            ['System Proxy', 'Use system proxy settings for model and connector calls.'],
          ].map(([title, detail]) => (
            <div key={title} className={cx(styles.featureCard, styles.featureCardAccent)}>
              <div className={styles.cardTitle}>{title}</div>
              <div className={styles.cardDetail}>{detail}</div>
            </div>
          ))}
        </div>
        <div className={styles.actionRow}>
          <button type="button" onClick={() => onAction('save_custom_model')} className={styles.smallButton}>
            Save Custom Model
          </button>
        </div>
      </section>
    );
  }

  if (panel === 'system') {
    return (
      <section aria-label="System and safety panel" className={styles.panel}>
        <h2 className={styles.sectionTitle}>System & Safety</h2>
        <div className={styles.featureGridThree}>
          {[
            ['Plugin Marketplace', 'Install, update, try, and uninstall skill or suite packages with security checks.'],
            ['Claw Setup', 'Connect or disconnect mobile bot channels and confirm remote commands.'],
            ['High-risk Confirmations', 'Review external sends, destructive changes, and full-access operations.'],
            ['UI Settings', 'Adjust font size, language, generated-content markers, and filter handling.'],
            ['Compact Mode', 'Collapse decorative chat details and tool-call chrome for dense task work.'],
            ['Auto-install Low-risk Skills', 'Continue non-high-risk skill installs after security scan approval.'],
            ['Prevent Sleep', 'Keep remote control and automation sessions running without the machine sleeping.'],
            ['Account Profile', 'Show avatar, account details, and OAuth sign-in providers from the sidebar.'],
            ['Version Information', 'Expose the current Agent parity/version build in the sidebar footer.'],
            ['Feedback & Logs', 'Send product feedback and upload diagnostic logs for support.'],
            ['Screenshot Attachment', 'Attach a screenshot with feedback so support can inspect visual issues.'],
            ['Desktop Platform Support', 'macOS Apple Silicon, macOS Intel, Windows x64, and Windows ARM64 support matrix.'],
            ['Online Requirement', 'Account, connector, and remote-control features require online service access.'],
            ['Multi-device Sync', 'Account settings sync across signed-in desktop installations.'],
            ['Log Folder Locations', 'Open Log Folder and Open Log Directory entries expose diagnostics paths.'],
            ['Windows Installation Guide', 'Windows 10 1809 or later, Windows 11, x64, ARM64, and .exe installer flow.'],
            ['macOS Installation Guide', 'Apple Silicon and Intel installation through a .dmg package.'],
            ['Universal Binary', 'macOS package supports Apple Silicon and Intel processors.'],
            ['Windows Defender SmartScreen', 'Installation troubleshooting covers SmartScreen prompts and first-launch delays.'],
            ['Privacy & Security Permission', 'macOS remote-control setup points users to System Settings -> Privacy & Security.'],
            ['Privacy Retention Matrix', 'Inputs and outputs retain for 14 days; configuration stays on the local device.'],
            ['Data Subject Rights', 'Access, portability, correction, erasure, restriction, objection, and consent withdrawal.'],
            ['AI Training Opt-out', 'Inputs and outputs training opt-out route is agent_ai@tencent.com.'],
            ['Billing Retention', 'Payment and billing information retention is tracked for 24 months.'],
          ].map(([title, detail]) => (
            <div key={title} className={styles.featureCard}>
              <div className={styles.cardTitle}>{title}</div>
              <div className={styles.cardDetail}>{detail}</div>
            </div>
          ))}
        </div>
        <div className={styles.actionRow}>
          <button type="button" onClick={() => onAction('install_plugin')} className={styles.smallButton}>
            Install Office Suite
          </button>
          <button type="button" onClick={() => onAction('connect_claw')} className={styles.smallButton}>
            Connect Slack Claw
          </button>
          <button type="button" onClick={() => onAction('create_approval')} className={cx(styles.smallButton, styles.warningButton)}>
            Create High-risk Approval
          </button>
          <button type="button" onClick={() => onAction('save_ui_settings')} className={styles.smallButton}>
            Save UI Settings
          </button>
          <button type="button" onClick={() => onAction('upload_logs')} className={styles.smallButton}>
            Upload Logs
          </button>
          <button type="button" onClick={() => onAction('upload_screenshot')} className={styles.smallButton}>
            Attach Screenshot
          </button>
        </div>
      </section>
    );
  }

  return (
    <section aria-label="Connector panel" className={styles.panel}>
      <h2 className={styles.sectionTitle}>Permissions</h2>
      <div className={styles.featureGridThree}>
        {[
          ['Permission Mode', 'Guarded'],
          ['Grant Folder', 'Authorize a folder for task reads and writes.'],
          ['Revoke Folder', 'Remove folder access from future tasks.'],
          ['Sandbox Execution', 'Run tools in an isolated execution boundary.'],
          ['Full Access Confirmation', 'Require explicit confirmation before fully opening permissions.'],
          ...((capabilities.computerUseModes || ['Normal', 'Auto', 'Full Access']).map((mode) => [`${mode} computer use`, 'Agent-style computer operation scope.'])),
        ].map(([title, detail]) => (
          <div key={title} className={styles.featureCard}>
            <div className={styles.cardTitle}>{title}</div>
            <div className={styles.cardDetail}>{detail}</div>
          </div>
        ))}
      </div>
    </section>
  );
}
