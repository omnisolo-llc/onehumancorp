export const POST: (request: Request) => Promise<Response> = async () =>
  Response.json(
    { error: "social-proof generation is not implemented" },
    { status: 501 },
  );
