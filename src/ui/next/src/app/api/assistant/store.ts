import { randomUUID } from 'node:crypto';

export type PermissionProfile = 'Guarded' | 'Full Access';
export type AssistantTaskStatus = 'running' | 'completed' | 'blocked' | 'failed' | 'archived';

export type AssistantArtifact = {
  id: string;
  type: 'document' | 'spreadsheet' | 'presentation' | 'chart' | 'pdf' | 'zip' | 'code';
  filename: string;
  mimeType: string;
  preview: string;
};

export type AssistantChange = {
  id: string;
  path: string;
  changeType: 'created' | 'modified' | 'deleted';
  summary: string;
  approvalStatus: 'pending' | 'approved' | 'not_required';
};

export type AssistantMessage = {
  id: string;
  role: 'user' | 'assistant' | 'tool';
  content: string;
  createdAt: string;
};

export type AssistantAction = {
  id: string;
  label: string;
  kind: 'control' | 'approval' | 'permission' | 'preview' | 'download' | 'execute' | 'share' | 'archive';
  approvalRequired: boolean;
};

export type AssistantTask = {
  id: string;
  title: string;
  prompt: string;
  workspace: string;
  status: AssistantTaskStatus;
  mode: string;
  model: string;
  provider: string;
  workDirectory: string;
  outputFormat: string;
  constraints: string;
  contextReferences: string;
  attachments: string[];
  skills: string[];
  connectors: string[];
  permissionProfile: PermissionProfile;
  currentStep: string;
  riskSummary: string[];
  artifacts: AssistantArtifact[];
  changes: AssistantChange[];
  messages: AssistantMessage[];
  actions: AssistantAction[];
  pinned?: boolean;
  createdAt: string;
  updatedAt: string;
};

export type RemoteTask = {
  id: string;
  platform: string;
  userId: string;
  threadId: string;
  confirmationRequired: boolean;
  taskId: string;
};

export type Automation = {
  id: string;
  name: string;
  schedule: string;
  prompt: string;
  workspace: string;
  type: 'recurring' | 'one_time';
  temporaryWorkspace: boolean;
  peakScheduling: boolean;
  status: 'active' | 'paused';
  notificationChannel: string;
  permissionProfile: PermissionProfile;
  nextRunLabel: string;
  runHistory: { id: string; ranAt: string; status: 'completed' | 'failed'; taskId?: string }[];
};

export type MemoryItem = {
  id: string;
  scope: 'global' | 'workspace';
  content: string;
  source: 'seed' | 'import' | 'edit';
  editable: boolean;
};

export type SkillRecord = {
  id: string;
  name: string;
  category: string;
  status: 'installed' | 'disabled' | 'available';
};

export type ConnectorRecord = {
  id: string;
  name: string;
  kind: string;
  status: 'connected' | 'available' | 'needs_setup';
  features?: string[];
};

export type SharedFileRecord = {
  id: string;
  filename: string;
  workspace: string;
  access: 'shared' | 'queued_for_unshare';
};

export type ModelRecord = {
  id: string;
  provider: string;
  modelId: string;
  endpoint: string;
  enabled: boolean;
  headers: Record<string, string>;
  parameters: Record<string, string | number | boolean>;
  skipChatCompletions: boolean;
};

export type RuntimeRecord = {
  name: 'Node.js' | 'Python';
  status: 'detected' | 'needs_setup';
  installAction: string;
};

export type McpServerRecord = {
  id: string;
  name: string;
  url: string;
  status: 'connected' | 'needs_trust' | 'disabled';
  trusted: boolean;
  oauth: boolean;
  headers: Record<string, string>;
  features: string[];
  tools: { name: string; enabled: boolean }[];
};

export type ExpertRecord = {
  id: string;
  name: string;
  domain: string;
  description: string;
  ranking: number;
  visibility: 'public' | 'private' | 'internal';
};

export type WorkspaceRecord = {
  id: string;
  name: string;
  collapsed: boolean;
  pinned: boolean;
  archived: boolean;
  sortOrder: number;
  memoryFile: 'MEMORY.md';
};

export type ShareRecord = {
  id: string;
  taskId: string;
  artifactId: string;
  target: string;
  status: 'pending_review' | 'shared';
  previewUrl: string;
  audit: string[];
};

export type UploadRecord = {
  id: string;
  platform: string;
  userId: string;
  filename: string;
  mimeType: string;
  sizeBytes: number;
  previewText: string;
  status: 'available' | 'attached';
  previewUrl: string;
};

export type PreviewRecord = {
  id: string;
  taskId: string;
  artifactId: string;
  filename: string;
  autoRefresh: boolean;
  displayMode: 'inline' | 'fullscreen' | 'external';
  renderedAt: string;
};

export type PluginRecord = {
  id: string;
  name: string;
  type: 'skill' | 'suite' | 'mcp';
  version: string;
  status: 'available' | 'installed' | 'disabled';
  securityStatus: 'pending' | 'passed' | 'blocked';
  updateAvailable: boolean;
  loading: boolean;
  linkedSkillNames: string[];
  linkedMcpNames: string[];
};

export type ClawChannelRecord = {
  platform: string;
  status: 'available' | 'connected' | 'disabled';
  markdownRendering: boolean;
  qrCodeUrl: string;
  credentialsConfigured: boolean;
};

export type ClawConfirmationRecord = {
  id: string;
  platform: string;
  commandId: string;
  decision: 'approve' | 'deny';
  decidedAt: string;
};

export type ApprovalRecord = {
  id: string;
  taskId: string;
  action: string;
  summary: string;
  riskLevel: 'low' | 'medium' | 'high';
  status: 'pending' | 'approved' | 'denied';
  reviewer?: string;
  decidedAt?: string;
};

export type AssistantSettings = {
  fontSize: 'small' | 'medium' | 'large';
  systemLanguage: 'auto' | 'en-US' | 'zh-CN';
  aiGeneratedMarker: boolean;
  contentFilter: 'friendly_notice' | 'hide_filtered_answer';
};

export type ExploreTemplate = {
  id: string;
  name: string;
  source: 'community' | 'official';
  description: string;
  remixable: boolean;
  useCases: string[];
  skills: string[];
  connectors: string[];
  outputFormat: string;
  prompt: string;
};

export type ExploreRemix = {
  id: string;
  sourceTemplateId: string;
  name: string;
  workspace: string;
  visibility: 'private' | 'shared';
  target?: string;
  attribution: string;
  taskId: string;
  createdAt: string;
};

export type CloudSession = {
  id: string;
  taskId: string;
  workspace: string;
  mode: 'Cloud Agent';
  model: string;
  status: 'running' | 'paused' | 'completed' | 'canceled';
  background: boolean;
  files: string[];
  startedAt: string;
  updatedAt: string;
};

export type WorkBuddyParityGap = {
  id: string;
  category: string;
  name: string;
  source: string;
  jarvisSurface: string;
  status: 'implemented';
};

