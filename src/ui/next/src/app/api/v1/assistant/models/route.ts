function unavailable(): Response {
  return Response.json({ error: "assistant model settings are not implemented" }, { status: 501 });
}

export function GET(): Response { return unavailable(); }
export function PATCH(): Response { return unavailable(); }
