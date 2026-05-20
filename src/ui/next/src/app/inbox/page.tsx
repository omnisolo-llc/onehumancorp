"use client";

import { useState } from "react";
import Link from "next/link";

export default function InboxPage() {
  const [messages] = useState([
    { id: 1, sender: "Instagram User", text: "Is this vegan?", time: "10:30 AM" },
    { id: 2, sender: "Facebook User", text: "When do you open?", time: "11:15 AM" },
    { id: 3, sender: "WhatsApp User", text: "Order status pls", time: "12:05 PM" }
  ]);

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <div className="flex items-center gap-4">
            <Link href="/dashboard" className="text-blue-600 hover:text-blue-700 font-medium">← Back</Link>
            <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F' }}>Customer Inbox</h1>
         </div>
      </header>
      <main className="p-6 max-w-4xl mx-auto w-full">
        <div className="flex flex-col gap-4" id="messages-list">
            {messages.map(msg => (
                <div key={msg.id} className="p-4 bg-white rounded-xl shadow-sm border border-gray-100">
                    <div className="flex justify-between mb-1">
                        <span className="font-bold text-gray-900">{msg.sender}</span>
                        <span className="text-xs text-gray-400">{msg.time}</span>
                    </div>
                    <p className="text-gray-600">{msg.text}</p>
                </div>
            ))}
        </div>
        <div className="mt-8 p-4 bg-white rounded-xl border border-gray-200">
            <input id="reply-input" type="text" placeholder="Type a message..." className="w-full p-2 border rounded-md mb-2" />
            <button className="px-4 py-2 bg-blue-600 text-white rounded-md">Send</button>
        </div>
      </main>
    </div>
  );
}
