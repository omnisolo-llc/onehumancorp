function unavailable(): Response {
  return Response.json({ error: "assistant uploads are not implemented" }, { status: 501 });
}

export function GET(): Response { return unavailable(); }
export function POST(): Response { return unavailable(); }
