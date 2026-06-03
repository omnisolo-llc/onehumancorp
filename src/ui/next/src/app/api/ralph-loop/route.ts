import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { task, taskId } = await req.json();

    if (!task) {
      return NextResponse.json({ error: 'Task is required' }, { status: 400 });
    }

    const effectiveTaskId = taskId || `ralph-${Date.now()}`;

    return NextResponse.json({
      status: "STARTED",
      taskId: effectiveTaskId,
      message: "Ralph Mission initialized (Mock)."
    });
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}

export async function GET(req: Request) {
  const { searchParams } = new URL(req.url);
  const taskId = searchParams.get('taskId');

  if (!taskId) {
    return NextResponse.json({ error: 'taskId is required' }, { status: 400 });
  }

  const mockProgress = {
    task_description: "Build a full inventory system",
    features: [
      { name: "Step 1: Database Schema Design", status: "completed" },
      { name: "Step 2: API Endpoint Implementation", status: "in_progress" },
      { name: "Step 3: Frontend Inventory Dashboard", status: "pending" },
      { name: "Step 4: Real-time Stock Alerts", status: "pending" },
    ],
    current_feature_index: 1,
    notes: [
      "Initialized task and broken down into features.",
      "Completed Database Schema: Created tables for products, categories, and stock_logs.",
      "Working on API: Implementing CRUD for products."
    ],
    architectural_decisions: [
      "Decision: Use PostgreSQL row-level security for inventory isolation.",
      "Decision: Use Redis for real-time alert caching."
    ],
    unresolved_bugs: [
      "Bug: Pagination fails on categories with 0 items."
    ],
    session_id: taskId,
    is_complete: false
  };

  return NextResponse.json(mockProgress);
}
