import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { toolName, projectTrusted, sessionAllowedTools, highRiskTools } = await req.json();

    // Stage 1: Trust establishment check
    if (!projectTrusted) {
      return NextResponse.json(
        { error: `Anthropic Guardrail Stage 1 (Trust) tripped: Project is not trusted. Tool '${toolName}' is not in the safe-for-untrusted list.` },
        { status: 403 }
      );
    }

    // Stage 2: Session permission check
    if (sessionAllowedTools && sessionAllowedTools.length > 0 && !sessionAllowedTools.includes(toolName)) {
      return NextResponse.json(
        { error: `Anthropic Guardrail Stage 2 (Permission) tripped: Tool '${toolName}' is not allowed in this session.` },
        { status: 403 }
      );
    }

    // Stage 3: High-risk explicit confirmation requirement
    if (highRiskTools && highRiskTools.includes(toolName)) {
      return NextResponse.json(
        { error: `Anthropic Guardrail Stage 3 (Confirmation) tripped: Tool '${toolName}' is marked as high-risk and requires explicit user confirmation.` },
        { status: 403 }
      );
    }

    return NextResponse.json({ result: "Tool check passed all guardrails." });
  } catch (err: any) {
    return NextResponse.json({ error: err.message }, { status: 500 });
  }
}
