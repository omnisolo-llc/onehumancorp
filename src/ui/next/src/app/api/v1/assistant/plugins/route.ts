function unavailable(): Response {
  return Response.json({ error: "assistant plugins are not implemented" }, { status: 501 });
}

export function GET(): Response { return unavailable(); }
export function PATCH(): Response { return unavailable(); }
