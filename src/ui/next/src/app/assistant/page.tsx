'use client';

import Link from 'next/link';
import { useEffect, useMemo, useState } from 'react';
import styles from './assistant.module.css';

type PermissionProfile = 'Guarded' | 'Full Access';
type AssistantTaskStatus = 'running' | 'completed' | 'blocked' | 'failed' | 'archived';

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
};

type AssistantCapabilities = {
  resultTabs: string[];
  remotePlatforms: string[];
  outputFormats: string[];
  workModes: string[];
  permissionProfiles: string[];
  modelProviders: string[];
  sharingTargets: string[];
  workspaceControls: string[];
  commandSurfaces: string[];
  mcpFeatures: string[];
};

type Panel = 'remote' | 'automations' | 'memory' | 'skills' | 'connectors' | 'data' | 'permissions' | 'models';
type ResultTab = 'Artifacts' | 'All Files' | 'Changes' | 'Preview';

const resultTabs: ResultTab[] = ['Artifacts', 'All Files', 'Changes', 'Preview'];
const defaultCapabilities: AssistantCapabilities = {
  resultTabs,
  remotePlatforms: ['Slack', 'Telegram', 'Discord', 'WeChat Work', 'Feishu', 'DingTalk', 'QQ', 'YuanbaoPai', 'WeChat ClawBot'],
  outputFormats: ['Document', 'Spreadsheet', 'Presentation', 'PDF', 'Chart', 'Code App', 'ZIP'],
  workModes: ['Ask', 'Craft', 'Plan', 'Coding'],
  permissionProfiles: ['Guarded', 'Full Access'],
  modelProviders: ['Auto', 'OpenAI', 'Anthropic', 'MiniMax', 'DeepSeek', 'Kimi', 'Local Ollama', 'Custom OpenAI Compatible'],
  sharingTargets: ['Share Link', 'WeChat', 'Slack', 'Download', 'Copy'],
  workspaceControls: ['Collapse All', 'Expand All', 'Hard Delete', 'Archive Cleanup'],
  commandSurfaces: ['/skill', '/compact', '/summarize', '/clear'],
  mcpFeatures: ['Tool Progress', 'Resources', 'Static Headers', 'Connector Try It'],
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
        content: 'Jarvis is ready.',
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
  const [tasks, setTasks] = useState<AssistantTask[]>([]);
  const [capabilities, setCapabilities] = useState<AssistantCapabilities>(defaultCapabilities);
  const [activeTaskId, setActiveTaskId] = useState('');
  const [resultTab, setResultTab] = useState<ResultTab>('Artifacts');
  const [panel, setPanel] = useState<Panel>('remote');
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

    loadTasks();
    return () => {
      mounted = false;
    };
  }, []);

  const activeTask = useMemo(
    () => tasks.find((task) => task.id === activeTaskId) || tasks[0] || fallbackTasks[0],
    [activeTaskId, tasks],
  );

  const workspaces = useMemo(() => Array.from(new Set(tasks.map((task) => task.workspace))), [tasks]);

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
        userId: 'jarvis-user',
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
    } else if (action === 'save_custom_model') {
      await runApiAction('/api/assistant/models', 'PATCH', {
        action: 'upsert',
        provider: 'Custom OpenAI Compatible',
        modelId: 'jarvis-custom',
        endpoint: 'https://models.example.test/v1',
        parameters: { temperature: 0.2 },
        skipChatCompletions: true,
      }, 'Custom model saved');
    }
  }

  return (
    <div className={styles.shell} data-testid="assistant-shell">
      <header className={styles.header}>
        <div className={styles.headerInner}>
          <div className={styles.headerTop}>
            <div>
              <h1 className={styles.title}>Jarvis Assistant</h1>
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
              ['permissions', 'Permissions'],
              ['models', 'Models & Runtime'],
            ] as [Panel, string][]).map(([id, label]) => (
              <PanelButton key={id} active={panel === id} onClick={() => setPanel(id)}>
                {label}
              </PanelButton>
            ))}
          </nav>
        </div>
      </header>

      <main className={styles.workstation} data-testid="assistant-workstation">
        <aside className={styles.panel}>
          <div className={styles.sectionHeader}>
            <div>
              <h2 className={styles.sectionTitle}>Task List</h2>
              <p className={styles.eyebrow}>Workspaces</p>
            </div>
            <span className={styles.countBadge}>{tasks.length} tasks</span>
          </div>
          <div className={styles.chipRow}>
            {workspaces.map((name) => (
              <span key={name} className={styles.chip}>
                {name}
              </span>
            ))}
          </div>
          <div className={styles.taskList}>
            {tasks.map((task) => (
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
                  <select value={model} onChange={(event) => setModel(event.target.value)} className={styles.select}>
                    <option>Auto</option>
                    <option>MiniMax-M3</option>
                    <option>OpenAI GPT-4.1</option>
                    <option>Claude Sonnet</option>
                    <option>Local Ollama</option>
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
          <FeaturePanel panel={panel} capabilities={capabilities} onAction={runFeatureAction} />
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
}: {
  panel: Panel;
  capabilities: AssistantCapabilities;
  onAction: (action: string) => void;
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
          {['Web Research', 'Document Writer', 'Chart Builder', 'Expert Ranking', 'Custom Expert Builder', 'Slash Command Runner'].map((skill) => (
            <div key={skill} className={styles.featureCard}>
              <div className={styles.cardTitle}>{skill}</div>
              <div className={styles.cardDetail}>{skill.includes('Expert') ? 'Expert Center' : 'Installed'}</div>
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
          {['Google Drive', 'Slack', 'MCP Endpoint', 'Tencent Docs', 'Tencent Meeting', 'WeCom Docs', 'QQ Mail'].map((connector) => (
            <div key={connector} className={styles.featureCard}>
              <div className={styles.cardTitle}>{connector}</div>
              <div className={styles.cardDetail}>Available</div>
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
          <button type="button" onClick={() => onAction('collapse_workspaces')} className={styles.smallButton}>
            Collapse All Workspaces
          </button>
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
            ['Custom Model UI', 'Configure API keys, endpoints, headers, and model parameters visually.'],
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
