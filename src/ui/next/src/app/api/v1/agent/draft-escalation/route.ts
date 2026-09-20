export const POST: (request: Request) => Promise<Response> = async () =>
  Response.json(
    { error: "agent escalation drafting is not implemented" },
    { status: 501 },
  );
