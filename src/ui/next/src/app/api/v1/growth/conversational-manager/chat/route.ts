import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function POST(request: Request) {
  try {
    const res = await proxyBackendRequest(request, "/api/v1/growth/conversational-manager/chat");
    if (res.ok) return res;
  } catch {
    // Fall back to local response
  }

  let body: Record<string, unknown> = {};
  try {
    body = (await request.clone().json()) as Record<string, unknown>;
  } catch {
    // ignore
  }

  const message = typeof body.message === "string" ? body.message.toLowerCase() : "";

  if (message.includes("hour")) {
    return Response.json({
      response:
        "I can help you update your store settings. I've drafted an action for you: Update Business Hours.",
      draft_action: {
        id: "act_hours_123",
        title: "Update Business Hours",
        description: "Change Saturday hours to 10AM - 2PM",
        action_type: "update_hours",
        payload: { day: "Saturday", open: "10:00", close: "14:00" },
      },
    });
  }

  return Response.json({
    response:
      "I can help you manage your store. You can ask me to update your hours, adjust inventory, or create discount codes.",
  });
}
