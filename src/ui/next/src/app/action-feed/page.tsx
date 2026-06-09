'use client';
import { useState } from 'react';

export default function ActionFeed() {
  const [tasks, setTasks] = useState([
    {
      id: 1,
      source: 'Instagram',
      customer: 'Customer A',
      message: 'Hi! Yes, I can do a vegan chocolate cake for Saturday. It will be $60. Shall I send a deposit link?',
    }
  ]);
  const [metrics] = useState({
    delivered: 5,
    collected: 450
  });

  const handleApprove = (id: number) => {
    setTasks(tasks.filter(t => t.id !== id));
  };

  return (
    <div className="max-w-md mx-auto p-4 space-y-4">
      {/* Feed Summary */}
      <div className="bg-white rounded-lg shadow p-4 border border-gray-200">
        <h2 className="text-lg font-bold">Summary: {metrics.delivered} orders delivered today, ${metrics.collected} collected.</h2>
      </div>

      {/* Action Required Card */}
      {tasks.length > 0 && (
        <div className="bg-white rounded-lg shadow p-4 border border-gray-200">
          <h2 className="text-lg font-bold text-red-600 mb-2">Action Required: {tasks.length} Pending Inquiries</h2>

          <div className="border rounded-md p-3 bg-gray-50 mb-3">
            <p className="text-sm font-semibold text-gray-700">{tasks[0].customer} via {tasks[0].source}</p>
            <p className="text-sm text-gray-600 mt-1">"Can you make a vegan chocolate cake for this Saturday?"</p>
          </div>

          <div className="bg-blue-50 border border-blue-100 rounded-md p-3 mb-4">
            <p className="text-xs font-semibold text-blue-800 uppercase mb-1">Suggested Reply:</p>
            <p className="text-sm text-blue-900">{tasks[0].message}</p>
          </div>

          <div className="flex space-x-2">
            <button
              onClick={() => handleApprove(tasks[0].id)}
              className="flex-1 bg-blue-600 text-white py-2 px-4 rounded-md font-medium hover:bg-blue-700 transition-colors"
            >
              Approve & Send
            </button>
            <button className="flex-1 bg-gray-200 text-gray-800 py-2 px-4 rounded-md font-medium hover:bg-gray-300 transition-colors">
              Edit
            </button>
            <button
              onClick={() => handleApprove(tasks[0].id)}
              className="flex-none bg-gray-100 text-gray-600 py-2 px-4 rounded-md hover:bg-gray-200 transition-colors"
            >
              Dismiss
            </button>
          </div>
        </div>
      )}

      {tasks.length === 0 && (
        <div className="bg-white rounded-lg shadow p-8 border border-gray-200 text-center">
          <div className="text-4xl mb-4">🎉</div>
          <h2 className="text-xl font-bold text-gray-800 mb-2">All caught up!</h2>
          <p className="text-gray-600">You have no pending tasks.</p>
        </div>
      )}

      {/* FAB */}
      <button className="fixed bottom-6 right-6 bg-black text-white rounded-full p-4 shadow-lg hover:bg-gray-800 transition-colors flex items-center space-x-2">
        <span className="text-xl">✨</span>
        <span className="font-medium">Ask Assistant</span>
      </button>
    </div>
  );
}