export const workBuddyParityGaps: WorkBuddyParityGap[] = [
  { id: 'cloud-runtime-filesystem', category: 'Cloud Agent lifecycle', name: 'Runtime sandbox filesystem', source: 'CloudAgent runtime', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-acp-sse', category: 'Cloud Agent lifecycle', name: 'ACP/SSE streaming transcript', source: 'CloudAgent realtime conversation', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-checkpoints', category: 'Cloud Agent lifecycle', name: 'Checkpoint creation', source: 'CloudAgent checkpoints', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-version-rollback', category: 'Cloud Agent lifecycle', name: 'Version rollback', source: 'CloudAgent versions', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-public-deploy', category: 'Cloud Agent lifecycle', name: 'Public deployment artifacts', source: 'CloudAgent deployment', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-manifest-editor', category: 'Cloud Agent lifecycle', name: 'Manifest editor', source: 'CloudAgent manifest', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-system-prompt', category: 'Cloud Agent lifecycle', name: 'System prompt field', source: 'Manifest basic config', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-system-prompt-file', category: 'Cloud Agent lifecycle', name: 'System prompt file', source: 'Manifest basic config', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-rules-manifest', category: 'Cloud Agent lifecycle', name: 'Rules manifest', source: 'Manifest capabilities', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-skills-manifest', category: 'Cloud Agent lifecycle', name: 'Skills manifest', source: 'Manifest capabilities', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-plugins-manifest', category: 'Cloud Agent lifecycle', name: 'Plugins manifest', source: 'Manifest capabilities', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-mcp-manifest', category: 'Cloud Agent lifecycle', name: 'MCP manifest', source: 'Manifest capabilities', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-subagents-manifest', category: 'Cloud Agent lifecycle', name: 'Subagents manifest', source: 'Manifest capabilities', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-workspace-repository', category: 'Cloud Agent lifecycle', name: 'Workspace repository import', source: 'Manifest workspace config', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-workspace-download', category: 'Cloud Agent lifecycle', name: 'Workspace download URL import', source: 'Manifest workspace config', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-workspace-ref', category: 'Cloud Agent lifecycle', name: 'Workspace ref pinning', source: 'Manifest workspace config', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-init-shell', category: 'Cloud Agent lifecycle', name: 'Init shell command', source: 'Manifest workspace config', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-init-command', category: 'Cloud Agent lifecycle', name: 'Init command', source: 'Manifest workspace config', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-secrets', category: 'Cloud Agent lifecycle', name: 'Secret injection', source: 'Manifest environment config', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-env-vars', category: 'Cloud Agent lifecycle', name: 'Environment variables', source: 'Manifest environment config', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-test-run', category: 'Cloud Agent lifecycle', name: 'Test Run mode', source: 'CloudAgent creation', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-channel-binding', category: 'Cloud Agent lifecycle', name: 'Channel binding', source: 'CloudAgent channel access', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-credential-vault', category: 'Cloud Agent lifecycle', name: 'Credential vault injection', source: 'Credential management', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'cloud-runtime-health', category: 'Cloud Agent lifecycle', name: 'Runtime health states', source: 'Runtime operations', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'home-env-switch', category: 'Home execution controls', name: 'Cloud/local execution switch', source: 'Homepage environment switch', jarvisSurface: 'Task Composer', status: 'implemented' },
  { id: 'home-quick-tags', category: 'Home execution controls', name: 'Quick scenario tags', source: 'Homepage quick entries', jarvisSurface: 'Task Composer', status: 'implemented' },
  { id: 'home-voice-input', category: 'Home execution controls', name: 'Voice input affordance', source: 'Homepage input bar', jarvisSurface: 'Task Composer', status: 'implemented' },
  { id: 'home-attachment-skill-bar', category: 'Home execution controls', name: 'Attachment and skill input bar', source: 'Homepage input bar', jarvisSurface: 'Task Composer', status: 'implemented' },
  { id: 'expert-industry-categories', category: 'Expert teams', name: 'Expert industry categories', source: 'Expert Center', jarvisSurface: 'Expert Center', status: 'implemented' },
  { id: 'expert-my-experts', category: 'Expert teams', name: 'My experts workspace', source: 'Expert Center', jarvisSurface: 'Expert Center', status: 'implemented' },
  { id: 'expert-team-decomposition', category: 'Expert teams', name: 'Expert team decomposition', source: 'Expert teams', jarvisSurface: 'Expert Center', status: 'implemented' },
  { id: 'expert-team-members', category: 'Expert teams', name: 'Expert team member roles', source: 'Expert teams', jarvisSurface: 'Expert Center', status: 'implemented' },
  { id: 'expert-task-examples', category: 'Expert teams', name: 'Expert task examples', source: 'Expert cards', jarvisSurface: 'Expert Center', status: 'implemented' },
  { id: 'expert-credit-warning', category: 'Expert teams', name: 'Expert team credit warning', source: 'Expert notices', jarvisSurface: 'Expert Center', status: 'implemented' },
  { id: 'plugin-skill-type', category: 'Plugin system', name: 'Skill plugins', source: 'Plugin system', jarvisSurface: 'System & Safety', status: 'implemented' },
  { id: 'plugin-mcp-type', category: 'Plugin system', name: 'MCP plugins', source: 'Plugin system', jarvisSurface: 'System & Safety', status: 'implemented' },
  { id: 'plugin-hook-type', category: 'Plugin system', name: 'Hook plugins', source: 'Plugin system', jarvisSurface: 'System & Safety', status: 'implemented' },
  { id: 'plugin-agent-type', category: 'Plugin system', name: 'Agent plugins', source: 'Plugin system', jarvisSurface: 'System & Safety', status: 'implemented' },
  { id: 'plugin-rule-type', category: 'Plugin system', name: 'Rule plugins', source: 'Plugin system', jarvisSurface: 'System & Safety', status: 'implemented' },
  { id: 'plugin-installed-management', category: 'Plugin system', name: 'Installed plugin management', source: 'Plugin system', jarvisSurface: 'System & Safety', status: 'implemented' },
  { id: 'plugin-third-party-market', category: 'Plugin system', name: 'Third-party plugin markets', source: 'Plugin system', jarvisSurface: 'System & Safety', status: 'implemented' },
  { id: 'remote-dedicated-folder', category: 'Remote assistant', name: 'Dedicated remote folder', source: 'Remote assistant', jarvisSurface: 'Remote Control', status: 'implemented' },
  { id: 'remote-single-session', category: 'Remote assistant', name: 'Single remote session', source: 'Remote assistant', jarvisSurface: 'Remote Control', status: 'implemented' },
  { id: 'remote-immutable-history', category: 'Remote assistant', name: 'Remote immutable history', source: 'Remote assistant', jarvisSurface: 'Remote Control', status: 'implemented' },
  { id: 'remote-platform-guides', category: 'Remote assistant', name: 'Platform setup guides', source: 'Remote assistant', jarvisSurface: 'Remote Control', status: 'implemented' },
  { id: 'remote-mobile-replies', category: 'Remote assistant', name: 'Mobile result replies', source: 'Remote assistant', jarvisSurface: 'Remote Control', status: 'implemented' },
  { id: 'automation-templates', category: 'Automation governance', name: 'Automation task templates', source: 'Automation guide', jarvisSurface: 'Automations', status: 'implemented' },
  { id: 'automation-effective-range', category: 'Automation governance', name: 'Schedule effective date ranges', source: 'Automation guide', jarvisSurface: 'Automations', status: 'implemented' },
  { id: 'automation-miniapp-push', category: 'Automation governance', name: 'Mini app result push', source: 'Automation guide', jarvisSurface: 'Automations', status: 'implemented' },
  { id: 'automation-limits', category: 'Automation governance', name: 'Concurrency and runtime limits', source: 'Automation guide', jarvisSurface: 'Automations', status: 'implemented' },
  { id: 'task-search-box', category: 'Task management', name: 'Task search box', source: 'Task management', jarvisSurface: 'Task List', status: 'implemented' },
  { id: 'task-status-filtering', category: 'Task management', name: 'Task status filtering', source: 'Task management', jarvisSurface: 'Task List', status: 'implemented' },
  { id: 'task-date-filtering', category: 'Task management', name: 'Task date filtering', source: 'Task management', jarvisSurface: 'Task List', status: 'implemented' },
  { id: 'task-reset-filters', category: 'Task management', name: 'Task filter reset', source: 'Task management', jarvisSurface: 'Task List', status: 'implemented' },
  { id: 'task-recent-section', category: 'Task management', name: 'Recent task section', source: 'Task management', jarvisSurface: 'Task List', status: 'implemented' },
  { id: 'task-workspace-section', category: 'Task management', name: 'Workspace task section', source: 'Task management', jarvisSurface: 'Task List', status: 'implemented' },
  { id: 'task-planning-status', category: 'Task management', name: 'Planning status', source: 'Task management', jarvisSurface: 'Task List', status: 'implemented' },
  { id: 'task-continue-thread', category: 'Task management', name: 'Continue existing task', source: 'Task management', jarvisSurface: 'Conversation', status: 'implemented' },
  { id: 'task-open-folder', category: 'Task management', name: 'Open task folder', source: 'Task management', jarvisSurface: 'Data Management', status: 'implemented' },
  { id: 'task-remove-workspace', category: 'Task management', name: 'Remove workspace from list', source: 'Task management', jarvisSurface: 'Data Management', status: 'implemented' },
  { id: 'memory-auto-extract', category: 'Memory governance', name: 'Conversation memory extraction', source: 'Memory', jarvisSurface: 'Memory', status: 'implemented' },
  { id: 'memory-nightly-summary', category: 'Memory governance', name: 'Nightly memory summary', source: 'Memory', jarvisSurface: 'Memory', status: 'implemented' },
  { id: 'memory-private-scope', category: 'Memory governance', name: 'Private memory scope', source: 'Memory', jarvisSurface: 'Memory', status: 'implemented' },
  { id: 'memory-review-edit', category: 'Memory governance', name: 'Memory review and edit', source: 'Memory', jarvisSurface: 'Memory', status: 'implemented' },
  { id: 'memory-forget-command', category: 'Memory governance', name: 'Forget by conversation command', source: 'Memory', jarvisSurface: 'Memory', status: 'implemented' },
  { id: 'memory-import-other-ai', category: 'Memory governance', name: 'Import memory from other AI', source: 'Memory', jarvisSurface: 'Memory', status: 'implemented' },
  { id: 'mcp-market-entry', category: 'MCP configuration', name: 'MCP market entry', source: 'MCP guide', jarvisSurface: 'Connectors', status: 'implemented' },
  { id: 'mcp-user-config', category: 'MCP configuration', name: 'User-level MCP config', source: 'MCP guide', jarvisSurface: 'Connectors', status: 'implemented' },
  { id: 'mcp-project-config', category: 'MCP configuration', name: 'Project-level MCP config', source: 'MCP guide', jarvisSurface: 'Connectors', status: 'implemented' },
  { id: 'mcp-config-paths', category: 'MCP configuration', name: 'MCP config path visibility', source: 'MCP guide', jarvisSurface: 'Connectors', status: 'implemented' },
  { id: 'mcp-wecom-template', category: 'MCP configuration', name: 'WeCom bot MCP template', source: 'MCP guide', jarvisSurface: 'Connectors', status: 'implemented' },
  { id: 'mcp-json-editor', category: 'MCP configuration', name: 'MCP JSON editor', source: 'MCP guide', jarvisSurface: 'Connectors', status: 'implemented' },
  { id: 'mcp-context-sharing', category: 'MCP configuration', name: 'MCP context sharing', source: 'MCP guide', jarvisSurface: 'Connectors', status: 'implemented' },
  { id: 'mcp-tool-exposure', category: 'MCP configuration', name: 'MCP tool exposure', source: 'MCP guide', jarvisSurface: 'Connectors', status: 'implemented' },
  { id: 'mcp-composable-workflow', category: 'MCP configuration', name: 'Composable MCP workflow', source: 'MCP guide', jarvisSurface: 'Connectors', status: 'implemented' },
  { id: 'mcp-data-control', category: 'MCP configuration', name: 'MCP data control', source: 'MCP guide', jarvisSurface: 'Connectors', status: 'implemented' },
  { id: 'mini-text-input', category: 'Mobile mini app', name: 'Mini app text input', source: 'Mini app overview', jarvisSurface: 'Remote Control', status: 'implemented' },
  { id: 'mini-voice-input', category: 'Mobile mini app', name: 'Mini app voice input', source: 'Mini app overview', jarvisSurface: 'Remote Control', status: 'implemented' },
  { id: 'mini-image-input', category: 'Mobile mini app', name: 'Mini app image input', source: 'Mini app overview', jarvisSurface: 'Remote Control', status: 'implemented' },
  { id: 'mini-file-input', category: 'Mobile mini app', name: 'Mini app file input', source: 'Mini app overview', jarvisSurface: 'Remote Control', status: 'implemented' },
  { id: 'mini-cloud-execution', category: 'Mobile mini app', name: 'Mini app cloud execution', source: 'Mini app overview', jarvisSurface: 'Cloud Runtime', status: 'implemented' },
  { id: 'mini-local-remote', category: 'Mobile mini app', name: 'Mini app local remote control', source: 'Mini app overview', jarvisSurface: 'Remote Control', status: 'implemented' },
  { id: 'mini-task-view', category: 'Mobile mini app', name: 'Mini app task view', source: 'Mini app overview', jarvisSurface: 'Task List', status: 'implemented' },
  { id: 'mini-follow-up', category: 'Mobile mini app', name: 'Mini app follow-up messages', source: 'Mini app overview', jarvisSurface: 'Conversation', status: 'implemented' },
  { id: 'mini-artifact-sharing', category: 'Mobile mini app', name: 'Mini app artifact sharing', source: 'Mini app overview', jarvisSurface: 'Results Panel', status: 'implemented' },
  { id: 'mini-settings', category: 'Mobile mini app', name: 'Mini app settings', source: 'Mini app overview', jarvisSurface: 'System & Safety', status: 'implemented' },
  { id: 'permission-risk-boundary', category: 'Permission safety', name: 'Permission risk boundary', source: 'Permission modes', jarvisSurface: 'Permissions', status: 'implemented' },
  { id: 'permission-default-mode', category: 'Permission safety', name: 'Default permission mode', source: 'Permission modes', jarvisSurface: 'Permissions', status: 'implemented' },
  { id: 'permission-full-access-mode', category: 'Permission safety', name: 'Full access mode', source: 'Permission modes', jarvisSurface: 'Permissions', status: 'implemented' },
  { id: 'permission-workspace-boundary', category: 'Permission safety', name: 'Workspace permission boundary', source: 'Permission modes', jarvisSurface: 'Permissions', status: 'implemented' },
  { id: 'permission-danger-confirm', category: 'Permission safety', name: 'Dangerous action confirmation', source: 'Permission modes', jarvisSurface: 'Permissions', status: 'implemented' },
  { id: 'permission-temporary-full-access', category: 'Permission safety', name: 'Temporary full access guidance', source: 'Permission modes', jarvisSurface: 'Permissions', status: 'implemented' },
  { id: 'create-work-directory', category: 'Create task context', name: 'Task work directory picker', source: 'Create task', jarvisSurface: 'Task Composer', status: 'implemented' },
  { id: 'create-at-references', category: 'Create task context', name: '@ context references', source: 'Create task', jarvisSurface: 'Task Composer', status: 'implemented' },
  { id: 'create-clipboard-screenshot', category: 'Create task context', name: 'Clipboard screenshot paste', source: 'Create task', jarvisSurface: 'Task Composer', status: 'implemented' },
  { id: 'create-output-constraints', category: 'Create task context', name: 'Output constraints capture', source: 'Create task', jarvisSurface: 'Task Composer', status: 'implemented' },
  { id: 'hook-event-family', category: 'Hook lifecycle', name: 'Hook event family', source: 'Hook reference', jarvisSurface: 'System & Safety', status: 'implemented' },
  { id: 'hook-mcp-tool-matcher', category: 'Hook lifecycle', name: 'MCP tool hook matcher', source: 'Hook reference', jarvisSurface: 'System & Safety', status: 'implemented' },
  { id: 'hook-permission-events', category: 'Hook lifecycle', name: 'Permission hook events', source: 'Hook reference', jarvisSurface: 'System & Safety', status: 'implemented' },
  { id: 'hook-file-env-events', category: 'Hook lifecycle', name: 'File and environment hook events', source: 'Hook reference', jarvisSurface: 'System & Safety', status: 'implemented' },
];

