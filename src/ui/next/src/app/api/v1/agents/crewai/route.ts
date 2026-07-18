export async function POST(_request: Request): Promise<Response> {
  return Response.json(
    { error: "CrewAI execution is not implemented" },
    { status: 501 },
  );
}
