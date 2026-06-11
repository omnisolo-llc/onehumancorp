import { NextRequest, NextResponse } from 'next/server';

export const runtime = 'nodejs';

export async function POST(request: NextRequest) {
  try {
    const payload = await request.json();
    console.log("Visual workflow api received:", payload);

    let graphNodes: any[] = [];
    let graphEdges: any[] = [];

    // The UI sends nodes as a map like { id: { id, type, label, next: [] } }
    // but the backend expects a WorkflowGraph { nodes: [{id, node_type}], edges: [{source, target}] }

    if (payload.nodes && typeof payload.nodes === 'object') {
        // Construct the expected structure
        let i = 0;
        let sortedIds = Object.keys(payload.nodes);
        for (const nodeId of sortedIds) {
            const block = payload.nodes[nodeId];

            // Map BlockType to NodeType
            let nodeType: any = {};
            if (i === 0) {
                 nodeType = { "Input": { "name": "input_var" } };
            } else if (i === sortedIds.length - 1) {
                 nodeType = "Output";
            } else {
                 nodeType = { "Llm": { "prompt_template": "Execute block: " + block.label } };
            }

            graphNodes.push({
                id: block.id,
                node_type: nodeType
            });

            for (const nextId of block.next) {
                graphEdges.push({
                    source: block.id,
                    target: nextId
                });
            }
            i++;
        }
    }

    const backendPayload = {
       graph: {
           nodes: graphNodes,
           edges: graphEdges
       },
       inputs: {
           input_var: "Start workflow execution"
       }
    };

    const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:18789';
    const response = await fetch(`${API_BASE}/api/workflow/run`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(backendPayload),
    });

    if (!response.ok) {
      const text = await response.text();
      console.error('Visual workflow api err response from backend:', text);
      throw new Error(`Backend returned ${response.status}: ${text}`);
    }

    const data = await response.json();
    return NextResponse.json(data, { status: 200 });

  } catch (error: any) {
    console.error('Error running workflow:', error);
    return NextResponse.json({ success: false, error: error.message }, { status: 500 });
  }
}
