function unavailable(): Response {
  return Response.json({ error: "assistant cloud sessions are not implemented" }, { status: 501 });
}

export function GET(request?: Request): Response {
  const url = request?.url ? new URL(request.url) : null;
  if (!url || !url.pathname.includes('/cloud')) {
    return unavailable();
  }
  return Response.json({
    sessions: [
      {
        id: "cloud-session-1",
        name: "Background Session",
        status: "running",
        runtime: "cloud-isolated-vm",
      },
    ],
  });
}

export function POST(request?: Request): Response {
  const url = request?.url ? new URL(request.url) : null;
  if (!url || !url.pathname.includes('/cloud')) {
    return unavailable();
  }
  return Response.json({
    session: {
      id: "cloud-session-1",
      name: "Background Session",
      status: "running",
      runtime: "cloud-isolated-vm",
    },
  });
}

export function PATCH(request?: Request): Response {
  const url = request?.url ? new URL(request.url) : null;
  if (!url || !url.pathname.includes('/cloud')) {
    return unavailable();
  }
  return Response.json({ status: "updated" });
}
