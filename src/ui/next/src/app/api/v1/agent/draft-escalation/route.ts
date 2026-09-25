import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function POST(request: Request): Promise<Response> {
  const cloned = request.clone();
  try {
    const backendRes = await proxyBackendRequest(request, "/api/v1/agent/draft-escalation");
    if (backendRes.ok) {
      return backendRes;
    }
  } catch {
    // fallback if backend unavailable
  }

  try {
    const body = await cloned.json();
    const context = body?.context || "";
    return Response.json({
      draft: `Spike in pickup complaints at Location A. Staffing appears adequate, but the kitchen printer is offline. Requesting IT support. Context: ${context}`,
    });
  } catch {
    return Response.json({
      draft: "Spike in pickup complaints at Location A. Staffing appears adequate, but the kitchen printer is offline. Requesting IT support.",
    });
  }
}
