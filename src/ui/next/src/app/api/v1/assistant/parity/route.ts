export function GET(): Response {
  return Response.json({ error: "assistant parity data is not implemented" }, { status: 501 });
}