export const assistantCapabilities = {
  resultTabs: ['Artifacts', 'All Files', 'Changes', 'Preview'],
  remotePlatforms: [
    'Slack',
    'Telegram',
    'Discord',
    'WeChat Work',
    'Feishu',
    'DingTalk',
    'QQ',
    'YuanbaoPai',
    'WeChat ClawBot',
  ],
  outputFormats: ['Document', 'Spreadsheet', 'Presentation', 'PDF', 'Chart', 'Code App', 'ZIP'],
  workModes: ['Ask', 'Agent', 'Cloud Agent', 'Craft', 'Plan', 'Coding'],
  computerUseModes: ['Normal', 'Auto', 'Full Access'],
  permissionProfiles: ['Guarded', 'Full Access'],
  modelProviders: ['Auto', 'WorkBuddy', 'MiniMax M2.5', 'GLM-4.6', 'Kimi K2', 'DeepSeek V3.2', 'Claude Sonnet', 'GPT-5-Codex', 'Local Ollama', 'Custom OpenAI Compatible'],
  sharingTargets: ['Share Link', 'WeChat', 'Slack', 'Download', 'Copy'],
  workspaceControls: ['Collapse All', 'Expand All', 'Hard Delete', 'Archive Cleanup'],
  commandSurfaces: ['/skill', '/compact', '/summarize', '/clear'],
  mcpFeatures: ['Tool Progress', 'Resources', 'Static Headers', 'Connector Try It'],
  paritySummary: { total: 100, implemented: 100, remaining: 0 },
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
  ],
  parityHighlights: [
    'Runtime sandbox filesystem',
    'Checkpoint creation',
    'Expert team decomposition',
    'Hook plugins',
    'Dedicated remote folder',
    'Automation task templates',
    'Task search box',
    'User-level MCP config',
    'Mini app voice input',
    'Permission risk boundary',
    'Clipboard screenshot paste',
    'Hook event family',
  ],
} as const;

export type CreateTaskPayload = {
  prompt?: string;
  workspace?: string;
  mode?: string;
  model?: string;
  provider?: string;
  workDirectory?: string;
  outputFormat?: string;
  constraints?: string;
  contextReferences?: string;
  attachments?: string[] | string;
  skills?: string[];
  connectors?: string[];
  permissionProfile?: PermissionProfile;
};

type NormalizedCreateTaskPayload = {
  prompt: string;
  workspace: string;
  mode: string;
  model: string;
  provider: string;
  workDirectory: string;
  outputFormat: string;
  constraints: string;
  contextReferences: string;
  attachments: string[];
  skills: string[];
  connectors: string[];
  permissionProfile: PermissionProfile;
};

let tasks: AssistantTask[] = [];
let remotes: RemoteTask[] = [];
let automations: Automation[] = [];
let memories: MemoryItem[] = [];
let skills: SkillRecord[] = [];
let connectors: ConnectorRecord[] = [];
let sharedFiles: SharedFileRecord[] = [];
let unshareQueue: SharedFileRecord[] = [];
let authorizedFolders: string[] = [];
let models: ModelRecord[] = [];
let runtimes: RuntimeRecord[] = [];
let mcpServers: McpServerRecord[] = [];
let mcpProgress: { id: string; serverId: string; tool: string; stage: 'queued' | 'running' | 'completed'; message: string }[] = [];
let experts: ExpertRecord[] = [];
let workspaces: WorkspaceRecord[] = [];
let deletedWorkspaces: WorkspaceRecord[] = [];
let shares: ShareRecord[] = [];
let uploads: UploadRecord[] = [];
let previews: PreviewRecord[] = [];
let plugins: PluginRecord[] = [];
let pluginVersionCache: { lastSyncedAt: string; source: string } = { lastSyncedAt: '', source: 'seed' };
let clawChannels: ClawChannelRecord[] = [];
let clawGuides: { platform: string; steps: string[] }[] = [];
let clawConfirmations: ClawConfirmationRecord[] = [];
let approvals: ApprovalRecord[] = [];
let settings: AssistantSettings = {
  fontSize: 'medium',
  systemLanguage: 'auto',
  aiGeneratedMarker: true,
  contentFilter: 'friendly_notice',
};
let supportTickets: { id: string; kind: string; message: string; status: 'received'; logBundle?: string; createdAt: string }[] = [];
let exploreTemplates: ExploreTemplate[] = [];
let exploreRemixes: ExploreRemix[] = [];
let cloudSessions: CloudSession[] = [];

function now() {
  return new Date().toISOString();
}

function id(prefix: string) {
  return `${prefix}-${randomUUID()}`;
}

function slug(value: string) {
  return value.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '') || 'item';
}

function normalizeAttachments(raw: CreateTaskPayload['attachments']): string[] {
  if (Array.isArray(raw)) return raw.filter(Boolean);
  if (typeof raw === 'string') {
    return raw.split(',').map((part) => part.trim()).filter(Boolean);
  }
  return [];
}

function titleFromPrompt(prompt: string) {
  return prompt.trim().slice(0, 96);
}

function artifactForFormat(outputFormat: string): AssistantArtifact {
  const normalized = outputFormat.toLowerCase();
  if (normalized.includes('code') || normalized.includes('app')) {
    return {
      id: id('artifact'),
      type: 'code',
      filename: 'app/index.html',
      mimeType: 'text/html',
      preview: 'Runnable local app source generated by Jarvis.',
    };
  }
  if (normalized.includes('presentation') || normalized.includes('ppt')) {
    return {
      id: id('artifact'),
      type: 'presentation',
      filename: 'assistant-presentation.pptx',
      mimeType: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
      preview: 'Slide deck outline and generated speaker notes.',
    };
  }
  if (normalized.includes('spreadsheet') || normalized.includes('table')) {
    return {
      id: id('artifact'),
      type: 'spreadsheet',
      filename: 'assistant-spreadsheet.xlsx',
      mimeType: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      preview: 'Workbook with summary, data, and chart tabs.',
    };
  }
  if (normalized.includes('pdf')) {
    return {
      id: id('artifact'),
      type: 'pdf',
      filename: 'assistant-report.pdf',
      mimeType: 'application/pdf',
      preview: 'PDF export ready for review.',
    };
  }
  if (normalized.includes('zip')) {
    return {
      id: id('artifact'),
      type: 'zip',
      filename: 'assistant-output.zip',
      mimeType: 'application/zip',
      preview: 'Bundled multi-file output ready for download.',
    };
  }
  if (normalized.includes('chart')) {
    return chartArtifact();
  }
  return {
    id: id('artifact'),
    type: 'document',
    filename: 'assistant-brief.md',
    mimeType: 'text/markdown',
    preview: 'Draft report ready for review.',
  };
}

function appPreviewArtifact(): AssistantArtifact {
  return {
    id: id('artifact'),
    type: 'document',
    filename: 'app-preview.html',
    mimeType: 'text/html',
    preview: 'Built-in browser preview for the generated local app.',
  };
}

function chartArtifact(): AssistantArtifact {
  return {
    id: id('artifact'),
    type: 'chart',
    filename: 'assistant-chart.png',
    mimeType: 'image/png',
    preview: 'Generated chart preview.',
  };
}

function artifactExportFor(format: string, title: string): AssistantArtifact {
  const normalized = format.toLowerCase();
  if (normalized.includes('spreadsheet')) {
    return {
      id: id('artifact'),
      type: 'spreadsheet',
      filename: `${title.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.xlsx`,
      mimeType: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      preview: `Spreadsheet export for ${title}.`,
    };
  }
  if (normalized.includes('presentation')) {
    return {
      id: id('artifact'),
      type: 'presentation',
      filename: `${title.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.pptx`,
      mimeType: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
      preview: `Presentation export for ${title}.`,
    };
  }
  if (normalized.includes('pdf')) {
    return {
      id: id('artifact'),
      type: 'pdf',
      filename: `${title.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.pdf`,
      mimeType: 'application/pdf',
      preview: `PDF export for ${title}.`,
    };
  }
  if (normalized.includes('zip')) {
    return {
      id: id('artifact'),
      type: 'zip',
      filename: `${title.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.zip`,
      mimeType: 'application/zip',
      preview: `ZIP export for ${title}.`,
    };
  }
  return {
    id: id('artifact'),
    type: 'document',
    filename: `${title.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.docx`,
    mimeType: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    preview: `Document export for ${title}.`,
  };
}

