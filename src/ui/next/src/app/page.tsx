import React from 'react';

export default function HomePage() {
  return (
    <div className="p-8 max-w-7xl mx-auto space-y-8">
      <header className="mb-10">
        <h2 className="text-3xl font-bold text-slate-900" style={{ fontFamily: 'Outfit, sans-serif' }}>Welcome back, Jane Storeowner</h2>
        <p className="text-slate-500 mt-2">Here is what is happening with your business today.</p>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white p-6 rounded-2xl border border-slate-200 shadow-sm hover:shadow-md transition-shadow">
          <div className="text-sm font-medium text-slate-500 mb-1">Today's Sales</div>
          <div className="text-3xl font-bold text-slate-900">$1,240.00</div>
          <div className="text-xs text-green-600 font-medium mt-2 flex items-center">
            <svg className="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 10l7-7m0 0l7 7m-7-7v18"></path></svg>
            12% from yesterday
          </div>
        </div>
        <div className="bg-white p-6 rounded-2xl border border-slate-200 shadow-sm hover:shadow-md transition-shadow">
          <div className="text-sm font-medium text-slate-500 mb-1">AI Support Tickets Handled</div>
          <div className="text-3xl font-bold text-slate-900">42</div>
          <div className="text-xs text-slate-500 mt-2">Saved you ~3 hours today</div>
        </div>
        <div className="bg-white p-6 rounded-2xl border border-slate-200 shadow-sm hover:shadow-md transition-shadow bg-gradient-to-br from-indigo-50 to-blue-50">
          <div className="text-sm font-bold text-indigo-900 mb-2">Need help?</div>
          <p className="text-sm text-indigo-700 mb-4">Click the floating ? button in the bottom right corner to access video tutorials, plain-language guides, or chat with your AI assistant.</p>
          <a href="/help/release-notes" className="text-xs font-semibold text-indigo-600 hover:underline uppercase tracking-wide">See what's new →</a>
        </div>
      </div>

      <div className="bg-white rounded-2xl border border-slate-200 shadow-sm overflow-hidden mt-8">
        <div className="px-6 py-4 border-b border-slate-200 bg-slate-50">
          <h3 className="font-bold text-slate-800">Recent Transactions</h3>
        </div>
        <div className="divide-y divide-slate-100">
          {[1,2,3].map(i => (
            <div key={i} className="px-6 py-4 flex items-center justify-between hover:bg-slate-50 transition-colors cursor-pointer">
              <div className="flex items-center gap-4">
                <div className="w-10 h-10 rounded-full bg-slate-100 flex items-center justify-center text-slate-500 font-medium">#{i}</div>
                <div>
                  <div className="text-sm font-medium text-slate-900">Customer {i}</div>
                  <div className="text-xs text-slate-500">Just now • Credit Card</div>
                </div>
              </div>
              <div className="font-semibold text-slate-900">+$120.00</div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
