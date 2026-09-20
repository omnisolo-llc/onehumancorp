export const POST: (request: Request) => Promise<Response> = async () =>
  Response.json(
    { error: "loyalty generation is not implemented" },
    { status: 501 },
  );
