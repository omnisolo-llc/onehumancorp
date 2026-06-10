import { render, screen, fireEvent, waitFor, within } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import AssistantPage from './page';

const tasksPayload = {
  tasks: [
    {
      id: 'task-weekly-brief',
      title: "Create this week's operating brief",
      workspace: 'Personal OS',
      status: 'running',
      currentStep: 'Drafting report',
      mode: 'Plan',
      model: 'Auto',
      provider: 'Auto',
      permissionProfile: 'Guarded',
      riskSummary: ['Guarded mode is active', 'External sends require approval'],
      artifacts: [
        { id: 'artifact-weekly', type: 'document', filename: 'weekly-brief.md', preview: 'Weekly brief draft with action items.' },
      ],
      changes: [
        { id: 'change-weekly', path: '/workspace/reports/weekly-brief.md', summary: 'Creates a markdown brief.', approvalStatus: 'pending' },
      ],
      messages: [
        { id: 'msg-1', role: 'user', content: 'Summarize this week and prepare a brief.' },
        { id: 'msg-2', role: 'assistant', content: 'I am gathering context and drafting the brief.' },
      ],
      actions: [
        { id: 'action-stop', label: 'Stop', kind: 'control', approvalRequired: false },
        { id: 'action-approve', label: 'Approve Changes', kind: 'approval', approvalRequired: true },
      ],
    },
    {
      id: 'task-downloads',
      title: 'Organize Downloads by file type',
      workspace: 'Files',
      status: 'blocked',
      currentStep: 'Waiting for folder permission',
      mode: 'Craft',
      model: 'MiniMax M2.5',
      provider: 'Auto',
      permissionProfile: 'Guarded',
      riskSummary: ['Needs permission for Downloads'],
      artifacts: [],
      changes: [],
      messages: [
        { id: 'msg-3', role: 'assistant', content: 'I need permission to read Downloads before continuing.' },
      ],
      actions: [
        { id: 'action-grant', label: 'Grant Folder Access', kind: 'permission', approvalRequired: true },
      ],
    },
    {
      id: 'task-investor-plan',
      title: 'Plan investor update automation',
      workspace: 'Research',
      status: 'planning',
      currentStep: 'Analyzing requirements',
      mode: 'Plan',
      model: 'Agent',
      provider: 'Auto',
      permissionProfile: 'Guarded',
      riskSummary: ['Guarded mode is active'],
      artifacts: [],
      changes: [],
      messages: [
        { id: 'msg-4', role: 'assistant', content: 'I am organizing the plan before execution.' },
      ],
      actions: [
        { id: 'action-stop-planning', label: 'Stop', kind: 'control', approvalRequired: false },
      ],
    },
    {
      id: 'task-remote-pending',
      title: 'Wait for Slack confirmation',
      workspace: 'Remote Control',
      status: 'pending',
      currentStep: 'Waiting for remote confirmation',
      mode: 'Agent',
      model: 'Auto',
      provider: 'Auto',
      permissionProfile: 'Guarded',
      riskSummary: ['External sends require approval'],
      artifacts: [],
      changes: [],
      messages: [
        { id: 'msg-5', role: 'assistant', content: 'I need a Slack confirmation before continuing.' },
      ],
      actions: [
        { id: 'action-confirm', label: 'Request Confirmation', kind: 'approval', approvalRequired: true },
      ],
    },
  ],
  capabilities: {
    resultTabs: ['Artifacts', 'All Files', 'Changes', 'Preview'],
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
    parityHighlights: ['Runtime sandbox filesystem', 'Checkpoint creation', 'Expert team decomposition', 'Hook plugins', 'Dedicated remote folder', 'Automation task templates', 'Task search box', 'User-level MCP config', 'Mini app voice input', 'Permission risk boundary', 'Clipboard screenshot paste', 'Hook event family', '/doctor environment check', 'User settings.json', 'TaskOutput retrieval', 'Project subagent directory', 'Camera attachment', 'Shared link expiry', 'Official connector roster', 'Custom protocol toggle', 'Prevent sleep', 'Cancel sharing', 'Unarchive task', 'Featured skills roster', 'Official practice case library', 'Platform-specific Claw setup guides', 'Desktop platform support matrix', 'New task bar anatomy', 'Conversation top toolbar', 'Privacy retention matrix', 'AI training opt-out'],
  },
};