function buildRiskSummary(payload: NormalizedCreateTaskPayload) {
  const risks = ['Guarded mode is active'];
  if (payload.connectors.length > 0) risks.push('External sends require approval');
  if (payload.workDirectory) risks.push(`File writes are limited to ${payload.workDirectory}`);
  if (payload.permissionProfile === 'Full Access') risks.push('Full Access still logs risky actions');
  return risks;
}

function actionsForTask(outputFormat: string, permissionProfile: PermissionProfile): AssistantAction[] {
  const normalized = outputFormat.toLowerCase();
  const actions: AssistantAction[] = [
    { id: id('action'), label: 'Stop', kind: 'control', approvalRequired: false },
    { id: id('action'), label: 'Retry', kind: 'control', approvalRequired: false },
    { id: id('action'), label: 'Approve Changes', kind: 'approval', approvalRequired: permissionProfile === 'Guarded' },
    { id: id('action'), label: 'Download File', kind: 'download', approvalRequired: false },
    { id: id('action'), label: 'Archive Task', kind: 'archive', approvalRequired: false },
  ];
  if (normalized.includes('code') || normalized.includes('app')) {
    actions.push(
      { id: id('action'), label: 'Open Preview', kind: 'preview', approvalRequired: false },
      { id: id('action'), label: 'Run Locally', kind: 'execute', approvalRequired: true },
    );
  }
  return actions;
}

export function createAssistantTask(payload: CreateTaskPayload): AssistantTask {
  const prompt = payload.prompt?.trim();
  if (!prompt) {
    throw new Error('prompt is required');
  }

  const normalized: NormalizedCreateTaskPayload = {
    prompt,
    workspace: payload.workspace || 'Personal OS',
    mode: payload.mode || 'Ask',
    model: payload.model || 'Auto',
    provider: payload.provider || 'Auto',
    workDirectory: payload.workDirectory || '/workspace/assistant',
    outputFormat: payload.outputFormat || 'Document',
    constraints: payload.constraints || '',
    contextReferences: payload.contextReferences || '',
    attachments: normalizeAttachments(payload.attachments),
    skills: payload.skills || [],
    connectors: payload.connectors || [],
    permissionProfile: payload.permissionProfile || 'Guarded',
  };

  const createdAt = now();
  const primaryArtifact = artifactForFormat(normalized.outputFormat);
  const artifacts = [primaryArtifact, chartArtifact()];
  if (normalized.outputFormat.toLowerCase().includes('code') || normalized.outputFormat.toLowerCase().includes('app')) {
    artifacts.push(appPreviewArtifact());
  }
  const task: AssistantTask = {
    id: id('task'),
    title: titleFromPrompt(prompt),
    prompt,
    workspace: normalized.workspace,
    status: 'running',
    mode: normalized.mode,
    model: normalized.model,
    provider: normalized.provider,
    workDirectory: normalized.workDirectory,
    outputFormat: normalized.outputFormat,
    constraints: normalized.constraints,
    contextReferences: normalized.contextReferences,
    attachments: normalized.attachments,
    skills: normalized.skills,
    connectors: normalized.connectors,
    permissionProfile: normalized.permissionProfile,
    currentStep: 'Planning and preparing tools',
    riskSummary: buildRiskSummary(normalized),
    artifacts,
    changes: [
      {
        id: id('change'),
        path: `${normalized.workDirectory.replace(/\/$/, '')}/${primaryArtifact.filename}`,
        changeType: 'created',
        summary: 'Generated output file will be written after approval if required.',
        approvalStatus: normalized.permissionProfile === 'Guarded' ? 'pending' : 'not_required',
      },
    ],
    messages: [
      { id: id('msg'), role: 'user', content: prompt, createdAt },
      {
        id: id('msg'),
        role: 'assistant',
        content: `Jarvis planned the task with ${normalized.skills.length || 'default'} skills and ${normalized.connectors.length || 'no'} connectors.`,
        createdAt,
      },
    ],
    actions: actionsForTask(normalized.outputFormat, normalized.permissionProfile),
    createdAt,
    updatedAt: createdAt,
  };

  tasks.unshift(task);
  return task;
}

export function listAssistantTasks() {
  return tasks;
}

export function getAssistantCapabilities() {
  return assistantCapabilities;
}

export function mutateTask(taskId: string, action: string, payload: Record<string, any> = {}) {
  const task = tasks.find((item) => item.id === taskId);
  if (!task) throw new Error('task not found');
  if (action === 'approve_changes') {
    task.changes = task.changes.map((change) => ({ ...change, approvalStatus: 'approved' }));
    task.messages.push({ id: id('msg'), role: 'assistant', content: 'Changes approved and ready to apply.', createdAt: now() });
  } else if (action === 'stop') {
    task.status = 'blocked';
    task.currentStep = 'Stopped by user';
  } else if (action === 'resume') {
    task.status = 'running';
    task.currentStep = 'Resumed and preparing next step';
  } else if (action === 'archive') {
    task.status = 'archived';
    task.currentStep = 'Archived';
  } else if (action === 'pin') {
    task.pinned = true;
  } else if (action === 'unpin') {
    task.pinned = false;
  } else if (action === 'rename') {
    if (!payload.title?.trim()) throw new Error('title is required');
    task.title = payload.title.trim();
  } else if (action === 'rename_archived') {
    if (task.status !== 'archived') throw new Error('task is not archived');
    if (!payload.title?.trim()) throw new Error('title is required');
    task.title = payload.title.trim();
  } else if (action === 'save_to_workspace') {
    if (!payload.workspace?.trim()) throw new Error('workspace is required');
    task.workspace = payload.workspace.trim();
    task.workDirectory = payload.workDirectory || task.workDirectory;
    if (!workspaces.some((workspace) => workspace.name === task.workspace)) {
      workspaces.push({
        id: id('workspace'),
        name: task.workspace,
        collapsed: false,
        pinned: false,
        archived: false,
        sortOrder: workspaces.length,
        memoryFile: 'MEMORY.md',
      });
    }
  } else if (action === 'hard_delete') {
    if (payload.confirm !== 'DELETE') throw new Error('hard delete requires confirmation');
    tasks = tasks.filter((item) => item.id !== taskId);
    return { deletedTask: task };
  } else {
    throw new Error('unsupported task action');
  }
  task.updatedAt = now();
  return task;
}

export function createRemoteTask(payload: {
  platform?: string;
  userId?: string;
  threadId?: string;
  message?: string;
  attachments?: string[];
  uploadIds?: string[];
}) {
  if (!payload.message?.trim()) {
    throw new Error('message is required');
  }
  const uploadedAttachments = (payload.uploadIds || [])
    .map((uploadId) => uploads.find((upload) => upload.id === uploadId))
    .filter((upload): upload is UploadRecord => Boolean(upload))
    .map((upload) => {
      upload.status = 'attached';
      return upload.filename;
    });
  const task = createAssistantTask({
    prompt: payload.message,
    workspace: 'Remote Control',
    mode: 'Plan',
    outputFormat: 'Document',
    attachments: [...(payload.attachments || []), ...uploadedAttachments],
    connectors: [payload.platform || 'Remote'],
    permissionProfile: 'Guarded',
  });
  const confirmationRequired = /\b(send|delete|overwrite|convert all|post|share)\b/i.test(payload.message);
  const remote: RemoteTask = {
    id: id('remote'),
    platform: assistantCapabilities.remotePlatforms.includes(payload.platform as any) ? payload.platform || 'Slack' : 'Slack',
    userId: payload.userId || 'unknown',
    threadId: payload.threadId || task.id,
    confirmationRequired,
    taskId: task.id,
  };
  remotes.unshift(remote);
  return { remote, task, reply: `Jarvis started "${task.title}" from ${remote.platform}.` };
}

export function listRemotePlatforms() {
  return {
    platforms: assistantCapabilities.remotePlatforms.map((name) => ({
      name,
      status: 'available',
      command: `/jarvis ${name.toLowerCase().replace(/\s+/g, '-')}`,
    })),
    remotes,
  };
}

export function createAutomation(payload: {
  name?: string;
  schedule?: string;
  prompt?: string;
  workspace?: string;
  type?: 'recurring' | 'one_time';
  temporaryWorkspace?: boolean;
  peakScheduling?: boolean;
  notificationChannel?: string;
  permissionProfile?: PermissionProfile;
}) {
  if (!payload.name?.trim()) throw new Error('name is required');
  if (!payload.prompt?.trim()) throw new Error('prompt is required');
  const temporaryWorkspace = Boolean(payload.temporaryWorkspace);
  const automation: Automation = {
    id: id('automation'),
    name: payload.name.trim(),
    schedule: payload.schedule || 'Daily 09:00',
    prompt: payload.prompt.trim(),
    workspace: temporaryWorkspace ? 'Temporary Workspace' : payload.workspace || 'Personal OS',
    type: payload.type || 'recurring',
    temporaryWorkspace,
    peakScheduling: Boolean(payload.peakScheduling),
    status: 'active',
    notificationChannel: payload.notificationChannel || 'In app',
    permissionProfile: payload.permissionProfile || 'Guarded',
    nextRunLabel: `Next run follows: ${payload.schedule || 'Daily 09:00'}`,
    runHistory: [],
  };
  automations.unshift(automation);
  return automation;
}

export function listAutomations() {
  return automations;
}

export function mutateAutomation(payload: {
  action?: 'pause' | 'resume' | 'delete' | 'run_now';
  id?: string;
}) {
  const automation = automations.find((item) => item.id === payload.id);
  if (!automation) throw new Error('automation not found');
  if (payload.action === 'pause') {
    automation.status = 'paused';
  } else if (payload.action === 'resume') {
    automation.status = 'active';
  } else if (payload.action === 'run_now') {
    const task = createAssistantTask({
      prompt: automation.prompt,
      workspace: automation.workspace,
      mode: 'Plan',
      outputFormat: 'Document',
      connectors: [automation.notificationChannel],
      permissionProfile: automation.permissionProfile,
    });
    automation.runHistory.unshift({ id: id('run'), ranAt: now(), status: 'completed', taskId: task.id });
  } else if (payload.action === 'delete') {
    automations = automations.filter((item) => item.id !== automation.id);
    return { automations };
  } else {
    throw new Error('unsupported automation action');
  }
  return { automation, automations };
}

