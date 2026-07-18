export async function POST(_request: Request): Promise<Response> {
  return Response.json(
    { error: "agent escalation drafting is not implemented" },
    { status: 501 },
  );
}
