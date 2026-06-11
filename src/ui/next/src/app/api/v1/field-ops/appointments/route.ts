import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  // In a real implementation, this would query the DB for appointments
  // filtered by tenant_id, staff_profile_id, and date.
  // For now, returning mocked structured data mapping to the schema.

  const mockAppointments = [
    {
      id: 'appt-1',
      customer_id: 'cust-1',
      customer_name: 'Alice Smith',
      job_template_id: 'job-plumbing',
      job_name: 'Plumbing Repair',
      status: 'Scheduled',
      scheduled_start_time: new Date(Date.now() + 3600000).toISOString(), // 1 hour from now
      scheduled_end_time: new Date(Date.now() + 7200000).toISOString(),   // 2 hours from now
      location_address: '123 Main St',
      notes: ''
    },
    {
      id: 'appt-2',
      customer_id: 'cust-2',
      customer_name: 'Bob Jones',
      job_template_id: 'job-elec',
      job_name: 'Electrical Inspection',
      status: 'Requested',
      scheduled_start_time: new Date(Date.now() + 10800000).toISOString(), // 3 hours from now
      scheduled_end_time: new Date(Date.now() + 14400000).toISOString(),   // 4 hours from now
      location_address: '456 Oak Ave',
      notes: ''
    }
  ];

  return NextResponse.json({ appointments: mockAppointments });
}

export async function POST(request: Request) {
  // Handles state transitions (e.g., Scheduled -> En-Route -> In-Progress -> Completed)
  try {
    const body = await request.json();
    const { id, status, notes } = body;

    if (!id || !status) {
      return NextResponse.json({ error: 'Missing id or status' }, { status: 400 });
    }

    // Process state transition here (update DB record)
    // If status is "Completed", we might trigger Agent route optimization
    // if the actual_end_time is earlier than scheduled_end_time.

    return NextResponse.json({ success: true, id, status, notes });
  } catch (error) {
    return NextResponse.json({ error: 'Invalid request' }, { status: 400 });
  }
}
