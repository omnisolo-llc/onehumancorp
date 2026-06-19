import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  // This endpoint simulates the Operations Agent recalculating the schedule
  // when a field operator reports they are running late.
  // It receives the current list of appointments, the delayed jobId, and delayMinutes.

  try {
    const body = await request.json();
    const { jobId, appointments, delayMinutes } = body;

    if (!jobId || !appointments || !Array.isArray(appointments) || typeof delayMinutes !== 'number') {
      return NextResponse.json({ error: 'Missing or invalid data' }, { status: 400 });
    }

    const jobIndex = appointments.findIndex(j => j.id === jobId);

    if (jobIndex === -1) {
      return NextResponse.json({ error: 'Job not found in provided list' }, { status: 404 });
    }

    // Identify subsequent jobs
    const subsequentJobs = appointments.slice(jobIndex + 1);

    if (subsequentJobs.length === 0) {
       return NextResponse.json({
           success: true,
           updatedRoute: appointments,
           notifiedCount: 0
       });
    }

    // Simulate Operations Agent logic:
    // 1. Calculate new estimated start times for subsequent jobs
    // 2. Draft and send SMS notifications to affected clients via Twilio/WhatsApp

    const updatedAppointments = appointments.map((job, index) => {
        if (index > jobIndex) {
            // Shift the start and end times by the delay amount
            const start = new Date(job.scheduled_start_time);
            const end = new Date(job.scheduled_end_time);

            start.setMinutes(start.getMinutes() + delayMinutes);
            end.setMinutes(end.getMinutes() + delayMinutes);

            return {
                ...job,
                scheduled_start_time: start.toISOString(),
                scheduled_end_time: end.toISOString()
            };
        }
        return job;
    });

    return NextResponse.json({
        success: true,
        updatedRoute: updatedAppointments,
        notifiedCount: subsequentJobs.length
    });
  } catch (error) {
    return NextResponse.json({ error: 'Invalid request' }, { status: 400 });
  }
}
