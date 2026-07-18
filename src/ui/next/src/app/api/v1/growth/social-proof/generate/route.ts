export async function POST(_request: Request): Promise<Response> {
  return Response.json(
    { error: "social-proof generation is not implemented" },
    { status: 501 },
  );
}
