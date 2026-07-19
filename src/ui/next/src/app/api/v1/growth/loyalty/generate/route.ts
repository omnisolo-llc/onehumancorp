export async function POST(_request: Request): Promise<Response> {
  return Response.json(
    { error: "loyalty generation is not implemented" },
    { status: 501 },
  );
}