beforeEach(() => {
  vi.clearAllMocks();
  global.fetch = vi.fn(async (url: RequestInfo | URL, init?: RequestInit) => {
    const urlString = typeof url === 'string' ? url : url.toString();
    if (urlString.includes('/api/assistant/tasks') && init?.method === 'POST') {
      return new Response(JSON.stringify({
        task: {
          id: 'task-new',
          title: 'Build a Q3 planning deck',
          workspace: 'Launch Room',
          status: 'running',
          currentStep: 'Planning and preparing tools',
          mode: 'Plan',
          model: 'MiniMax-M3',
          provider: 'Auto',
          permissionProfile: 'Guarded',
          riskSummary: ['Guarded mode is active', 'External sends require approval'],
          artifacts: [
            { id: 'artifact-deck', type: 'presentation', filename: 'assistant-presentation.pptx', preview: 'Slide deck outline.' },
            { id: 'artifact-chart', type: 'chart', filename: 'assistant-chart.png', preview: 'Generated chart preview.' },
          ],
          changes: [
            { id: 'change-deck', path: '/workspace/launch/assistant-presentation.pptx', summary: 'Creates a deck.', approvalStatus: 'pending' },
          ],
          messages: [
            { id: 'msg-user', role: 'user', content: 'Build a Q3 planning deck' },
            { id: 'msg-assistant', role: 'assistant', content: 'Agent planned the task with Web Research.' },
          ],
          actions: [
            { id: 'action-preview', label: 'Open Preview', kind: 'preview', approvalRequired: false },
            { id: 'action-download', label: 'Download File', kind: 'download', approvalRequired: false },
          ],
        },
      }), { status: 201, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/share')) {
      return new Response(JSON.stringify({ share: { id: 'share-1', target: 'WeChat', status: 'pending_review' } }), { status: 201, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/previews')) {
      return new Response(JSON.stringify({ preview: { artifactId: 'artifact-weekly', displayMode: 'external' } }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/uploads')) {
      return new Response(JSON.stringify({ upload: { id: 'upload-1', filename: 'remote-upload.png', status: 'available' } }), { status: 201, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/experts')) {
      return new Response(JSON.stringify({ task: { id: 'task-weekly-brief' } }), { status: 202, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/mcp')) {
      return new Response(JSON.stringify({ progress: [{ stage: 'completed', tool: 'search_issues' }] }), { status: 202, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/workspaces')) {
      return new Response(JSON.stringify({ workspaces: [{ name: 'Personal OS', collapsed: true }] }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/commands')) {
      return new Response(JSON.stringify({ result: { command: '/summarize', status: 'completed' } }), { status: 202, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/models')) {
      return new Response(JSON.stringify({ models: [{ provider: 'Custom OpenAI Compatible' }] }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/plugins')) {
      return new Response(JSON.stringify({ plugins: [{ id: 'plugin-office-suite', status: 'installed' }] }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/claw')) {
      return new Response(JSON.stringify({ channels: [{ platform: 'Slack', status: 'connected' }] }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/approvals')) {
      return new Response(JSON.stringify({ approval: { id: 'approval-1', status: 'approved' } }), { status: 201, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/settings')) {
      return new Response(JSON.stringify({ settings: { fontSize: 'large', agentName: 'Agent' } }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/support')) {
      return new Response(JSON.stringify({ ticket: { id: 'ticket-1', status: 'received' } }), { status: 201, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/explore') && init?.method === 'POST') {
      return new Response(JSON.stringify({
        remix: { id: 'remix-1', name: 'Investor Update Agent Remix', visibility: 'private' },
        task: { id: 'task-remix', mode: 'Agent' },
      }), { status: 201, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/explore') && init?.method === 'PATCH') {
      return new Response(JSON.stringify({ remix: { id: 'remix-1', visibility: 'shared', target: 'Share Link' } }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/cloud') && init?.method === 'POST') {
      return new Response(JSON.stringify({ session: { id: 'cloud-1', status: 'running', mode: 'Cloud Agent' } }), { status: 201, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/cloud') && init?.method === 'PATCH') {
      return new Response(JSON.stringify({ session: { id: 'cloud-1', status: 'paused', mode: 'Cloud Agent' } }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/tasks')) {
      return new Response(JSON.stringify(tasksPayload), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    return new Response(JSON.stringify({}), { status: 200, headers: { 'Content-Type': 'application/json' } });
  }) as any;
});

test('renders the primary Agent workstation and preserves Expert Center navigation', async () => {
  render(<AssistantPage />);

  expect(await screen.findByRole('heading', { name: 'Agent Assistant' })).toBeDefined();
  expect(screen.getAllByText('Personal OS').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Files').length).toBeGreaterThan(0);
  expect(screen.getAllByText("Create this week's operating brief").length).toBeGreaterThan(0);
  expect(screen.getByText('Organize Downloads by file type')).toBeDefined();
  expect(screen.getByRole('link', { name: 'Expert Center' })).toHaveAttribute('href', '/agents');
  expect(screen.getByTestId('assistant-shell')).toBeDefined();
  expect(screen.getByTestId('assistant-workstation')).toBeDefined();
  expect(screen.getByRole('button', { name: 'Remote Control' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Automations' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Memory' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Skills' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Connectors' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Data Management' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Explore' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Cloud Runtime' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Parity Audit' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Permissions' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Models & Runtime' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'System & Safety' })).toBeDefined();
  expect(screen.getByText('Task List')).toBeDefined();
  expect(screen.getByText('Conversation')).toBeDefined();
  expect(screen.getByText('Results Panel')).toBeDefined();
});

test('shows conversation, artifacts, files, changes, and preview for the active task', async () => {
  render(<AssistantPage />);

  expect(await screen.findByText('I am gathering context and drafting the brief.')).toBeDefined();
  expect(screen.getByText('weekly-brief.md')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'All Files' }));
  expect(screen.getByText('/workspace/reports/weekly-brief.md')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Changes' }));
  expect(screen.getByText('Creates a markdown brief.')).toBeDefined();
  expect(screen.getAllByText('pending').length).toBeGreaterThan(0);

  fireEvent.click(screen.getByRole('button', { name: 'Preview' }));
  expect(screen.getByText('Weekly brief draft with action items.')).toBeDefined();
  expect(screen.getByRole('button', { name: 'Stop' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Approve Changes' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Export DOCX' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Export XLSX' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Export PPTX' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Export PDF' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Export ZIP' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Share Link' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Share to WeChat' })).toBeDefined();
  expect(screen.getByText('Preview Auto Refresh')).toBeDefined();
});

test('filters the Agent task rail by search text and status', async () => {
  render(<AssistantPage />);

  await screen.findAllByText("Create this week's operating brief");
  const taskRail = screen.getByLabelText('Task rail');
  expect(within(taskRail).getByText("Create this week's operating brief")).toBeDefined();
  expect(within(taskRail).getByText('Organize Downloads by file type')).toBeDefined();

  fireEvent.change(screen.getByLabelText('Search tasks'), {
    target: { value: 'downloads' },
  });
  expect(within(taskRail).getByText('Organize Downloads by file type')).toBeDefined();
  expect(within(taskRail).queryByText("Create this week's operating brief")).toBeNull();
  expect(screen.getByText('1 task shown')).toBeDefined();

  fireEvent.change(screen.getByLabelText('Search tasks'), {
    target: { value: '' },
  });
  fireEvent.change(screen.getByLabelText('Task status filter'), {
    target: { value: 'running' },
  });
  expect(within(taskRail).getByText("Create this week's operating brief")).toBeDefined();
  expect(within(taskRail).queryByText('Organize Downloads by file type')).toBeNull();

  fireEvent.change(screen.getByLabelText('Task status filter'), {
    target: { value: 'planning' },
  });
  expect(within(taskRail).getByText('Plan investor update automation')).toBeDefined();
  expect(within(taskRail).queryByText("Create this week's operating brief")).toBeNull();

  fireEvent.change(screen.getByLabelText('Task date filter'), {
    target: { value: 'today' },
  });
  expect(screen.getByDisplayValue('Today')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Reset task filters' }));
  expect(within(taskRail).getByText("Create this week's operating brief")).toBeDefined();
  expect(within(taskRail).getByText('Organize Downloads by file type')).toBeDefined();
  expect(within(taskRail).getByText('Wait for Slack confirmation')).toBeDefined();
});

<<<<<<< HEAD
test('shows an empty state before API data arrives', () => {
=======
test('shows a populated fallback task rail before API data arrives', () => {
>>>>>>> 359e384d (feat(memory): Implement AgentMemoryService for tenant-isolated episodic memory)
  global.fetch = vi.fn(() => new Promise(() => undefined)) as any;

  render(<AssistantPage />);

  const taskRail = screen.getByLabelText('Task rail');
<<<<<<< HEAD
  expect(screen.getByText('0 tasks')).toBeDefined();
  expect(within(taskRail).getByText('No matching tasks.')).toBeDefined();
=======
  expect(screen.getByText('1 task')).toBeDefined();
  expect(within(taskRail).getByText('Create a personal briefing')).toBeDefined();
  expect(within(taskRail).getByText('Ready to plan')).toBeDefined();
>>>>>>> 359e384d (feat(memory): Implement AgentMemoryService for tenant-isolated episodic memory)
});

test('uses the current expanded Agent parity summary before API data arrives', () => {
  global.fetch = vi.fn(() => new Promise(() => undefined)) as any;

  render(<AssistantPage />);

  fireEvent.click(screen.getByRole('button', { name: 'Parity Audit' }));
  const parityPanel = screen.getByLabelText('Parity audit panel');
  expect(within(parityPanel).getByText('212 / 212 implemented')).toBeDefined();
  expect(within(parityPanel).getByText('Task management: 10/10')).toBeDefined();
  expect(within(parityPanel).getByText('Official docs gap closure: 14/14')).toBeDefined();
  expect(within(parityPanel).getByText('Extended docs gap closure: 24/24')).toBeDefined();
  expect(within(parityPanel).getByText('Core docs gap closure: 24/24')).toBeDefined();
  expect(within(parityPanel).getByText('Project subagent directory')).toBeDefined();
});

test('submits full Agent-style composer payload and selects the new task', async () => {
  render(<AssistantPage />);

  await screen.findAllByText("Create this week's operating brief");
  fireEvent.change(screen.getByLabelText('Task prompt'), {
    target: { value: 'Build a Q3 planning deck' },
  });
  fireEvent.change(screen.getByLabelText('Workspace'), {
    target: { value: 'Launch Room' },
  });
  fireEvent.change(screen.getByLabelText('Work directory'), {
    target: { value: '/workspace/launch' },
  });
  fireEvent.change(screen.getByLabelText('Output format'), {
    target: { value: 'Presentation' },
  });
  fireEvent.click(screen.getByRole('button', { name: 'Start Task' }));

  await waitFor(() => {
    expect(global.fetch).toHaveBeenCalledWith('/api/assistant/tasks', expect.objectContaining({ method: 'POST' }));
  });
  const taskCall = (global.fetch as any).mock.calls.find(
    ([url, init]: any[]) => url === '/api/assistant/tasks' && init?.method === 'POST',
  );
  const payload = JSON.parse(taskCall[1].body);
  expect(payload).toMatchObject({
    prompt: 'Build a Q3 planning deck',
    workspace: 'Launch Room',
    workDirectory: '/workspace/launch',
    outputFormat: 'Presentation',
    permissionProfile: 'Guarded',
  });
  expect(await screen.findByText('assistant-presentation.pptx')).toBeDefined();
});

test('opens feature panels for remote control, automations, memory, skills, connectors, and data management', async () => {
  render(<AssistantPage />);

  await screen.findByRole('heading', { name: 'Agent Assistant' });
  fireEvent.click(screen.getByRole('button', { name: 'Remote Control' }));
  for (const platform of ['Slack', 'Telegram', 'Discord', 'WeChat Work', 'Feishu', 'DingTalk', 'QQ', 'YuanbaoPai', 'WeChat ClawBot']) {
    expect(screen.getByText(platform)).toBeDefined();
  }

  fireEvent.click(screen.getByRole('button', { name: 'My Plan' }));
  expect(screen.getByText('Cost Transparency Dashboard')).toBeDefined();
  expect(screen.getByText('Current plan')).toBeDefined();
  expect(screen.getByText('Estimated next bill')).toBeDefined();
  expect(screen.getByText('AI actions used this month')).toBeDefined();
  expect(screen.getByText('Storage used')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Remote Control' }));
  expect(screen.getByText('File/Image Upload')).toBeDefined();
  expect(screen.getByText('Socket Mode')).toBeDefined();
  expect(screen.getByText('WebSocket Long Connection')).toBeDefined();
  expect(screen.getByText('URL Callback')).toBeDefined();
  expect(screen.getByText('Pairing Code')).toBeDefined();
  expect(screen.getByText('QR Code Linking')).toBeDefined();
  expect(screen.getByText('Credential Fields')).toBeDefined();
  expect(screen.getByText('Troubleshooting Catalog')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Automations' }));
  expect(screen.getByText('Weekly research brief')).toBeDefined();
  expect(screen.getByText('Execution history')).toBeDefined();
  expect(screen.getByText('One-time task')).toBeDefined();
  expect(screen.getByText('List mode')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Memory' }));
  expect(screen.getByText('Prefer concise technical summaries with citations.')).toBeDefined();
  expect(screen.getByText('Import Memory')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Skills' }));
  expect(screen.getByText('Skill Marketplace')).toBeDefined();
  expect(screen.getByText('Web Research')).toBeDefined();
  expect(screen.getByText('Expert Ranking')).toBeDefined();
  expect(screen.getByText('Custom Expert Builder')).toBeDefined();
  expect(screen.getByText('Slash Command Runner')).toBeDefined();
  for (const skill of ['Agent Browser', 'Google Calendar', 'Google Drive', 'Google Search', 'Office Document Suite', 'Local Whisper', 'yt-dlp Downloader', 'Obsidian', 'Frontend Design', 'Batch Skill Updates', 'Generated Skill Package']) {
    expect(screen.getByText(skill)).toBeDefined();
  }

  fireEvent.click(screen.getByRole('button', { name: 'Connectors' }));
  const connectorPanel = screen.getByLabelText('Connector panel');
  for (const connector of ['GitHub', 'GitLab', 'Jira', 'Confluence', 'Google Calendar', 'Google Drive', 'Gmail', 'Notion', 'Slack']) {
    expect(within(connectorPanel).getByText(connector)).toBeDefined();
  }
  expect(within(connectorPanel).getByText('OAuth Flow')).toBeDefined();
  expect(within(connectorPanel).getByText('Google Drive')).toBeDefined();
  expect(within(connectorPanel).getByText('MCP Endpoint')).toBeDefined();
  expect(within(connectorPanel).getByText('Tencent Docs')).toBeDefined();
  expect(within(connectorPanel).getByText('QQ Mail')).toBeDefined();
  expect(within(connectorPanel).getByText('Tool Progress')).toBeDefined();
  expect(within(connectorPanel).getByText('Static Headers')).toBeDefined();
  expect(within(connectorPanel).getByText('Connector Try It')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Data Management' }));
  expect(screen.getByText('Shared Files')).toBeDefined();
  expect(screen.getByText('Archived Tasks')).toBeDefined();
  expect(screen.getByText('Unshare Queue')).toBeDefined();
  expect(screen.getByText('Copy Share Link')).toBeDefined();
  expect(screen.getByText('Download Shared File')).toBeDefined();
  expect(screen.getByText('Cancel Sharing')).toBeDefined();
  expect(screen.getByText('Unarchive Task')).toBeDefined();
  expect(screen.getByText('Collapse All')).toBeDefined();
  expect(screen.getByText('Hard Delete')).toBeDefined();
  expect(screen.getByText('Batch Convert')).toBeDefined();
  expect(screen.getByText('Rename Files')).toBeDefined();
  expect(screen.getByText('Merge PDFs')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Explore' }));
  expect(screen.getByText('Community Templates')).toBeDefined();
  expect(screen.getByText('Investor Update Agent')).toBeDefined();
  for (const practice of ['File Content Recognition', 'Document Generation & Editing', 'Data Analysis & Visualization', 'Social Media Content Creation', 'Automated Daily News Briefing', 'Remote Control via Slack', 'Google Calendar & Drive Integration', 'Zero-Code Local Application Development', 'Creating Custom Skills', 'AI Self-Driven Workflows']) {
    expect(screen.getByText(practice)).toBeDefined();
  }
  expect(screen.getByText('Make My Version')).toBeDefined();
  expect(screen.getByText('Remix as Agent Agent')).toBeDefined();
  expect(screen.getByText('Share Exploration')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Cloud Runtime' }));
  const cloudPanel = screen.getByLabelText('Cloud runtime panel');
  expect(within(cloudPanel).getByText('Cloud Agent')).toBeDefined();
  expect(within(cloudPanel).getByText('Background Session')).toBeDefined();
  expect(within(cloudPanel).getByText('Uploaded Files')).toBeDefined();
  expect(within(cloudPanel).getByText('Pause/Resume')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Parity Audit' }));
  const parityPanel = screen.getByLabelText('Parity audit panel');
  expect(within(parityPanel).getByText('212 / 212 implemented')).toBeDefined();
  expect(within(parityPanel).getByText('Cloud Agent lifecycle: 24/24')).toBeDefined();
  expect(within(parityPanel).getByText('MCP configuration: 10/10')).toBeDefined();
  expect(within(parityPanel).getByText('Slash command coverage: 16/16')).toBeDefined();
  expect(within(parityPanel).getByText('Hook plugins')).toBeDefined();
  expect(within(parityPanel).getByText('Mini app voice input')).toBeDefined();
  expect(within(parityPanel).getByText('/doctor environment check')).toBeDefined();
  expect(within(parityPanel).getByText('Project subagent directory')).toBeDefined();
  expect(within(parityPanel).getByText('Official docs gap closure: 14/14')).toBeDefined();
  expect(within(parityPanel).getByText('Extended docs gap closure: 24/24')).toBeDefined();
  expect(within(parityPanel).getByText('Core docs gap closure: 24/24')).toBeDefined();
  expect(within(parityPanel).getByText('Official connector roster')).toBeDefined();
  expect(within(parityPanel).getByText('Cancel sharing')).toBeDefined();
  expect(within(parityPanel).getByText('Featured skills roster')).toBeDefined();
  expect(within(parityPanel).getByText('Platform-specific Claw setup guides')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Permissions' }));
  expect(screen.getByText('Permission Mode')).toBeDefined();
  expect(screen.getAllByText('Guarded').length).toBeGreaterThan(0);
  expect(screen.getByText('Grant Folder')).toBeDefined();
  expect(screen.getByText('Revoke Folder')).toBeDefined();
  expect(screen.getByText('Sandbox Execution')).toBeDefined();
  expect(screen.getByText('Full Access Confirmation')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Models & Runtime' }));
  expect(screen.getByText('Custom Model UI')).toBeDefined();
  expect(screen.getByText('Provider Presets')).toBeDefined();
  expect(screen.getByText('Model Capability Flags')).toBeDefined();
  expect(screen.getByText('Custom Protocol')).toBeDefined();
  expect(screen.getByText('Runtime Detection')).toBeDefined();
  expect(screen.getAllByText('Local Ollama').length).toBeGreaterThan(0);
  expect(screen.getAllByText('GPT-5-Codex').length).toBeGreaterThan(0);

  fireEvent.click(screen.getByRole('button', { name: 'System & Safety' }));
  expect(screen.getByText('Plugin Marketplace')).toBeDefined();
  expect(screen.getByText('Claw Setup')).toBeDefined();
  expect(screen.getByText('High-risk Confirmations')).toBeDefined();
  expect(screen.getByText('UI Settings')).toBeDefined();
  expect(screen.getByText('Compact Mode')).toBeDefined();
  expect(screen.getByText('Auto-install Low-risk Skills')).toBeDefined();
  expect(screen.getByText('Prevent Sleep')).toBeDefined();
  expect(screen.getByText('Account Profile')).toBeDefined();
  expect(screen.getByText('Version Information')).toBeDefined();
  expect(screen.getByText('Feedback & Logs')).toBeDefined();
  expect(screen.getByText('Screenshot Attachment')).toBeDefined();
  expect(screen.getByText('Desktop Platform Support')).toBeDefined();
  expect(screen.getByText('Online Requirement')).toBeDefined();
  expect(screen.getByText('Multi-device Sync')).toBeDefined();
  expect(screen.getByText('Log Folder Locations')).toBeDefined();
});

test('supports Agent-style mode and artifact options including Coding and Code App', async () => {
  render(<AssistantPage />);

  await screen.findAllByText("Create this week's operating brief");
  fireEvent.change(screen.getByLabelText('Mode'), { target: { value: 'Coding' } });
  fireEvent.change(screen.getByLabelText('Output format'), { target: { value: 'Code App' } });

  expect(screen.getByDisplayValue('Coding')).toBeDefined();
  expect(screen.getByDisplayValue('Code App')).toBeDefined();
  fireEvent.change(screen.getByLabelText('Mode'), { target: { value: 'Cloud Agent' } });
  fireEvent.change(screen.getByLabelText('Model'), { target: { value: 'DeepSeek V3.2' } });
  expect(screen.getByDisplayValue('Cloud Agent')).toBeDefined();
  expect(screen.getByDisplayValue('DeepSeek V3.2')).toBeDefined();
  expect(screen.getByText('Open Preview')).toBeDefined();
  expect(screen.getByText('Run Locally')).toBeDefined();
});

test('renders core Agent docs shell affordances for task bar conversation results sidebar and privacy', async () => {
  render(<AssistantPage />);

  await screen.findByRole('heading', { name: 'Agent Assistant' });

  for (const taskBarItem of ['Input Field', 'Model Selector', 'Context Tools', 'Mode Selector', 'Send Button', 'One-sentence Assignment', 'Default Directory', 'Parallel Work']) {
    expect(screen.getByText(taskBarItem)).toBeDefined();
  }
  for (const conversationItem of ['Collapse Sidebar', 'History', 'Show Details Panel', 'File & Image Upload', 'Execution Progress', 'Interrupt & Resume']) {
    expect(screen.getByText(conversationItem)).toBeDefined();
  }
  for (const resultItem of ['Selected Artifact Preview', 'Spreadsheet Preview', 'Document Preview', 'Web Preview Controls', 'All Files Tree', 'Changes Detail Review']) {
    expect(screen.getByText(resultItem)).toBeDefined();
  }

  fireEvent.click(screen.getByRole('button', { name: 'Data Management' }));
  for (const dataItem of ['Pinned Tasks', 'Workspace Management', 'Feedback Product Team Route']) {
    expect(screen.getByText(dataItem)).toBeDefined();
  }

  fireEvent.click(screen.getByRole('button', { name: 'System & Safety' }));
  for (const systemItem of ['Windows Installation Guide', 'macOS Installation Guide', 'Universal Binary', 'Windows Defender SmartScreen', 'Privacy & Security Permission', 'Privacy Retention Matrix', 'Data Subject Rights', 'AI Training Opt-out', 'Billing Retention']) {
    expect(screen.getByText(systemItem)).toBeDefined();
  }

  fireEvent.click(screen.getByRole('button', { name: 'Parity Audit' }));
  const parityPanel = screen.getByLabelText('Parity audit panel');
  expect(within(parityPanel).getByText('212 / 212 implemented')).toBeDefined();
  expect(within(parityPanel).getByText('Core docs gap closure: 24/24')).toBeDefined();
  expect(within(parityPanel).getByText('New task bar anatomy')).toBeDefined();
  expect(within(parityPanel).getByText('Conversation top toolbar')).toBeDefined();
  expect(within(parityPanel).getByText('Privacy retention matrix')).toBeDefined();
  expect(within(parityPanel).getByText('AI training opt-out')).toBeDefined();
});

test('backs parity controls with assistant API actions', async () => {
  render(<AssistantPage />);

  await screen.findByRole('heading', { name: 'Agent Assistant' });

  fireEvent.click(screen.getByRole('button', { name: 'Share to WeChat' }));
  await waitFor(() => {
    expect(global.fetch).toHaveBeenCalledWith('/api/assistant/share', expect.objectContaining({ method: 'POST' }));
  });

  fireEvent.click(screen.getByRole('button', { name: 'Open External Preview' }));
  await waitFor(() => {
    expect(global.fetch).toHaveBeenCalledWith('/api/assistant/previews', expect.objectContaining({ method: 'PATCH' }));
  });

  fireEvent.click(screen.getByRole('button', { name: 'Remote Control' }));
  fireEvent.click(screen.getByRole('button', { name: 'Upload Remote File' }));
  await waitFor(() => {
    expect(global.fetch).toHaveBeenCalledWith('/api/assistant/uploads', expect.objectContaining({ method: 'POST' }));
  });

  fireEvent.click(screen.getByRole('button', { name: 'Skills' }));
  fireEvent.click(screen.getByRole('button', { name: 'Summon Expert' }));
  fireEvent.click(screen.getByRole('button', { name: 'Run /summarize' }));

  fireEvent.click(screen.getByRole('button', { name: 'Connectors' }));
  fireEvent.click(screen.getByRole('button', { name: 'Try MCP Tool' }));

  fireEvent.click(screen.getByRole('button', { name: 'Data Management' }));
  fireEvent.click(screen.getByRole('button', { name: 'Collapse All Workspaces' }));

  fireEvent.click(screen.getByRole('button', { name: 'Models & Runtime' }));
  fireEvent.click(screen.getByRole('button', { name: 'Save Custom Model' }));

  fireEvent.click(screen.getByRole('button', { name: 'Explore' }));
  fireEvent.click(screen.getByRole('button', { name: 'Remix Template' }));
  fireEvent.click(screen.getByRole('button', { name: 'Share Exploration' }));

  fireEvent.click(screen.getByRole('button', { name: 'Cloud Runtime' }));
  fireEvent.click(screen.getByRole('button', { name: 'Start Cloud Session' }));
  fireEvent.click(screen.getByRole('button', { name: 'Pause Cloud Session' }));

  fireEvent.click(screen.getByRole('button', { name: 'System & Safety' }));
  fireEvent.click(screen.getByRole('button', { name: 'Install Office Suite' }));
  fireEvent.click(screen.getByRole('button', { name: 'Connect Slack Claw' }));
  fireEvent.click(screen.getByRole('button', { name: 'Create High-risk Approval' }));
  fireEvent.click(screen.getByRole('button', { name: 'Save UI Settings' }));
  fireEvent.click(screen.getByRole('button', { name: 'Upload Logs' }));

  await waitFor(() => {
    expect(screen.getByText('Logs uploaded')).toBeDefined();
  });
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/experts', expect.objectContaining({ method: 'PATCH' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/commands', expect.objectContaining({ method: 'POST' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/mcp', expect.objectContaining({ method: 'PATCH' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/workspaces', expect.objectContaining({ method: 'PATCH' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/models', expect.objectContaining({ method: 'PATCH' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/explore', expect.objectContaining({ method: 'POST' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/explore', expect.objectContaining({ method: 'PATCH' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/cloud', expect.objectContaining({ method: 'POST' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/cloud', expect.objectContaining({ method: 'PATCH' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/plugins', expect.objectContaining({ method: 'PATCH' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/claw', expect.objectContaining({ method: 'PATCH' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/approvals', expect.objectContaining({ method: 'POST' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/settings', expect.objectContaining({ method: 'PATCH' }));
  expect(global.fetch).toHaveBeenCalledWith('/api/assistant/support', expect.objectContaining({ method: 'POST' }));
});
