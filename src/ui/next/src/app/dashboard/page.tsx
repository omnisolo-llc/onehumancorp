"use client";

import { useState, useEffect } from "react";

export default function DashboardPage() {
  const [briefing, setBriefing] = useState<any>(null);
  const [tasks, setTasks] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchData() {
      try {
        const response = await fetch('/dashboard/api');
        const data = await response.json();
        setBriefing(data.dailyBriefing);
        setTasks(data.agentTasks);
      } catch (error) {
        console.error("Failed to fetch dashboard data", error);
      } finally {
        setLoading(false);
      }
    }
    fetchData();
  }, []);

  const handleApprove = (taskId: string) => {
    setTasks(tasks.filter(t => t.id !== taskId));
    // In a real app, this would send an API request to the backend to execute the task
  };

  const handleEdit = (taskId: string) => {
    // In a real app, this would open a modal to edit the drafted content
    console.log("Edit task", taskId);
  };

  if (loading) {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col items-center justify-center border-x border-gray-200">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 mb-6"></div>
          <p className="text-gray-600 font-medium">Loading your AI Dashboard...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
      <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col relative border-x border-gray-200 overflow-hidden">

        {/* Header */}
        <div className="bg-gray-900 text-white p-6 pb-8 pt-10 rounded-b-[2rem] shadow-md z-10 relative">
          <div className="flex justify-between items-center mb-6">
            <h1 className="text-2xl font-bold font-outfit">OHC Dashboard</h1>
            <div className="w-10 h-10 bg-gray-800 rounded-full flex items-center justify-center border border-gray-700">
              <span className="font-bold">C</span>
            </div>
          </div>

          {/* Daily Briefing Card */}
          {briefing && (
            <div className="bg-white/10 backdrop-blur-md border border-white/20 p-5 rounded-2xl relative overflow-hidden">
               <div className="absolute top-0 right-0 w-24 h-24 bg-blue-500/20 rounded-full blur-2xl -mr-10 -mt-10 pointer-events-none"></div>
              <h2 className="text-xl font-bold font-outfit mb-1">{briefing.title}</h2>
              <p className="text-gray-300 text-xs mb-4">{briefing.date}</p>

              <div className="space-y-3 mt-4">
                 <h3 className="text-sm font-semibold text-blue-300 uppercase tracking-wider mb-2">Today's Insights</h3>
                <ul className="space-y-2">
                  {briefing.summary.map((point: string, i: number) => (
                    <li key={i} className="flex items-start">
                      <span className="text-blue-400 mr-2 mt-0.5">•</span>
                      <span className="text-sm text-gray-100 leading-snug">{point}</span>
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          )}
        </div>

        {/* Action Feed */}
        <div className="flex-1 overflow-y-auto px-5 pt-6 pb-8 bg-gray-50 hide-scrollbar -mt-6">
          <div className="flex justify-between items-center mb-4 mt-6">
            <h2 className="text-lg font-bold font-outfit text-gray-900">Agent Activity Feed</h2>
            <span className="bg-blue-100 text-blue-800 text-xs font-bold px-2 py-1 rounded-full">{tasks.length} pending</span>
          </div>

          {tasks.length === 0 ? (
            <div className="text-center py-10">
              <div className="w-16 h-16 bg-green-100 text-green-500 rounded-full flex items-center justify-center mx-auto mb-4">
                 <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
              </div>
              <p className="text-gray-500 font-medium">All caught up! Your agents are standing by.</p>
            </div>
          ) : (
            <div className="space-y-4">
              {tasks.map(task => (
                <div key={task.id} className="bg-white p-5 rounded-2xl shadow-sm border border-gray-100 flex flex-col gap-3">
                  {/* Task Header */}
                  <div className="flex justify-between items-start">
                    <div className="flex items-center gap-2">
                      <div className="w-8 h-8 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center font-bold text-xs">
                        {task.agentName.charAt(4)} {/* Quick hack for initial */}
                      </div>
                      <div>
                        <p className="text-xs font-semibold text-gray-900">{task.agentName}</p>
                        <p className="text-[10px] text-gray-500 uppercase tracking-wider">{task.agentRole}</p>
                      </div>
                    </div>
                    <span className="text-xs font-medium text-gray-500 bg-gray-100 px-2 py-1 rounded-md">{task.actionType}</span>
                  </div>

                  {/* Task Content */}
                  <div>
                    <p className="text-sm font-medium text-gray-800 mb-2">{task.description}</p>
                    <div className="bg-gray-50 p-3 rounded-lg border border-gray-100 text-sm text-gray-600 italic">
                      "{task.previewText}"
                    </div>
                  </div>

                  {/* Task Actions */}
                  <div className="flex gap-2 mt-2">
                    <button
                      onClick={() => handleEdit(task.id)}
                      className="flex-1 py-2.5 bg-white border border-gray-300 text-gray-700 text-sm font-semibold rounded-xl hover:bg-gray-50 transition-colors"
                    >
                      Edit
                    </button>
                    <button
                      onClick={() => handleApprove(task.id)}
                      className="flex-[2] py-2.5 bg-blue-600 text-white text-sm font-bold rounded-xl shadow-sm hover:bg-blue-700 active:scale-[0.98] transition-all flex items-center justify-center gap-2"
                    >
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                      Approve & Send
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Bottom Navigation Bar */}
        <div className="absolute bottom-0 w-full bg-white border-t border-gray-200 py-3 px-6 flex justify-between items-center z-50">
           <button className="flex flex-col items-center text-blue-600">
             <svg className="w-6 h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" /></svg>
             <span className="text-[10px] font-medium">Home</span>
           </button>
           <button className="flex flex-col items-center text-gray-400 hover:text-gray-600">
             <svg className="w-6 h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" /></svg>
             <span className="text-[10px] font-medium">Orders</span>
           </button>
           <button className="flex flex-col items-center text-gray-400 hover:text-gray-600">
             <svg className="w-6 h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" /></svg>
             <span className="text-[10px] font-medium">Inbox</span>
           </button>
           <button className="flex flex-col items-center text-gray-400 hover:text-gray-600">
             <svg className="w-6 h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /></svg>
             <span className="text-[10px] font-medium">Settings</span>
           </button>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
