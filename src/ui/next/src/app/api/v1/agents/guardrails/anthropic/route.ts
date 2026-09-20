export const POST: (request: Request) => Promise<Response> = async () =>
  Response.json(
    { error: "Anthropic guardrail evaluation is not implemented" },
    { status: 501 },
  );
