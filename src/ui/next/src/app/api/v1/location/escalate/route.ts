export const POST: (request: Request) => Promise<Response> = async () =>
  Response.json(
    { error: "location escalation is not implemented" },
    { status: 501 },
  );
