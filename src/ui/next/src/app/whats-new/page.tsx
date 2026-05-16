'use client';

export default function WhatsNew() {
  const releases = [
    { version: "1.2.0", date: "Oct 24", notes: "Added AI-Powered Help Chat and Interactive Walkthroughs to make getting started even easier!" },
    { version: "1.1.0", date: "Sep 15", notes: "New Video Tutorials section added." }
  ];

  return (
    <div className="min-h-screen bg-gray-50 p-6 font-inter">
      <div className="max-w-2xl mx-auto">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-8">What's New</h1>

        <div className="space-y-8">
          {releases.map(r => (
            <div key={r.version} className="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
              <div className="flex items-center gap-3 mb-3">
                <span className="bg-blue-100 text-blue-800 text-xs font-bold px-2 py-1 rounded">v{r.version}</span>
                <span className="text-sm text-gray-500">{r.date}</span>
              </div>
              <p className="text-gray-800">{r.notes}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
