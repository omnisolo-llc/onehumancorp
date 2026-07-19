export function POST(): Response {
  return Response.json({ error: "assistant support tickets are not implemented" }, { status: 501 });
}
