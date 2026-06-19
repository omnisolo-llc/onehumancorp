import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { appointments, delayJobId } = body;

    if (!appointments || !Array.isArray(appointments) || !delayJobId) {
      return NextResponse.json({ error: 'Missing or invalid data' }, { status: 400 });
    }

    const delayedJobIndex = appointments.findIndex((a: any) => a.id === delayJobId);

    if (delayedJobIndex === -1) {
      return NextResponse.json({ error: 'Job not found' }, { status: 404 });
    }

    const DELAY_MINUTES = 30;
    const DELAY_MS = DELAY_MINUTES * 60 * 1000;

    let subsequentCount = 0;
    const optimizedRoute = appointments.map((job: any, index: number) => {
      if (index > delayedJobIndex && !['Completed', 'Cancelled'].includes(job.status)) {
        subsequentCount++;
        const newStart = new Date(new Date(job.scheduled_start_time).getTime() + DELAY_MS).toISOString();
        const newEnd = new Date(new Date(job.scheduled_end_time).getTime() + DELAY_MS).toISOString();
        return {
          ...job,
          scheduled_start_time: newStart,
          scheduled_end_time: newEnd
        };
      }
      return job;
    });

    let agentSuggestion = null;
    if (subsequentCount > 0) {
      agentSuggestion = `Drafting delay notifications for the next ${subsequentCount} clients. Approve?`;
    } else {
      agentSuggestion = "No subsequent appointments to notify.";
    }

    return NextResponse.json({
        success: true,
        optimizedRoute,
        subsequentCount,
        agentSuggestion
    });
  } catch (error) {
    return NextResponse.json({ error: 'Invalid request' }, { status: 400 });
  }
}
