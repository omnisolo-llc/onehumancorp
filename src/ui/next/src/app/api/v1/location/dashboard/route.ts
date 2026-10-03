import { proxyBackendRequest } from "@/lib/auth/backendTransport";

function record(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("invalid backend data");
  return value as Record<string, unknown>;
}
function text(value: unknown): string {
  if (typeof value !== "string" || value.length > 200_000) throw new Error("invalid backend text");
  return value;
}
async function rows(response: Response, key: string): Promise<Record<string, unknown>[]> {
  const payload = record(await response.json());
  if (payload.error != null || payload.success !== undefined && payload.success !== true
      || !Array.isArray(payload[key]) || payload[key].length > 1000) throw new Error("invalid backend rows");
  return payload[key].map(record);
}

export async function GET(request: Request): Promise<Response> {
  try {
    const responses = await Promise.all([
      proxyBackendRequest(request, "/api/v1/staff/tasks", { suppressRequestBody: true }),
      proxyBackendRequest(request, "/api/v1/staff/summaries", { suppressRequestBody: true }),
      proxyBackendRequest(request, "/api/v1/staff", { suppressRequestBody: true }),
    ]);
    // Preserve actual authentication and backend failures; an unavailable read
    // does not establish that a location is empty or has sample business data.
    const failure = responses.find(response => response.status === 401 || response.status === 403)
      ?? responses.find(response => !response.ok);
    if (failure) return failure;
    const [tasks, summaries, staff] = await Promise.all([
      rows(responses[0], "tasks"), rows(responses[1], "summaries"), rows(responses[2], "staff"),
    ]);
    return Response.json({
      tasks: tasks.map(task => ({ id: text(task.id), title: text(task.title === undefined || task.title === "" ? task.description : task.title), status: text(task.status).toUpperCase(),
        ...(task.priority === undefined ? {} : { priority: text(task.priority) }) })),
      alerts: summaries.map(summary => ({ id: text(summary.id), message: text(summary.summary_text), severity: "info" })),
      // The staff API has no shift/attendance field. Do not invent "Active".
      staff: staff.map(member => ({ id: text(member.id), name: text(member.name), role: text(member.role) })),
    }, { headers: { "cache-control": "private, no-store" } });
  } catch {
    return Response.json(
      { error: "Location information is unavailable" },
      { status: 502, headers: { "cache-control": "private, no-store" } },
    );
  }
}
