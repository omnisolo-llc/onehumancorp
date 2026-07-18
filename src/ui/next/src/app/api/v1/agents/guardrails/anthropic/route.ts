export async function POST(_request: Request): Promise<Response> {
  return Response.json(
    { error: "Anthropic guardrail evaluation is not implemented" },
    { status: 501 },
  );
}