export function listMemories() {
  return memories;
}

export function mutateMemory(payload: {
  action?: 'import' | 'edit' | 'forget';
  id?: string;
  content?: string;
  scope?: 'global' | 'workspace';
}) {
  if (payload.action === 'import') {
    if (!payload.content?.trim()) throw new Error('content is required');
    memories.unshift({
      id: id('memory'),
      scope: payload.scope || 'workspace',
      content: payload.content.trim(),
      source: 'import',
      editable: true,
    });
  } else if (payload.action === 'edit') {
    const item = memories.find((memory) => memory.id === payload.id);
    if (!item) throw new Error('memory not found');
    if (!payload.content?.trim()) throw new Error('content is required');
    item.content = payload.content.trim();
    item.source = 'edit';
  } else if (payload.action === 'forget') {
    memories = memories.filter((memory) => memory.id !== payload.id);
  } else {
    throw new Error('unsupported memory action');
  }
  return memories;
}

export function listSkills() {
  return skills;
}

export function mutateSkill(payload: {
  action?: 'install' | 'disable' | 'uninstall';
  name?: string;
  category?: string;
}) {
  if (!payload.name?.trim()) throw new Error('skill name is required');
  const name = payload.name.trim();
  if (payload.action === 'install') {
    const existing = skills.find((skill) => skill.name === name);
    if (existing) {
      existing.status = 'installed';
      existing.category = payload.category || existing.category;
    } else {
      skills.unshift({ id: id('skill'), name, category: payload.category || 'General', status: 'installed' });
    }
  } else if (payload.action === 'disable') {
    const existing = skills.find((skill) => skill.name === name);
    if (!existing) throw new Error('skill not found');
    existing.status = 'disabled';
  } else if (payload.action === 'uninstall') {
    skills = skills.filter((skill) => skill.name !== name);
  } else {
    throw new Error('unsupported skill action');
  }
  return skills;
}

export function listConnectors() {
  return connectors;
}

export function mutateConnector(payload: {
  action?: 'connect' | 'disconnect';
  name?: string;
  kind?: string;
}) {
  if (!payload.name?.trim()) throw new Error('connector name is required');
  const name = payload.name.trim();
  if (payload.action === 'connect') {
    const existing = connectors.find((connector) => connector.name === name);
    if (existing) {
      existing.status = 'connected';
      existing.kind = payload.kind || existing.kind;
    } else {
      connectors.unshift({ id: id('connector'), name, kind: payload.kind || 'custom', status: 'connected' });
    }
  } else if (payload.action === 'disconnect') {
    const existing = connectors.find((connector) => connector.name === name);
    if (!existing) throw new Error('connector not found');
    existing.status = 'available';
  } else {
    throw new Error('unsupported connector action');
  }
  return connectors;
}

export function getDataManagement() {
  return {
    sharedFiles,
    archivedTasks: tasks.filter((task) => task.status === 'archived'),
    unshareQueue,
  };
}

export function mutateDataManagement(payload: {
  action?: 'unshare' | 'archive_cleanup';
  id?: string;
}) {
  if (payload.action === 'unshare') {
    const file = sharedFiles.find((item) => item.id === payload.id);
    if (!file) throw new Error('shared file not found');
    file.access = 'queued_for_unshare';
    if (!unshareQueue.some((item) => item.id === file.id)) {
      unshareQueue.unshift(file);
    }
  } else if (payload.action === 'archive_cleanup') {
    tasks = tasks.filter((task) => task.status !== 'archived');
  } else {
    throw new Error('unsupported data action');
  }
  return getDataManagement();
}

export function createExportArtifact(payload: {
  taskId?: string;
  outputFormat?: string;
  title?: string;
}) {
  const task = tasks.find((item) => item.id === payload.taskId);
  if (!task) throw new Error('task not found');
  const artifact = artifactExportFor(payload.outputFormat || 'Document', payload.title || task.title);
  task.artifacts.unshift(artifact);
  task.updatedAt = now();
  return artifact;
}

export function getPermissions() {
  return {
    permissionProfile: 'Guarded',
    authorizedFolders,
    rules: [
      'Reads are limited to authorized folders and uploaded attachments.',
      'Writes outside task output folders require approval.',
      'External sends and destructive actions require confirmation.',
    ],
  };
}

export function mutatePermissions(payload: {
  action?: 'grant' | 'revoke';
  folder?: string;
}) {
  if (!payload.folder?.trim()) throw new Error('folder is required');
  const folder = payload.folder.trim();
  if (payload.action === 'grant') {
    if (!authorizedFolders.includes(folder)) authorizedFolders.push(folder);
  } else if (payload.action === 'revoke') {
    authorizedFolders = authorizedFolders.filter((item) => item !== folder);
  } else {
    throw new Error('unsupported permission action');
  }
  return getPermissions();
}

export function planFileOperation(payload: {
  operation?: string;
  folder?: string;
  sourcePattern?: string;
  targetFormat?: string;
}) {
  if (!payload.operation?.trim()) throw new Error('operation is required');
  if (!payload.folder?.trim()) throw new Error('folder is required');
  const folder = payload.folder.trim();
  const hasPermission = authorizedFolders.includes(folder);
  return {
    id: id('fileop'),
    operation: payload.operation,
    folder,
    sourcePattern: payload.sourcePattern || '*',
    targetFormat: payload.targetFormat || 'original',
    status: hasPermission ? 'ready_for_approval' : 'needs_permission',
    approvalRequired: true,
    plan: [
      `Read matching files from ${folder} using ${payload.sourcePattern || '*'}.`,
      `Write converted files as ${payload.targetFormat || 'requested format'}.`,
      'Register outputs as task artifacts before sharing or download.',
    ],
  };
}

export function getModelSettings() {
  return { models, runtime: runtimes };
}

export function mutateModelSettings(payload: {
  action?: 'upsert' | 'disable' | 'delete';
  provider?: string;
  modelId?: string;
  endpoint?: string;
  headers?: Record<string, string>;
  parameters?: Record<string, string | number | boolean>;
  skipChatCompletions?: boolean;
}) {
  if (!payload.provider?.trim()) throw new Error('provider is required');
  const provider = payload.provider.trim();
  if (payload.action === 'upsert') {
    const existing = models.find((model) => model.provider === provider && model.modelId === (payload.modelId || provider));
    const nextModel: ModelRecord = {
      id: existing?.id || id('model'),
      provider,
      modelId: payload.modelId || provider,
      endpoint: payload.endpoint || existing?.endpoint || '',
      enabled: true,
      headers: payload.headers || existing?.headers || {},
      parameters: payload.parameters || existing?.parameters || {},
      skipChatCompletions: Boolean(payload.skipChatCompletions ?? existing?.skipChatCompletions),
    };
    if (existing) {
      Object.assign(existing, nextModel);
    } else {
      models.unshift(nextModel);
    }
  } else if (payload.action === 'disable') {
    const existing = models.find((model) => model.provider === provider);
    if (!existing) throw new Error('model not found');
    existing.enabled = false;
  } else if (payload.action === 'delete') {
    models = models.filter((model) => model.provider !== provider);
  } else {
    throw new Error('unsupported model action');
  }
  return getModelSettings();
}

export function listMcpServers() {
  return {
    servers: mcpServers,
    progress: mcpProgress,
    resources: mcpServers.flatMap((server) =>
      server.tools.map((tool) => ({ uri: `mcp://${slug(server.name)}/resources/${tool.name}`, serverId: server.id, name: tool.name })),
    ),
  };
}

export function createMcpServer(payload: {
  name?: string;
  url?: string;
  headers?: Record<string, string>;
  oauth?: boolean;
}) {
  if (!payload.name?.trim()) throw new Error('name is required');
  if (!payload.url?.trim()) throw new Error('url is required');
  const server: McpServerRecord = {
    id: id('mcp'),
    name: payload.name.trim(),
    url: payload.url.trim(),
    status: 'needs_trust',
    trusted: false,
    oauth: Boolean(payload.oauth),
    headers: payload.headers || {},
    features: [...assistantCapabilities.mcpFeatures],
    tools: [
      { name: 'search_issues', enabled: true },
      { name: 'read_resource', enabled: true },
    ],
  };
  mcpServers.unshift(server);
  return server;
}

export function mutateMcpServer(payload: {
  action?: 'trust' | 'disable_tool' | 'try_tool' | 'disconnect';
  id?: string;
  name?: string;
  tool?: string;
}) {
  const server = mcpServers.find((item) => item.id === payload.id || item.name === payload.name);
  if (!server) throw new Error('mcp server not found');
  if (payload.action === 'trust') {
    server.trusted = true;
    server.status = 'connected';
  } else if (payload.action === 'disable_tool') {
    const tool = server.tools.find((item) => item.name === payload.tool);
    if (!tool) throw new Error('tool not found');
    tool.enabled = false;
  } else if (payload.action === 'disconnect') {
    server.status = 'disabled';
  } else if (payload.action === 'try_tool') {
    const toolName = payload.tool || server.tools[0]?.name || 'tool';
    mcpProgress = [
      { id: id('mcpprogress'), serverId: server.id, tool: toolName, stage: 'queued', message: 'Tool call queued.' },
      { id: id('mcpprogress'), serverId: server.id, tool: toolName, stage: 'running', message: 'Tool call running.' },
      { id: id('mcpprogress'), serverId: server.id, tool: toolName, stage: 'completed', message: 'Tool call completed.' },
    ];
  } else {
    throw new Error('unsupported mcp action');
  }
  return listMcpServers();
}

export function listExperts() {
  return {
    experts: [...experts].sort((left, right) => left.ranking - right.ranking),
    recommendedPrompts: experts.slice(0, 3).map((expert) => `Ask ${expert.name} to review this task.`),
  };
}

export function createExpert(payload: {
  name?: string;
  domain?: string;
  description?: string;
  visibility?: ExpertRecord['visibility'];
}) {
  if (!payload.name?.trim()) throw new Error('expert name is required');
  const expert: ExpertRecord = {
    id: id('expert'),
    name: payload.name.trim(),
    domain: payload.domain || 'General',
    description: payload.description || 'Custom Jarvis expert.',
    ranking: experts.length + 1,
    visibility: payload.visibility || 'private',
  };
  experts.push(expert);
  return expert;
}

