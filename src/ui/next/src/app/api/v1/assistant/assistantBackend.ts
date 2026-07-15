const TASK_ID = /^[A-Za-z0-9._-]{1,128}$/;
const decoder = new TextDecoder("utf-8", { fatal: true });

function validTaskId(value: unknown): string {
  if (typeof value !== "string" || !TASK_ID.test(value)) {
    throw new Error("invalid task ID");
  }
  return value;
}

export function taskBackendPath(id: unknown, suffix = ""): string {
  return `/api/v1/assistant/tasks/${validTaskId(id)}${suffix}`;
}

export function taskBackendPathFromBody(
  suffix: string,
): (body: Uint8Array<ArrayBuffer>) => string {
  return (body) => {
    const value = JSON.parse(decoder.decode(body));
    if (value === null || typeof value !== "object" || Array.isArray(value)) {
      throw new Error("invalid request");
    }
    return taskBackendPath((value as Record<string, unknown>).taskId, suffix);
  };
}

export async function withSuccessStatus(
  pending: Promise<Response>,
  status: number,
): Promise<Response> {
  const response = await pending;
  if (!response.ok) return response;
  return new Response(response.body, {
    status,
    headers: response.headers,
  });
}

export async function withTaskMutationEnvelope(
  pending: Promise<Response>,
): Promise<Response> {
  const response = await pending;
  if (!response.ok) return response;
  const value = await response.json().catch(() => null);
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    return privateJson(502, { error: "invalid task response" });
  }
  if ("deletedTask" in value) return privateJson(response.status, value);
  return privateJson(response.status, { task: value });
}

export function privateJson(status: number, value: unknown): Response {
  return Response.json(value, {
    status,
    headers: {
      "cache-control": "private, no-store",
      pragma: "no-cache",
      "x-content-type-options": "nosniff",
    },
  });
}
