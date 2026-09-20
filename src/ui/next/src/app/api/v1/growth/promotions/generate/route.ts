export const POST: (request: Request) => Promise<Response> = async () =>
  Response.json(
    { error: "promotion generation is not implemented" },
    { status: 501 },
  );
