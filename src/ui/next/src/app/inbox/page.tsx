'use client';
import { useState, useEffect } from 'react';
import Link from 'next/link';

type TicketMessage = {
  id: string;
  sender_type: string;
  content: string;
  ai_confidence?: number;
  created_at: string;
};

type SupportTicket = {
  id: string;
  channel: string;
  customer_id?: string;
  status: string;
  created_at: string;
  messages?: TicketMessage[];
};

export default function InboxPage() {
  const [tickets, setTickets] = useState<SupportTicket[]>([]);
  const [selectedTicket, setSelectedTicket] = useState<SupportTicket | null>(null);
  const [loading, setLoading] = useState(true);
  const tenantId = 'e2e-tenant'; // In a real app, this comes from auth context

  useEffect(() => {
    fetchTickets();
  }, []);

  const fetchTickets = async () => {
    try {
      const res = await fetch(`/api/support_engine/${tenantId}/tickets`);
      if (res.ok) {
        const data = await res.json();
        setTickets(data);
      }
    } catch (err) {
      console.error('Failed to fetch tickets', err);
    } finally {
      setLoading(false);
    }
  };

  const loadTicketDetails = async (ticket: SupportTicket) => {
    try {
      const res = await fetch(`/api/support_engine/${tenantId}/tickets/${ticket.id}`);
      if (res.ok) {
        const messages = await res.json();
        setSelectedTicket({ ...ticket, messages });
      }
    } catch (err) {
      console.error('Failed to fetch ticket messages', err);
    }
  };

  const handleApproveDraft = async (ticketId: string) => {
    try {
      await fetch(`/api/support_engine/${tenantId}/tickets/${ticketId}/status`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ status: 'resolved' }),
      });
      fetchTickets();
      setSelectedTicket(null);
    } catch (err) {
      console.error('Failed to update ticket status', err);
    }
  };

  if (loading) return <div className="p-4">Loading inbox...</div>;

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col md:flex-row max-w-[375px] md:max-w-full mx-auto overflow-hidden">
      {/* Inbox List View */}
      <div className={`w-full md:w-1/3 bg-white border-r flex flex-col ${selectedTicket ? 'hidden md:flex' : 'flex'}`}>
        <div className="p-4 border-b flex justify-between items-center bg-gray-100">
          <h1 className="text-xl font-bold text-gray-800">Support Inbox</h1>
          <Link href="/dashboard" className="text-sm text-blue-600">Back</Link>
        </div>
        <div className="overflow-y-auto flex-1">
          {tickets.length === 0 ? (
            <p className="p-4 text-gray-500 text-center">No open tickets.</p>
          ) : (
            tickets.map(ticket => (
              <div
                key={ticket.id}
                className={`p-4 border-b cursor-pointer hover:bg-gray-50 ${selectedTicket?.id === ticket.id ? 'bg-blue-50' : ''}`}
                onClick={() => loadTicketDetails(ticket)}
              >
                <div className="flex justify-between items-center mb-1">
                  <span className="font-semibold text-gray-800 uppercase text-xs">{ticket.channel}</span>
                  <span className="text-xs text-gray-500">{new Date(ticket.created_at).toLocaleDateString()}</span>
                </div>
                <div className="text-sm text-gray-600 truncate">
                  Ticket ID: {ticket.id.substring(0, 8)}...
                </div>
                {ticket.status === 'draft' && (
                  <span className="inline-block mt-2 px-2 py-1 bg-yellow-100 text-yellow-800 text-xs font-medium rounded">
                    AI Draft Ready
                  </span>
                )}
              </div>
            ))
          )}
        </div>
      </div>

      {/* Draft Review View */}
      <div className={`w-full md:w-2/3 flex flex-col ${!selectedTicket ? 'hidden md:flex' : 'flex'}`}>
        {selectedTicket ? (
          <>
            <div className="p-4 border-b bg-white flex justify-between items-center">
              <button
                className="md:hidden text-blue-600 font-medium"
                onClick={() => setSelectedTicket(null)}
              >
                &larr; Back
              </button>
              <h2 className="font-semibold text-gray-800 uppercase">{selectedTicket.channel} Ticket</h2>
            </div>
            <div className="flex-1 overflow-y-auto p-4 space-y-4 bg-gray-50">
              {selectedTicket.messages?.map((msg) => (
                <div key={msg.id} className={`flex ${msg.sender_type === 'customer' ? 'justify-start' : 'justify-end'}`}>
                  <div className={`max-w-[80%] rounded-lg p-3 ${msg.sender_type === 'customer' ? 'bg-white border' : 'bg-blue-600 text-white'}`}>
                    <p className="text-sm">{msg.content}</p>
                    <span className={`text-[10px] block mt-1 ${msg.sender_type === 'customer' ? 'text-gray-400' : 'text-blue-200'}`}>
                      {new Date(msg.created_at).toLocaleTimeString()}
                      {msg.ai_confidence && ` • AI Confidence: ${msg.ai_confidence}%`}
                    </span>
                  </div>
                </div>
              ))}
            </div>

            {/* Action Bar */}
            <div className="p-4 bg-white border-t space-y-3">
              <div className="flex space-x-2">
                <button
                  className="flex-1 bg-gray-200 text-gray-800 font-semibold py-3 rounded-lg text-sm"
                  onClick={() => setSelectedTicket(null)}
                >
                  Edit Draft
                </button>
                <button
                  className="flex-1 bg-blue-600 text-white font-bold py-3 rounded-lg text-sm shadow-md"
                  onClick={() => handleApproveDraft(selectedTicket.id)}
                >
                  Approve & Send
                </button>
              </div>
            </div>
          </>
        ) : (
          <div className="flex-1 flex items-center justify-center text-gray-400">
            Select a ticket to review
          </div>
        )}
      </div>
    </div>
  );
}
