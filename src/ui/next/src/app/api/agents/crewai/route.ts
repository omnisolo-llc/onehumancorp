import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { task_description } = body;

    // Simulate backend call to the Rust CrewAI module
    // In a real implementation this would make a gRPC or HTTP call to the ohc_builtin_agent



    return NextResponse.json({
      status: 'success',
      report: `[CrewAI Flow Executed]\n\nTask: ${task_description}\n\nResearcher Output: Analysis complete.\nWriter Output: Final JSON Report generated successfully.`,
      execution_time_ms: 1500,
    });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to execute CrewAI workflow' }, { status: 500 });
  }
}
