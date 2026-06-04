import { NextResponse } from 'next/server';

let feed = [
  {
    id: '1',
    department: 'Operations',
    title: 'Operations',
    description: '3 new orders to fulfill.',
    actionLabel: 'Fulfill Now',
    type: 'urgent',
    timestamp: 'Just now'
  },
  {
    id: '2',
    department: 'Advisory',
    title: 'Advisory',
    description: 'It\'s been 30 days since your last promo. Should I draft an email?',
    actionLabel: 'Yes, draft it',
    draftContent: 'Subject: We Miss You! Here\'s 20% Off ☀️\n\nHi [Name],\n\nIt\'s been a while! We wanted to treat you to 20% off your next order.\n\nUse code: SUMMER20 at checkout.\n\nCheers,\nYour Business Name',
    type: 'proposal',
    timestamp: '2h ago'
  },
  {
    id: '3',
    department: 'Marketing',
    title: 'Marketing',
    description: 'Here is your generated Instagram post for the new cake.',
    actionLabel: 'Approve & Post',
    type: 'proposal',
    timestamp: '4h ago'
  },
];

export async function GET() {
  return NextResponse.json({ feed });
}
