import React from 'react';

export default function ReleaseNotesPage() {
  const releases = [
    {
      version: "v2.1.0",
      date: "October 24, 2023",
      title: "The All-New Help Center",
      badge: "Major Update",
      content: "We've completely redesigned how you get help in OneHumanCorp. Now you can search articles, watch short videos, and chat with an AI Support Agent directly from anywhere in the app. No more digging through confusing manuals!",
      image: "https://placehold.co/600x300/e2e8f0/475569?text=New+Help+Center"
    },
    {
      version: "v2.0.5",
      date: "October 10, 2023",
      title: "Faster Payment Processing",
      badge: "Improvement",
      content: "We've upgraded our payment engine. When a customer pays you, the money will now appear in your account 24 hours faster on average. We also made the payment emails look much nicer on mobile phones.",
      image: null
    },
    {
      version: "v2.0.0",
      date: "September 1, 2023",
      title: "Launch of AI Agents",
      badge: "New Feature",
      content: "Say hello to your new digital employees. You can now activate an AI Support Agent that learns everything about your business and answers customer questions for you 24/7.",
      image: "https://placehold.co/600x300/e2e8f0/475569?text=AI+Agents"
    }
  ];

  return (
    <div className="min-h-screen bg-slate-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-3xl mx-auto">
        <div className="mb-10 text-center">
          <h1 className="text-4xl font-extrabold text-slate-900 tracking-tight" style={{ fontFamily: 'Outfit, sans-serif' }}>What's New</h1>
          <p className="mt-4 text-lg text-slate-600">The latest features, fixes, and improvements to help you run your business better.</p>
        </div>

        <div className="space-y-12">
          {releases.map((release, idx) => (
            <div key={idx} className="relative pl-8 sm:pl-32 py-6 group">
              {/* Timeline line */}
              <div className="absolute left-4 sm:left-28 top-0 bottom-0 w-px bg-slate-200 group-last:bottom-auto group-last:h-full"></div>
              {/* Timeline dot */}
              <div className="absolute left-3 sm:left-[6.7rem] top-8 w-3 h-3 bg-blue-500 rounded-full ring-4 ring-slate-50"></div>

              {/* Date (desktop side, mobile top) */}
              <div className="hidden sm:block absolute left-0 top-6 w-24 text-right">
                <span className="text-xs font-semibold text-slate-500 uppercase tracking-wider">{release.date}</span>
                <div className="text-[10px] text-slate-400 mt-1">{release.version}</div>
              </div>

              <div className="bg-white rounded-2xl shadow-sm border border-slate-200 p-6 sm:p-8 hover:shadow-md transition-shadow">
                <div className="sm:hidden mb-4">
                  <span className="text-xs font-semibold text-slate-500 uppercase tracking-wider">{release.date}</span>
                </div>

                <div className="flex items-center gap-3 mb-3">
                  <h2 className="text-2xl font-bold text-slate-900" style={{ fontFamily: 'Outfit, sans-serif' }}>{release.title}</h2>
                  <span className={`px-2.5 py-0.5 rounded-full text-xs font-medium ${release.badge === 'New Feature' || release.badge === 'Major Update' ? 'bg-blue-100 text-blue-800' : 'bg-green-100 text-green-800'}`}>
                    {release.badge}
                  </span>
                </div>

                <p className="text-slate-600 text-sm leading-relaxed mb-6" style={{ fontFamily: 'Inter, sans-serif' }}>
                  {release.content}
                </p>

                {release.image && (
                  <div className="mt-4 rounded-xl overflow-hidden border border-slate-100 bg-slate-50">
                    <img src={release.image} alt={release.title} className="w-full h-auto object-cover opacity-90 hover:opacity-100 transition-opacity" />
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
