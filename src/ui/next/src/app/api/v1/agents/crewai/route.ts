export const POST: (request: Request) => Promise<Response> = async () =>
  Response.json(
    { error: "CrewAI execution is not implemented" },
    { status: 501 },
  );
