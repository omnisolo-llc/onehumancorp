export async function POST(_request: Request): Promise<Response> {
  return Response.json(
    { error: "location escalation is not implemented" },
    { status: 501 },
  );
}