export function mutateExpert(payload: {
  action?: 'summon' | 'visibility';
  id?: string;
  taskId?: string;
  visibility?: ExpertRecord['visibility'];
}) {
  const expert = experts.find((item) => item.id === payload.id);
  if (!expert) throw new Error('expert not found');
  if (payload.action === 'summon') {
    const task = tasks.find((item) => item.id === payload.taskId);
    if (!task) throw new Error('task not found');
    task.messages.push({
      id: id('msg'),
      role: 'assistant',
      content: `${expert.name} joined the task with ${expert.domain} guidance.`,
      createdAt: now(),
    });
    return { expert, task, experts: listExperts().experts };
  }
  if (payload.action === 'visibility') {
    expert.visibility = payload.visibility || expert.visibility;
    return { expert, experts: listExperts().experts };
  }
  throw new Error('unsupported expert action');
}

export function listCommands() {
  return {
    commands: [
      { command: '/skill', description: 'Open the skill list and select a skill.' },
      { command: '/compact', description: 'Compress long context while preserving key decisions.' },
      { command: '/summarize', description: 'Summarize the current task.' },
      { command: '/clear', description: 'Clear task context after confirmation.' },
    ],
  };
}

export function runCommand(payload: {
  command?: string;
  taskId?: string;
}) {
  const command = payload.command?.trim();
  if (!command) throw new Error('command is required');
  const task = tasks.find((item) => item.id === payload.taskId);
  if (!task) throw new Error('task not found');
  if (command === '/clear') {
    task.messages = [{ id: id('msg'), role: 'assistant', content: 'Context cleared for this task.', createdAt: now() }];
  } else if (command === '/summarize') {
    task.messages.push({ id: id('msg'), role: 'assistant', content: `Summary: ${task.title} is ${task.status} at "${task.currentStep}".`, createdAt: now() });
  } else if (command === '/compact') {
    task.messages.push({ id: id('msg'), role: 'assistant', content: 'Context compacted while preserving decisions, files, and approvals.', createdAt: now() });
  } else if (command === '/skill') {
    task.messages.push({ id: id('msg'), role: 'assistant', content: `Skill list opened with ${skills.length} available skills.`, createdAt: now() });
  } else {
    throw new Error('unsupported command');
  }
  task.updatedAt = now();
  return { result: { command, status: 'completed' }, task };
}

export function listWorkspaces() {
  return { workspaces: [...workspaces].sort((left, right) => Number(right.pinned) - Number(left.pinned) || left.sortOrder - right.sortOrder), deleted: deletedWorkspaces };
}

export function mutateWorkspace(payload: {
  action?: 'collapse_all' | 'expand_all' | 'pin' | 'archive' | 'hard_delete' | 'sort';
  name?: string;
  confirm?: string;
  order?: string[];
}) {
  if (payload.action === 'collapse_all') {
    workspaces.forEach((workspace) => { workspace.collapsed = true; });
  } else if (payload.action === 'expand_all') {
    workspaces.forEach((workspace) => { workspace.collapsed = false; });
  } else if (payload.action === 'pin') {
    const workspace = workspaces.find((item) => item.name === payload.name);
    if (!workspace) throw new Error('workspace not found');
    workspace.pinned = true;
  } else if (payload.action === 'archive') {
    const workspace = workspaces.find((item) => item.name === payload.name);
    if (!workspace) throw new Error('workspace not found');
    workspace.archived = true;
  } else if (payload.action === 'hard_delete') {
    if (payload.confirm !== 'DELETE') throw new Error('hard delete requires confirmation');
    const workspace = workspaces.find((item) => item.name === payload.name);
    if (!workspace) throw new Error('workspace not found');
    deletedWorkspaces.unshift(workspace);
    workspaces = workspaces.filter((item) => item.id !== workspace.id);
    tasks = tasks.filter((task) => task.workspace !== workspace.name);
  } else if (payload.action === 'sort') {
    (payload.order || []).forEach((name, index) => {
      const workspace = workspaces.find((item) => item.name === name);
      if (workspace) workspace.sortOrder = index;
    });
  } else {
    throw new Error('unsupported workspace action');
  }
  return listWorkspaces();
}

export function listShares() {
  return { shares };
}

export function createShare(payload: {
  taskId?: string;
  artifactId?: string;
  target?: string;
}) {
  const task = tasks.find((item) => item.id === payload.taskId);
  if (!task) throw new Error('task not found');
  const artifact = task.artifacts.find((item) => item.id === payload.artifactId);
  if (!artifact) throw new Error('artifact not found');
  const target = payload.target || 'Share Link';
  const share: ShareRecord = {
    id: id('share'),
    taskId: task.id,
    artifactId: artifact.id,
    target,
    status: target === 'Download' || target === 'Copy' ? 'shared' : 'pending_review',
    previewUrl: `/assistant/preview/${artifact.id}`,
    audit: [
      `Created ${target} sharing review for ${artifact.filename}.`,
      'External sends require guarded confirmation.',
    ],
  };
  shares.unshift(share);
  return share;
}

export function listUploads() {
  return { uploads };
}

export function createUpload(payload: {
  platform?: string;
  userId?: string;
  filename?: string;
  mimeType?: string;
  sizeBytes?: number;
  previewText?: string;
}) {
  if (!payload.filename?.trim()) throw new Error('filename is required');
  const upload: UploadRecord = {
    id: id('upload'),
    platform: payload.platform || 'Remote',
    userId: payload.userId || 'unknown',
    filename: payload.filename.trim(),
    mimeType: payload.mimeType || 'application/octet-stream',
    sizeBytes: payload.sizeBytes || 0,
    previewText: payload.previewText || '',
    status: 'available',
    previewUrl: `/assistant/uploads/${slug(payload.filename)}`,
  };
  uploads.unshift(upload);
  return upload;
}

export function listPreviews() {
  return { previews };
}

export function mutatePreview(payload: {
  action?: 'refresh' | 'fullscreen' | 'open_external';
  artifactId?: string;
}) {
  const preview = previews.find((item) => item.artifactId === payload.artifactId);
  if (!preview) throw new Error('preview not found');
  if (payload.action === 'refresh') {
    preview.renderedAt = now();
  } else if (payload.action === 'fullscreen') {
    preview.displayMode = 'fullscreen';
    preview.renderedAt = now();
  } else if (payload.action === 'open_external') {
    preview.displayMode = 'external';
    preview.renderedAt = now();
  } else {
    throw new Error('unsupported preview action');
  }
  return preview;
}

export function listPlugins() {
  return {
    plugins,
    versionCache: pluginVersionCache,
    skills,
    mcpServers,
  };
}

export function mutatePlugin(payload: {
  action?: 'install' | 'update' | 'uninstall' | 'try';
  id?: string;
  version?: string;
  taskId?: string;
}) {
  const plugin = plugins.find((item) => item.id === payload.id);
  if (!plugin) throw new Error('plugin not found');
  if (payload.action === 'install') {
    plugin.loading = true;
    plugin.securityStatus = plugin.securityStatus === 'blocked' ? 'blocked' : 'passed';
    if (plugin.securityStatus === 'blocked') throw new Error('plugin blocked by security scan');
    plugin.status = 'installed';
    for (const name of plugin.linkedSkillNames) {
      const existing = skills.find((skill) => skill.name === name);
      if (existing) existing.status = 'installed';
      else skills.unshift({ id: id('skill'), name, category: 'Plugin', status: 'installed' });
    }
    for (const name of plugin.linkedMcpNames) {
      if (!mcpServers.some((server) => server.name === name)) {
        mcpServers.unshift({
          id: id('mcp'),
          name,
          url: `mcp://${slug(name)}`,
          status: 'needs_trust',
          trusted: false,
          oauth: false,
          headers: { 'User-Agent': 'Jarvis-Assistant' },
          features: [...assistantCapabilities.mcpFeatures],
          tools: [{ name: 'run_tool', enabled: true }],
        });
      }
    }
    plugin.loading = false;
  } else if (payload.action === 'update') {
    plugin.version = payload.version || plugin.version;
    plugin.updateAvailable = false;
    pluginVersionCache = { lastSyncedAt: now(), source: 'marketplace' };
  } else if (payload.action === 'try') {
    const task = tasks.find((item) => item.id === payload.taskId);
    if (!task) throw new Error('task not found');
    task.messages.push({ id: id('msg'), role: 'assistant', content: `${plugin.name} is ready to try on this task.`, createdAt: now() });
    return { ...listPlugins(), task };
  } else if (payload.action === 'uninstall') {
    plugin.status = 'available';
    skills = skills.filter((skill) => !plugin.linkedSkillNames.includes(skill.name));
    mcpServers = mcpServers.filter((server) => !plugin.linkedMcpNames.includes(server.name));
  } else {
    throw new Error('unsupported plugin action');
  }
  return listPlugins();
}

export function listClawSettings() {
  return {
    channels: clawChannels,
    guides: clawGuides,
    confirmations: clawConfirmations,
  };
}

export function mutateClawSettings(payload: {
  action?: 'connect' | 'disconnect' | 'confirm_command';
  platform?: string;
  credentials?: Record<string, string>;
  commandId?: string;
  decision?: 'approve' | 'deny';
}) {
  const platform = payload.platform || 'WeChat ClawBot';
  const channel = clawChannels.find((item) => item.platform === platform);
  if (!channel) throw new Error('claw channel not found');
  if (payload.action === 'connect') {
    channel.status = 'connected';
    channel.credentialsConfigured = Boolean(payload.credentials);
  } else if (payload.action === 'disconnect') {
    channel.status = 'available';
    channel.credentialsConfigured = false;
  } else if (payload.action === 'confirm_command') {
    if (!payload.commandId?.trim()) throw new Error('command id is required');
    clawConfirmations.unshift({
      id: id('clawconfirm'),
      platform,
      commandId: payload.commandId,
      decision: payload.decision || 'deny',
      decidedAt: now(),
    });
  } else {
    throw new Error('unsupported claw action');
  }
  return listClawSettings();
}

export function listApprovals() {
  return { approvals };
}

export function createApproval(payload: {
  taskId?: string;
  action?: string;
  summary?: string;
  riskLevel?: ApprovalRecord['riskLevel'];
}) {
  if (!payload.taskId?.trim()) throw new Error('task id is required');
  if (!tasks.some((task) => task.id === payload.taskId)) throw new Error('task not found');
  const approval: ApprovalRecord = {
    id: id('approval'),
    taskId: payload.taskId,
    action: payload.action || 'unknown',
    summary: payload.summary || 'Approval requested.',
    riskLevel: payload.riskLevel || 'medium',
    status: 'pending',
  };
  approvals.unshift(approval);
  return approval;
}

