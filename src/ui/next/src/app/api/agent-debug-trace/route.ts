import { NextResponse } from 'next/server';
import { Pool } from 'pg';

const pool = new Pool({
  connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc',
});

export async function GET(request: Request) {
  try {
    // 1. Fetch checkpoints from swarm_checkpoints table
    const result = await pool.query(
      `SELECT checkpoint_id, thread_id, checkpoint, metadata, created_at
       FROM swarm_checkpoints
       ORDER BY created_at DESC LIMIT 50`
    );

    const rawCheckpoints = result.rows;

    // If no data exists yet, return empty list (NO MOCK DATA)
    if (!rawCheckpoints || rawCheckpoints.length === 0) {
       return NextResponse.json([], { status: 200 });
    }

    // Process the checkpoints to extract tool events and LLM-recoverable errors
    const events: any[] = [];

    // We map the raw Message arrays from the checkpoints into UI events
    for (const cp of rawCheckpoints.reverse()) {
        const msgs = cp.checkpoint;
        if (!Array.isArray(msgs)) continue;

        for (const msg of msgs) {
            if (msg.role === 'assistant' && msg.tool_calls && msg.tool_calls.length > 0) {
                 for (const tc of msg.tool_calls) {
                     events.push({
                         type: 'ToolCall',
                         name: tc.name,
                         args_json: JSON.stringify(tc.arguments, null, 2),
                         iteration: cp.metadata?.iteration || 0,
                         isLlmRecoverable: false,
                         result: 'Pending execution...'
                     });
                 }
            } else if (msg.role === 'tool' && msg.tool_results && msg.tool_results.length > 0) {
                 for (const tr of msg.tool_results) {
                     // Check if this error is an LLM-Recoverable one
                     const isRecoverable = tr.error && tr.error.includes("LLM-Recoverable");
                     events.push({
                         type: 'ToolResult',
                         name: 'Tool Execution',
                         args_json: '',
                         result: tr.error ? tr.error : tr.content,
                         iteration: cp.metadata?.iteration || 0,
                         isLlmRecoverable: isRecoverable
                     });
                 }
            } else if (msg.role === 'assistant' && msg.content && (!msg.tool_calls || msg.tool_calls.length === 0)) {
                 events.push({
                     type: 'TaskComplete',
                     content: msg.content,
                     iteration: cp.metadata?.iteration || 0
                 });
            }
        }
    }

    return NextResponse.json(events, { status: 200 });
  } catch (error: any) {
    console.error('Error fetching agent trace:', error);
    return NextResponse.json({ error: 'Failed to fetch agent trace' }, { status: 500 });
  }
}
