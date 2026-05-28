'use client';
import { useState } from 'react';
import Link from 'next/link';

export default function MarketingPage() {
  const [subject, setSubject] = useState('');
  const [body, setBody] = useState('');
  const [sent, setSent] = useState(false);

  const handleSend = () => {
    // Simulated Resend integration
    setSent(true);
    setTimeout(() => {
        setSubject('');
        setBody('');
        setSent(false);
    }, 3000);
  };

  return (
    <div className="p-4 max-w-[375px] mx-auto bg-white min-h-screen shadow-xl relative overflow-x-hidden flex flex-col font-inter">
      <div className="flex items-center mb-4 border-b pb-2">
        <Link href="/dashboard" className="mr-4 text-blue-500 hover:text-blue-700">
          &lt; Back
        </Link>
        <h1 className="text-2xl font-bold">Email Broadcast</h1>
      </div>

      <div className="flex flex-col gap-4">
        <div>
          <label className="block text-sm font-semibold mb-1">Subject</label>
          <input
            type="text"
            className="w-full border rounded p-2"
            placeholder="Announcing our new feature!"
            value={subject}
            onChange={e => setSubject(e.target.value)}
          />
        </div>

        <div>
          <label className="block text-sm font-semibold mb-1">Message</label>
          <textarea
            className="w-full border rounded p-2 h-40"
            placeholder="Type your email here..."
            value={body}
            onChange={e => setBody(e.target.value)}
          />
        </div>

        <button
          onClick={handleSend}
          className="bg-[#805ad5] text-white font-bold py-3 rounded-lg mt-4 shadow hover:bg-[#6b46c1] transition-colors"
          disabled={sent || !subject || !body}
        >
          {sent ? 'Sent successfully via Resend!' : 'Send Campaign'}
        </button>
        <p className="text-xs text-gray-500 text-center mt-2">Unsubscribe link will be automatically appended.</p>
      </div>
    </div>
  );
}
