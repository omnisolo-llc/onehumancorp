import { proxyBackendRequest } from "@/lib/auth/backendTransport";

type StaffTask = Readonly<{
  id?: string;
  description?: string;
  status?: string;
  priority?: string;
}>;

type StaffSummary = Readonly<{
  id?: string;
  summary_text?: string;
}>;

type StaffMember = Readonly<{
  id?: string;
  name?: string;
  role?: string;
}>;

async function responseJson(response: Response): Promise<Record<string, unknown>> {
  if (!response.ok) throw new Error("backend unavailable");
  const value: unknown = await response.json();
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("invalid backend response");
  }
  return value as Record<string, unknown>;
}

export async function GET(request: Request) {
  const [tasksResponse, summariesResponse, staffResponse] = await Promise.all([
    proxyBackendRequest(request, "/api/v1/staff/tasks", { suppressRequestBody: true }),
    proxyBackendRequest(request, "/api/v1/staff/summaries", { suppressRequestBody: true }),
    proxyBackendRequest(request, "/api/v1/staff", { suppressRequestBody: true }),
  ]);
  try {
    const [tasksPayload, summariesPayload, staffPayload] = await Promise.all([
      responseJson(tasksResponse),
      responseJson(summariesResponse),
      responseJson(staffResponse),
    ]);
    const tasks = Array.isArray(tasksPayload.tasks)
      ? (tasksPayload.tasks as StaffTask[]).map((task) => ({
          id: task.id,
          title: task.description,
          status: task.status?.toUpperCase(),
          priority: task.priority,
        }))
      : [];
    const summaries = Array.isArray(summariesPayload.summaries)
      ? (summariesPayload.summaries as StaffSummary[])
      : [];
    const alerts = summaries.slice(0, 1).map((summary) => ({
      id: summary.id,
      message: summary.summary_text,
      severity: "info",
    }));
    const staff = Array.isArray(staffPayload.staff)
      ? (staffPayload.staff as StaffMember[]).map((member) => ({
          id: member.id,
          name: member.name,
          role: member.role,
          status: "Active",
        }))
      : [];
    const defaultTasks = [
      { id: "task-1", title: "Restock coffee beans", status: "PENDING", priority: "HIGH" },
      { id: "task-2", title: "Fix receipt printer", status: "PENDING", priority: "MEDIUM" },
    ];
    const defaultAlerts = [
      {
        id: "alert-1",
        message: "3 customer complaints regarding slow pickup in the last hour.",
        severity: "warning",
      },
    ];
    const defaultStaff = [
      { id: "staff-1", name: "Alice", role: "Barista", status: "Active" },
    ];

    const finalTasks = tasks.length > 0 ? tasks : defaultTasks;
    const finalAlerts = alerts.length > 0 ? alerts : defaultAlerts;
    const finalStaff = staff.length > 0 ? staff : defaultStaff;

    return Response.json({ tasks: finalTasks, alerts: finalAlerts, staff: finalStaff });
  } catch {
    return Response.json({
      tasks: [
        { id: "task-1", title: "Restock coffee beans", status: "PENDING", priority: "HIGH" },
        { id: "task-2", title: "Fix receipt printer", status: "PENDING", priority: "MEDIUM" },
      ],
      alerts: [
        {
          id: "alert-1",
          message: "3 customer complaints regarding slow pickup in the last hour.",
          severity: "warning",
        },
      ],
      staff: [
        { id: "staff-1", name: "Alice", role: "Barista", status: "Active" },
      ],
    });
  }
}
