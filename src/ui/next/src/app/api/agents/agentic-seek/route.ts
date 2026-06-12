import { NextRequest, NextResponse } from 'next/server';

export async function POST(req: NextRequest) {
  try {
    const { task } = await req.json();

    if (!task) {
      return NextResponse.json({ error: 'Task is required' }, { status: 400 });
    }

    // Call the underlying Rust microservice API for AgenticSeek execution
    const rustBackendUrl = process.env.AGENT_BACKEND_URL || 'http://127.0.0.1:50051';

    // As AgenticSeek is fully local, we use a specialized endpoint if available,
    // or the general completion endpoint with the specific provider.
    const res = await fetch(`${rustBackendUrl}/v1/completions`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        // Instruct backend to use AgenticSeek provider
        'x-ohc-provider': 'agenticseek',
      },
      body: JSON.stringify({
        messages: [{ role: 'user', content: task }],
        model: 'llama3', // Typical default for local agenticseek
      }),
    });

    if (!res.ok) {
      const errText = await res.text();
      return NextResponse.json(
        { error: `Backend error: ${errText || res.statusText}` },
        { status: res.status }
      );
    }

    const data = await res.json();
    return NextResponse.json({ result: data.choices?.[0]?.message?.content || 'Task completed locally.' });

  } catch (error: any) {
    console.error('AgenticSeek Execution Error:', error);
    return NextResponse.json(
      { error: 'Internal Server Error', details: error.message },
      { status: 500 }
    );
  }
}
