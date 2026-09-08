import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { privateJson } from "../assistantBackend";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

type RecordValue = Record<string, unknown>;

function record(value: unknown): RecordValue | null {
  return value !== null && typeof value === "object" && !Array.isArray(value)
    ? value as RecordValue
    : null;
}

function text(value: unknown, fallback = ""): string {
  return typeof value === "string" ? value : fallback;
}

function workspaceId(value: unknown): string {
  const slug = text(value, "personal-os")
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9._-]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .slice(0, 128);
  return slug || "personal-os";
}

function toBackendTask(body: Uint8Array<ArrayBuffer>): Uint8Array<ArrayBuffer> {
  const input = record(JSON.parse(decoder.decode(body)));
  const prompt = text(input?.prompt).trim();
  if (!input || !prompt || prompt.length > 20_000) throw new Error("invalid task prompt");
  const now = Math.floor(Date.now() / 1000);
  return encoder.encode(JSON.stringify({
    id: crypto.randomUUID(),
    workspace_id: workspaceId(input.workspace),
    title: prompt.split("\n", 1)[0].slice(0, 160),
    prompt,
    status: "planning",
    mode: text(input.mode, "Plan"),
    permission_profile: text(input.permissionProfile, "Guarded"),
    model_config_json: {
      model: text(input.model, "Auto"),
      provider: text(input.provider, "Auto"),
      workDirectory: text(input.workDirectory),
      outputFormat: text(input.outputFormat, "Document"),
      constraints: text(input.constraints),
    },
    current_step: "Task created",
    archived: false,
    created_at_unix: now,
    updated_at_unix: now,
  }));
}

function toUiTask(value: unknown): RecordValue | null {
  const task = record(value);
  if (!task || !text(task.id) || !text(task.title)) return null;
  const config = record(task.model_config_json) ?? {};
  const createdAt = Number(task.created_at_unix);
  const updatedAt = Number(task.updated_at_unix);
  return {
    id: text(task.id),
    title: text(task.title),
    workspace: text(task.workspace_id, "personal-os"),
    status: text(task.status, "pending"),
    currentStep: text(task.current_step),
    mode: text(task.mode, "Plan"),
    model: text(config.model, "Auto"),
    provider: text(config.provider, "Auto"),
    permissionProfile: text(task.permission_profile, "Guarded"),
    riskSummary: [],
    artifacts: [],
    changes: [],
    messages: [],
    createdAt: Number.isFinite(createdAt) && createdAt > 0 ? new Date(createdAt * 1000).toISOString() : undefined,
    updatedAt: Number.isFinite(updatedAt) && updatedAt > 0 ? new Date(updatedAt * 1000).toISOString() : undefined,
  };
}

export async function GET(request: Request): Promise<Response> {
  const response = await proxyBackendRequest(request, "/api/v1/assistant/tasks");
  if (!response.ok) return response;
  const payload = await response.json().catch(() => null);
  if (!Array.isArray(payload)) return privateJson(502, { error: "invalid assistant task list" });
  const tasks = payload.map(toUiTask);
  if (tasks.some((task) => task === null)) return privateJson(502, { error: "invalid assistant task" });
  return privateJson(200, {
    tasks,
    capabilities: {
      outputFormats: ["Document", "Presentation", "PDF", "Code App"],
      workModes: ["Ask", "Agent", "Plan", "Coding"],
      modelProviders: ["Auto", "Agent"],
    },
  });
}

export async function POST(request: Request): Promise<Response> {
  const response = await proxyBackendRequest(request, "/api/v1/assistant/tasks", {
    requestContentType: "application/json",
    transformRequestBody: toBackendTask,
  });
  if (!response.ok) return response;
  const task = toUiTask(await response.json().catch(() => null));
  return task ? privateJson(201, { task }) : privateJson(502, { error: "invalid assistant task" });
}
