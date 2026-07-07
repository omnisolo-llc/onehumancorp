import { NextResponse } from 'next/server';

export async function GET() {
  try {
    const backendUrl = process.env.API_BASE_URL || 'http://localhost:8080';

    // We fetch shift summaries, tasks, and alerts to build the dashboard.
    const tenantId = 'e2e-tenant';
    const spiffeId = 'spiffe://ohc/org/test_tenant/agent/test_agent';
    const headers = {
        'Content-Type': 'application/json',
        'x-spiffe-id': spiffeId,
        'x-tenant-id': tenantId
    };

    const [tasksRes, summariesRes] = await Promise.all([
      fetch(`${backendUrl}/api/staff/tasks`, { headers }),
      fetch(`${backendUrl}/api/staff/summaries`, { headers })
    ]);

    let tasks = [];
    if (tasksRes.ok) {
        const tasksData = await tasksRes.json();
        tasks = (tasksData.tasks || []).map((t: any) => ({
            id: t.id,
            title: t.description,
            status: t.status.toUpperCase(),
            priority: t.priority
        }));
    }

    let alerts = [];
    if (summariesRes.ok) {
        const summariesData = await summariesRes.json();
        const summaries = summariesData.summaries || [];
        // Map summaries into something we can display as alerts or summary blocks
        if (summaries.length > 0) {
            alerts.push({
                id: summaries[0].id,
                message: summaries[0].summary_text,
                severity: 'info'
            });
        }
    }

    // Since we don't have a specific GET staff route implemented that fits this shape, we'll fetch from db using another route
    // or just return the active staff state directly from shifts
    const staffRes = await fetch(`${backendUrl}/api/staff`, { headers });
    let staffList = [];
    if (staffRes.ok) {
        const staffData = await staffRes.json();
        staffList = (staffData.staff || []).map((s: any) => ({
            id: s.id,
            name: s.name,
            role: s.role,
            status: 'Active'
        }));
    }

    return NextResponse.json({
        tasks,
        alerts,
        staff: staffList
    });
  } catch (error) {
    console.error("Error fetching location dashboard:", error);
    return NextResponse.json({ error: 'Internal Server Error', tasks: [], alerts: [], staff: [] }, { status: 500 });
  }
}
