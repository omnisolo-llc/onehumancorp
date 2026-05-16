'use client';
import { useState, useEffect } from 'react';
import ReactMarkdown from 'react-markdown';
import Link from 'next/link';

export default function HelpCenter() {
  const [articles, setArticles] = useState([]);
  const [chatOpen, setChatOpen] = useState(false);
  const [messages, setMessages] = useState<{role: string, content: string}[]>([
    { role: 'agent', content: 'Hi there! I am your AI Help Agent. Ask me anything about OHC!' }
  ]);
  const [inputText, setInputText] = useState("");

  useEffect(() => {
    fetch('/api/v1/docs/help-center')
      .then(r => r.json())
      .then(d => {
        if (d.status === 'ok') setArticles(d.articles || []);
      })
      .catch(() => {});
  }, []);

  const handleSendMessage = (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputText.trim()) return;

    // Add user message
    const newMessages = [...messages, { role: 'user', content: inputText }];
    setMessages(newMessages);
    setInputText("");

    // Simulate AI response with context
    setTimeout(() => {
        setMessages([...newMessages, { role: 'agent', content: "That's a great question! Based on our help center, here is a relevant article you might find helpful. [Read the full article →](/help-center/article/1)"}]);
    }, 1000);
  };

  return (
    <div className="min-h-screen bg-gray-50 text-gray-900 font-inter p-6 relative">
      <div className="max-w-4xl mx-auto">
        <header className="mb-8">
          <h1 className="text-3xl font-bold font-outfit text-gray-900">One Human Corp Help Center</h1>
          <p className="mt-2 text-gray-600">Everything you need to know about managing your business.</p>
          <div className="mt-4 flex gap-4">
             <Link href="/help-center/videos" className="text-blue-600 hover:underline">Watch Video Tutorials</Link>
             <Link href="/whats-new" className="text-blue-600 hover:underline">Release Notes</Link>
             <Link href="/api-reference" className="text-blue-600 hover:underline">API Docs</Link>
          </div>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {articles.map((a: any) => (
            <div key={a.id} className="bg-white rounded-lg p-6 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
              <span className="text-xs font-semibold text-blue-600 uppercase tracking-wider">{a.topic}</span>
              <h2 className="text-xl font-bold mt-2 mb-3">{a.title}</h2>
              <div className="text-gray-600 text-sm leading-relaxed prose prose-sm max-w-none">
                <ReactMarkdown>{a.content_markdown || ''}</ReactMarkdown>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* AI Help Chat Window */}
      {chatOpen && (
        <div className="fixed bottom-24 right-6 w-80 bg-white/90 backdrop-blur-2xl saturate-200 border border-gray-200 rounded-2xl shadow-2xl flex flex-col overflow-hidden z-50">
          <div className="bg-blue-600 p-4 text-white flex justify-between items-center">
            <h3 className="font-bold">Ask anything</h3>
            <button onClick={() => setChatOpen(false)} className="text-white/80 hover:text-white">&times;</button>
          </div>
          <div className="p-4 h-64 overflow-y-auto flex flex-col gap-3">
             {messages.map((m, i) => (
               <div key={i} className={`p-2 rounded-lg text-sm max-w-[85%] ${m.role === 'agent' ? 'bg-gray-100 text-gray-800 self-start' : 'bg-blue-100 text-blue-900 self-end'}`}>
                 <ReactMarkdown>{m.content}</ReactMarkdown>
               </div>
             ))}
          </div>
          <form onSubmit={handleSendMessage} className="p-3 border-t border-gray-100 bg-white flex gap-2">
            <input
              type="text"
              value={inputText}
              onChange={(e) => setInputText(e.target.value)}
              placeholder="Type a question..."
              className="w-full bg-gray-50 border border-gray-200 rounded-full px-4 py-2 text-sm focus:outline-none focus:border-blue-500"
            />
          </form>
        </div>
      )}

      {/* Chat floating button */}
      <button
        onClick={() => setChatOpen(!chatOpen)}
        className="fixed bottom-6 right-6 w-14 h-14 bg-blue-600 rounded-full shadow-xl flex items-center justify-center text-white hover:bg-blue-700 transition-colors z-50"
        aria-label="Ask anything"
      >
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
          <path strokeLinecap="round" strokeLinejoin="round" d="M2.25 12.76c0 1.6 1.123 2.994 2.707 3.227 1.087.16 2.185.283 3.293.369V21l4.076-4.076a1.526 1.526 0 0 1 1.037-.443 48.282 48.282 0 0 0 5.68-.494c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0 0 12 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018Z" />
        </svg>
      </button>
    </div>
  );
}