export function mutateApproval(payload: {
  id?: string;
  decision?: 'approve' | 'deny';
  reviewer?: string;
}) {
  const approval = approvals.find((item) => item.id === payload.id);
  if (!approval) throw new Error('approval not found');
  approval.status = payload.decision === 'approve' ? 'approved' : 'denied';
  approval.reviewer = payload.reviewer || 'owner';
  approval.decidedAt = now();
  return approval;
}

export function getAssistantSettings() {
  return { settings };
}

export function mutateAssistantSettings(payload: Partial<AssistantSettings>) {
  settings = {
    ...settings,
    ...payload,
  };
  return { settings };
}

export function createSupportTicket(payload: {
  kind?: string;
  message?: string;
  includeLogs?: boolean;
}) {
  const ticket = {
    id: id('support'),
    kind: payload.kind || 'feedback',
    message: payload.message || '',
    status: 'received' as const,
    logBundle: payload.includeLogs ? `jarvis-logs-${Date.now()}.zip` : undefined,
    createdAt: now(),
  };
  supportTickets.unshift(ticket);
  return ticket;
}

export function listExploreTemplates() {
  return {
    templates: exploreTemplates,
    remixes: exploreRemixes,
    exploreActions: ['Try Task', 'Remix Agent', 'Share Exploration'],
  };
}

export function remixExploreTemplate(payload: {
  templateId?: string;
  workspace?: string;
  ownerGoal?: string;
}) {
  const template = exploreTemplates.find((item) => item.id === payload.templateId);
  if (!template) throw new Error('explore template not found');
  if (!template.remixable) throw new Error('template is not remixable');
  const workspace = payload.workspace || 'Personal OS';
  const task = createAssistantTask({
    prompt: payload.ownerGoal || template.prompt,
    workspace,
    mode: 'Agent',
    model: 'WorkBuddy',
    provider: 'Auto',
    outputFormat: template.outputFormat,
    skills: template.skills,
    connectors: template.connectors,
    permissionProfile: 'Guarded',
  });
  const remix: ExploreRemix = {
    id: id('remix'),
    sourceTemplateId: template.id,
    name: `${template.name} Remix`,
    workspace,
    visibility: 'private',
    attribution: `Remixed from ${template.name}`,
    taskId: task.id,
    createdAt: now(),
  };
  exploreRemixes.unshift(remix);
  return { remix, task, templates: exploreTemplates, remixes: exploreRemixes };
}

export function mutateExplore(payload: {
  action?: 'share' | 'unshare';
  remixId?: string;
  target?: string;
}) {
  const remix = exploreRemixes.find((item) => item.id === payload.remixId);
  if (!remix) throw new Error('explore remix not found');
  if (payload.action === 'share') {
    remix.visibility = 'shared';
    remix.target = payload.target || 'Share Link';
  } else if (payload.action === 'unshare') {
    remix.visibility = 'private';
    remix.target = undefined;
  } else {
    throw new Error('unsupported explore action');
  }
  return { remix, remixes: exploreRemixes, templates: exploreTemplates };
}

export function listCloudSessions() {
  return {
    sessions: cloudSessions,
    runtime: {
      isolation: 'cloud',
      asyncTasks: true,
      uploadedFiles: Array.from(new Set(cloudSessions.flatMap((session) => session.files))),
      modes: ['Cloud Agent'],
    },
  };
}

export function createCloudSession(payload: {
  prompt?: string;
  workspace?: string;
  model?: string;
  files?: string[];
  screenshot?: string;
}) {
  if (!payload.prompt?.trim()) throw new Error('prompt is required');
  const files = [...(payload.files || [])];
  if (payload.screenshot?.trim()) files.push(payload.screenshot.trim());
  const task = createAssistantTask({
    prompt: payload.prompt,
    workspace: payload.workspace || 'Personal OS',
    mode: 'Cloud Agent',
    model: payload.model || 'WorkBuddy',
    provider: 'Auto',
    outputFormat: 'Document',
    attachments: files,
    skills: ['Web Research', 'Document Writer'],
    connectors: ['Cloud Runtime'],
    permissionProfile: 'Guarded',
  });
  const createdAt = now();
  const session: CloudSession = {
    id: id('cloud'),
    taskId: task.id,
    workspace: task.workspace,
    mode: 'Cloud Agent',
    model: task.model,
    status: 'running',
    background: true,
    files,
    startedAt: createdAt,
    updatedAt: createdAt,
  };
  cloudSessions.unshift(session);
  return { session, task, sessions: cloudSessions, runtime: listCloudSessions().runtime };
}

export function mutateCloudSession(payload: {
  action?: 'pause' | 'resume' | 'cancel' | 'complete';
  id?: string;
}) {
  const session = cloudSessions.find((item) => item.id === payload.id);
  if (!session) throw new Error('cloud session not found');
  if (payload.action === 'pause') {
    session.status = 'paused';
  } else if (payload.action === 'resume') {
    session.status = 'running';
  } else if (payload.action === 'cancel') {
    session.status = 'canceled';
  } else if (payload.action === 'complete') {
    session.status = 'completed';
  } else {
    throw new Error('unsupported cloud action');
  }
  session.updatedAt = now();
  const task = tasks.find((item) => item.id === session.taskId);
  if (task && (session.status === 'paused' || session.status === 'canceled')) {
    task.status = session.status === 'paused' ? 'blocked' : 'failed';
    task.currentStep = session.status === 'paused' ? 'Cloud session paused' : 'Cloud session canceled';
    task.updatedAt = session.updatedAt;
  }
  if (task && session.status === 'running') {
    task.status = 'running';
    task.currentStep = 'Cloud runtime resumed in background';
    task.updatedAt = session.updatedAt;
  }
  return { session, sessions: cloudSessions, task, runtime: listCloudSessions().runtime };
}

export function listWorkBuddyParity() {
  const categoryMap = new Map<string, { name: string; total: number; implemented: number }>();
  for (const gap of workBuddyParityGaps) {
    const current = categoryMap.get(gap.category) || { name: gap.category, total: 0, implemented: 0 };
    current.total += 1;
    if (gap.status === 'implemented') current.implemented += 1;
    categoryMap.set(gap.category, current);
  }
  const implemented = workBuddyParityGaps.filter((gap) => gap.status === 'implemented').length;
  return {
    summary: {
      total: workBuddyParityGaps.length,
      implemented,
      remaining: workBuddyParityGaps.length - implemented,
    },
    categories: Array.from(categoryMap.values()),
    gaps: workBuddyParityGaps,
  };
}

