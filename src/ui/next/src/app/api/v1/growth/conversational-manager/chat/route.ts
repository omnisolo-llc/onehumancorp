import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function POST(request: Request) {
  const bodyText = await request.text();
  let body: Record<string, unknown> = {};
  try {
    body = JSON.parse(bodyText) as Record<string, unknown>;
  } catch {
    // ignore
  }

  try {
    const proxyReq = new Request(request.url, {
      method: request.method,
      headers: request.headers,
      body: bodyText,
      signal: request.signal,
    });
    const res = await proxyBackendRequest(proxyReq, "/api/v1/growth/conversational-manager/chat");
    if (res.ok) return res;
  } catch {
    // Fall back to local response
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
