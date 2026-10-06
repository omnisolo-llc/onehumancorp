export type AssistantTaskStatus = 'running' | 'completed' | 'blocked' | 'failed' | 'planning' | 'pending' | 'archived' | 'queued' | 'cancelled' | 'outcome_unknown';
type AssistantArtifact = {
  id: string;
  type: string;
  filename: string;
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
  tool_metadata_json?: { proposed_action?: Record<string, unknown> };
};

export type AssistantTask = {
  id: string;
  title: string;
  workspace: string;
  status: AssistantTaskStatus;
  currentStep: string;
  mode: string;
  model: string;
  provider: string;
  permissionProfile: string;
  riskSummary: string[];
  artifacts: AssistantArtifact[];
  changes: AssistantChange[];
  messages: AssistantMessage[];
  createdAt?: string;
  updatedAt?: string;
  archived?: boolean;
  legacy?: boolean;
  output?: string | null;
  execution?: { id: string; request_id: string; root_request_id?: string; tenant_id?: string; actor_id?: string; phase: string; output: string | null } | null;
  matchedRequestId?: string;
};
