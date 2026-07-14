import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { privateJson } from "../assistantBackend";

export async function GET(request: Request): Promise<Response> {
  const response = await proxyBackendRequest(request, "/api/assistant/workspaces");
  if (!response.ok) return response;
  const value = await response.json().catch(() => null);
  if (!Array.isArray(value)) {
    return privateJson(502, { error: "invalid workspace response" });
  }
  const workspaces = [];
  for (const entry of value) {
    if (
      entry === null ||
      typeof entry !== "object" ||
      Array.isArray(entry) ||
      typeof (entry as Record<string, unknown>).id !== "string" ||
      typeof (entry as Record<string, unknown>).name !== "string"
    ) {
      return privateJson(502, { error: "invalid workspace response" });
    }
    const workspace = entry as Record<string, string>;
    workspaces.push({
      id: workspace.id,
      name: workspace.name,
      collapsed: false,
      pinned: false,
      archived: false,
      sortOrder: 0,
      memoryFile: "MEMORY.md",
    });
  }
  return privateJson(200, { workspaces, deleted: [] });
}

export function PATCH(_request: Request): Response {
  return privateJson(405, { error: "workspace mutation unavailable" });
}
