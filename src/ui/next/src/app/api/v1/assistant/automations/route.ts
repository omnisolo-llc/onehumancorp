function unavailable(): Response {
  return Response.json({ error: "assistant automations are not implemented" }, { status: 501 });
}

export function GET(): Response { return unavailable(); }
export function POST(): Response { return unavailable(); }
export function PATCH(): Response { return unavailable(); }
