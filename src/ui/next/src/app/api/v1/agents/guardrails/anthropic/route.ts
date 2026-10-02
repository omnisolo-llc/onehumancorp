export const POST = async (request: Request): Promise<Response> => {
  try {
    const body = await request.json();
    const toolName = typeof body.toolName === "string" ? body.toolName.trim() : "";
    const projectTrusted = Boolean(body.projectTrusted);
    const sessionAllowedTools: string[] = Array.isArray(body.sessionAllowedTools)
      ? body.sessionAllowedTools
      : [];
    const highRiskTools: string[] = Array.isArray(body.highRiskTools)
      ? body.highRiskTools
      : [];
    const confirmed = Boolean(body.confirmed);

    // Read-only tools
    const isReadOnly =
      toolName.startsWith("read_") ||
      toolName.startsWith("list_") ||
      toolName.startsWith("get_") ||
      toolName.startsWith("view_") ||
      toolName.startsWith("search_") ||
      toolName === "echo";

    // Stage 1: Project Trust Check
    // If project is untrusted, block mutating tools
    if (!projectTrusted && !isReadOnly) {
      return Response.json(
        {
          error: `Stage 1 Violation: Untrusted project blocks mutating tool '${toolName}'. Only read-only tools are permitted.`,
        },
        { status: 400 },
      );
    }

    // Stage 2: Session Permission Check
    // Tool must be explicitly in sessionAllowedTools
    if (!sessionAllowedTools.includes(toolName)) {
      return Response.json(
        {
          error: `Stage 2 Violation: Tool '${toolName}' is not permitted in the current session allowed tools.`,
        },
        { status: 400 },
      );
    }

    // Stage 3: High-Risk User Confirmation
    // If tool is in highRiskTools, user must explicitly confirm
    if (highRiskTools.includes(toolName) && !confirmed) {
      return Response.json(
        {
          error: `Stage 3 Violation: Tool '${toolName}' is classified as high-risk and requires explicit confirmation.`,
        },
        { status: 400 },
      );
    }

    return Response.json({
      result: "Validation passed successfully",
    });
  } catch {
    return Response.json(
      { error: "Invalid request payload or guardrail service unavailable" },
      { status: 501 },
    );
  }
};