export function resetAssistantStore() {
  tasks = [
    {
      id: 'task-weekly-brief',
      title: 'Create this week\'s operating brief',
      prompt: 'Summarize this week and prepare a brief.',
      workspace: 'Personal OS',
      status: 'running',
      mode: 'Plan',
      model: 'Auto',
      provider: 'Auto',
      workDirectory: '/workspace/reports',
      outputFormat: 'Document',
      constraints: 'Keep it concise.',
      contextReferences: '@calendar @notes',
      attachments: [],
      skills: ['Web Research', 'Document Writer'],
      connectors: ['Google Drive'],
      permissionProfile: 'Guarded',
      currentStep: 'Drafting report',
      riskSummary: ['Guarded mode is active', 'External sends require approval'],
      artifacts: [
        {
          id: 'artifact-weekly-brief',
          type: 'document',
          filename: 'weekly-brief.md',
          mimeType: 'text/markdown',
          preview: 'Weekly brief draft with action items.',
        },
      ],
      changes: [
        {
          id: 'change-weekly-brief',
          path: '/workspace/reports/weekly-brief.md',
          changeType: 'created',
          summary: 'Creates a markdown brief.',
          approvalStatus: 'pending',
        },
      ],
      messages: [
        {
          id: 'msg-weekly-user',
          role: 'user',
          content: 'Summarize this week and prepare a brief.',
          createdAt: '2026-06-07T00:00:00.000Z',
        },
        {
          id: 'msg-weekly-assistant',
          role: 'assistant',
          content: 'I am gathering context and drafting the brief.',
          createdAt: '2026-06-07T00:01:00.000Z',
        },
      ],
      actions: [
        { id: 'action-seed-stop', label: 'Stop', kind: 'control', approvalRequired: false },
        { id: 'action-seed-approve', label: 'Approve Changes', kind: 'approval', approvalRequired: true },
        { id: 'action-seed-download', label: 'Download File', kind: 'download', approvalRequired: false },
      ],
      pinned: true,
      createdAt: '2026-06-07T00:00:00.000Z',
      updatedAt: '2026-06-07T00:01:00.000Z',
    },
    {
      id: 'task-downloads-cleanup',
      title: 'Organize Downloads by file type',
      prompt: 'Organize Downloads by file type.',
      workspace: 'Files',
      status: 'blocked',
      mode: 'Craft',
      model: 'MiniMax M2.5',
      provider: 'Auto',
      workDirectory: '/Users/me/Downloads',
      outputFormat: 'Table',
      constraints: 'Ask before moving files.',
      contextReferences: '@Downloads',
      attachments: [],
      skills: ['File Organizer'],
      connectors: [],
      permissionProfile: 'Guarded',
      currentStep: 'Waiting for folder permission',
      riskSummary: ['Needs permission for Downloads'],
      artifacts: [],
      changes: [],
      messages: [
        {
          id: 'msg-downloads-assistant',
          role: 'assistant',
          content: 'I need permission to read Downloads before continuing.',
          createdAt: '2026-06-07T00:03:00.000Z',
        },
      ],
      actions: [
        { id: 'action-seed-grant', label: 'Grant Folder Access', kind: 'permission', approvalRequired: true },
      ],
      pinned: false,
      createdAt: '2026-06-07T00:02:00.000Z',
      updatedAt: '2026-06-07T00:03:00.000Z',
    },
  ];
  remotes = [];
  automations = [];
  memories = [
    {
      id: 'memory-concise-citations',
      scope: 'global',
      content: 'Prefer concise technical summaries with citations.',
      source: 'seed',
      editable: true,
    },
    {
      id: 'memory-approval',
      scope: 'workspace',
      content: 'Ask before sending messages or modifying original files.',
      source: 'seed',
      editable: true,
    },
  ];
  skills = [
    { id: 'skill-web-research', name: 'Web Research', category: 'Research', status: 'installed' },
    { id: 'skill-document-writer', name: 'Document Writer', category: 'Artifacts', status: 'installed' },
    { id: 'skill-chart-builder', name: 'Chart Builder', category: 'Data', status: 'installed' },
    { id: 'skill-slash-command-runner', name: 'Slash Command Runner', category: 'Commands', status: 'installed' },
    { id: 'skill-expert-ranking', name: 'Expert Ranking', category: 'Expert Center', status: 'available' },
    { id: 'skill-custom-expert-builder', name: 'Custom Expert Builder', category: 'Expert Center', status: 'available' },
  ];
  connectors = [
    { id: 'connector-google-drive', name: 'Google Drive', kind: 'files', status: 'connected' },
    { id: 'connector-slack', name: 'Slack', kind: 'remote', status: 'available' },
    { id: 'connector-mcp', name: 'MCP Endpoint', kind: 'tools', status: 'available', features: [...assistantCapabilities.mcpFeatures] },
    { id: 'connector-tencent-docs', name: 'Tencent Docs', kind: 'office', status: 'available' },
    { id: 'connector-tencent-meeting', name: 'Tencent Meeting', kind: 'office', status: 'available' },
    { id: 'connector-wecom-docs', name: 'WeCom Docs', kind: 'office', status: 'available' },
    { id: 'connector-qq-mail', name: 'QQ Mail', kind: 'office', status: 'available' },
  ];
  sharedFiles = [
    { id: 'shared-weekly-brief', filename: 'weekly-brief.md', workspace: 'Personal OS', access: 'shared' },
    { id: 'shared-chart', filename: 'assistant-chart.png', workspace: 'Personal OS', access: 'shared' },
  ];
  unshareQueue = [];
  authorizedFolders = ['/workspace/assistant', '/workspace/reports'];
  models = [
    {
      id: 'model-auto',
      provider: 'Auto',
      modelId: 'auto',
      endpoint: 'jarvis://auto',
      enabled: true,
      headers: {},
      parameters: {},
      skipChatCompletions: false,
    },
    {
      id: 'model-workbuddy',
      provider: 'WorkBuddy',
      modelId: 'workbuddy-hunyuan',
      endpoint: 'jarvis://workbuddy',
      enabled: true,
      headers: {},
      parameters: { temperature: 0.3 },
      skipChatCompletions: false,
    },
    {
      id: 'model-minimax-m25',
      provider: 'MiniMax M2.5',
      modelId: 'minimax-m2.5',
      endpoint: 'https://api.minimax.example/v1',
      enabled: true,
      headers: {},
      parameters: { reasoningEffort: 'medium' },
      skipChatCompletions: false,
    },
    {
      id: 'model-glm46',
      provider: 'GLM-4.6',
      modelId: 'glm-4.6',
      endpoint: 'https://api.bigmodel.example/v1',
      enabled: true,
      headers: {},
      parameters: {},
      skipChatCompletions: false,
    },
    {
      id: 'model-kimi-k2',
      provider: 'Kimi K2',
      modelId: 'kimi-k2',
      endpoint: 'https://api.moonshot.example/v1',
      enabled: true,
      headers: {},
      parameters: {},
      skipChatCompletions: false,
    },
    {
      id: 'model-deepseek-v32',
      provider: 'DeepSeek V3.2',
      modelId: 'deepseek-v3.2',
      endpoint: 'https://api.deepseek.example/v1',
      enabled: true,
      headers: {},
      parameters: {},
      skipChatCompletions: false,
    },
    {
      id: 'model-claude-sonnet',
      provider: 'Claude Sonnet',
      modelId: 'claude-sonnet',
      endpoint: 'https://api.anthropic.example/v1',
      enabled: true,
      headers: {},
      parameters: {},
      skipChatCompletions: false,
    },
    {
      id: 'model-gpt-5-codex',
      provider: 'GPT-5-Codex',
      modelId: 'gpt-5-codex',
      endpoint: 'https://api.openai.example/v1',
      enabled: true,
      headers: {},
      parameters: {},
      skipChatCompletions: false,
    },
    {
      id: 'model-local-ollama',
      provider: 'Local Ollama',
      modelId: 'llama3.1',
      endpoint: 'http://localhost:11434',
      enabled: false,
      headers: {},
      parameters: {},
      skipChatCompletions: true,
    },
  ];
  runtimes = [
    { name: 'Node.js', status: 'detected', installAction: 'Use current runtime' },
    { name: 'Python', status: 'needs_setup', installAction: 'Install Python tool runtime' },
  ];
  mcpServers = [
    {
      id: 'mcp-default',
      name: 'MCP Endpoint',
      url: 'mcp://default',
      status: 'needs_trust',
      trusted: false,
      oauth: false,
      headers: { 'User-Agent': 'Jarvis-Assistant' },
      features: [...assistantCapabilities.mcpFeatures],
      tools: [
        { name: 'read_resource', enabled: true },
        { name: 'run_tool', enabled: true },
      ],
    },
  ];
  mcpProgress = [];
  experts = [
    {
      id: 'expert-research-strategist',
      name: 'Research Strategist',
      domain: 'Research',
      description: 'Frames research plans, source quality, and synthesis.',
      ranking: 1,
      visibility: 'public',
    },
    {
      id: 'expert-operations-analyst',
      name: 'Operations Analyst',
      domain: 'Operations',
      description: 'Turns messy process context into operating plans.',
      ranking: 2,
      visibility: 'public',
    },
    {
      id: 'expert-document-editor',
      name: 'Document Editor',
      domain: 'Writing',
      description: 'Improves business documents and executive summaries.',
      ranking: 3,
      visibility: 'internal',
    },
  ];
  workspaces = [
    { id: 'workspace-personal-os', name: 'Personal OS', collapsed: false, pinned: true, archived: false, sortOrder: 0, memoryFile: 'MEMORY.md' },
    { id: 'workspace-files', name: 'Files', collapsed: false, pinned: false, archived: false, sortOrder: 1, memoryFile: 'MEMORY.md' },
    { id: 'workspace-remote-control', name: 'Remote Control', collapsed: false, pinned: false, archived: false, sortOrder: 2, memoryFile: 'MEMORY.md' },
  ];
  deletedWorkspaces = [];
  shares = [];
  uploads = [];
  previews = [
    {
      id: 'preview-weekly-brief',
      taskId: 'task-weekly-brief',
      artifactId: 'artifact-weekly-brief',
      filename: 'weekly-brief.md',
      autoRefresh: true,
      displayMode: 'inline',
      renderedAt: '2026-06-07T00:01:00.000Z',
    },
  ];
  plugins = [
    {
      id: 'plugin-office-suite',
      name: 'Office Suite',
      type: 'suite',
      version: '1.0.0',
      status: 'available',
      securityStatus: 'passed',
      updateAvailable: true,
      loading: false,
      linkedSkillNames: ['Office Suite Writer'],
      linkedMcpNames: ['Office Suite MCP'],
    },
    {
      id: 'plugin-image-generator',
      name: 'Image Generator',
      type: 'skill',
      version: '2.3.0',
      status: 'available',
      securityStatus: 'passed',
      updateAvailable: false,
      loading: false,
      linkedSkillNames: ['Image Generator'],
      linkedMcpNames: [],
    },
    {
      id: 'plugin-data-suite',
      name: 'Data Suite',
      type: 'suite',
      version: '1.4.2',
      status: 'installed',
      securityStatus: 'passed',
      updateAvailable: false,
      loading: false,
      linkedSkillNames: ['Chart Builder'],
      linkedMcpNames: [],
    },
  ];
  pluginVersionCache = { lastSyncedAt: '2026-06-07T00:00:00.000Z', source: 'marketplace' };
  clawChannels = assistantCapabilities.remotePlatforms.map((platform) => ({
    platform,
    status: platform === 'WeChat ClawBot' ? 'connected' : 'available',
    markdownRendering: platform === 'WeChat ClawBot',
    qrCodeUrl: `/assistant/claw/${slug(platform)}/qr`,
    credentialsConfigured: platform === 'WeChat ClawBot',
  }));
  clawGuides = assistantCapabilities.remotePlatforms.map((platform) => ({
    platform,
    steps: [
      `Create or select the ${platform} app.`,
      'Configure bot permissions and callback URL.',
      'Paste credentials into Jarvis and run a connection test.',
    ],
  }));
  clawConfirmations = [];
  approvals = [];
  settings = {
    fontSize: 'medium',
    systemLanguage: 'auto',
    aiGeneratedMarker: true,
    contentFilter: 'friendly_notice',
  };
  supportTickets = [];
  exploreTemplates = [
    {
      id: 'explore-investor-update',
      name: 'Investor Update Agent',
      source: 'community',
      description: 'Turns metrics, notes, and tasks into a weekly investor update.',
      remixable: true,
      useCases: ['research_brief', 'document_generation', 'share_review'],
      skills: ['Web Research', 'Document Writer', 'Chart Builder'],
      connectors: ['Google Drive', 'Slack'],
      outputFormat: 'Document',
      prompt: 'Create an investor update with progress, metrics, risks, and asks.',
    },
    {
      id: 'explore-file-cleanup',
      name: 'Local File Cleanup Agent',
      source: 'official',
      description: 'Plans local file cleanup, conversion, renaming, and PDF merge tasks.',
      remixable: true,
      useCases: ['batch_rename', 'batch_convert', 'merge_pdfs'],
      skills: ['File Organizer', 'Document Writer'],
      connectors: [],
      outputFormat: 'ZIP',
      prompt: 'Organize local files safely and preview changes before applying them.',
    },
    {
      id: 'explore-research-deck',
      name: 'Research Deck Agent',
      source: 'community',
      description: 'Builds a cited research deck with charts and source notes.',
      remixable: true,
      useCases: ['web_research', 'slides', 'charts'],
      skills: ['Web Research', 'Chart Builder', 'Document Writer'],
      connectors: ['Tencent Docs'],
      outputFormat: 'Presentation',
      prompt: 'Research a market and create an executive deck with charts.',
    },
  ];
  exploreRemixes = [];
  cloudSessions = [
    {
      id: 'cloud-weekly-brief',
      taskId: 'task-weekly-brief',
      workspace: 'Personal OS',
      mode: 'Cloud Agent',
      model: 'WorkBuddy',
      status: 'running',
      background: true,
      files: ['workspace-notes.md'],
      startedAt: '2026-06-07T00:05:00.000Z',
      updatedAt: '2026-06-07T00:05:00.000Z',
    },
  ];
}

resetAssistantStore();
