import React, { useState, useEffect } from 'react';

type InsightAction = {
  id: string;
  title: string;
  description: string;
  actionLabel: string;
  urgency: 'high' | 'medium' | 'low';
};

export function AssistantInsightsWidget() {
  const [insights, setInsights] = useState<InsightAction[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchInsights() {
      try {
        const res = await fetch('/api/v1/assistant/insights');
        if (res.ok) {
          const data = await res.json();
          if (data && data.insights) { setInsights(data.insights); } else { setInsights([]); }
        }
      } catch (error) {
        console.error("Failed to fetch insights", error);
      } finally {
        setLoading(false);
      }
    }
    fetchInsights();
  }, []);

  const handleAction = (id: string) => {
    // In a real app, this would call an API to execute the action
    setInsights(insights.filter(insight => insight.id !== id));
  };

  if (loading) {
    return (
      <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-2xl p-6 shadow-sm animate-pulse">
        <div className="h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/3 mb-4"></div>
        <div className="space-y-3">
          <div className="h-20 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div className="h-20 bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
      </div>
    );
  }

  if (insights.length === 0) {
    return (
      <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-2xl p-6 shadow-sm flex flex-col items-center justify-center text-center min-h-[200px]">
        <div className="text-4xl mb-2 opacity-50">✨</div>
        <h3 className="text-lg font-semibold font-outfit text-gray-900 dark:text-gray-100">All caught up!</h3>
        <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">Your assistant will let you know when there's an action to take.</p>
      </div>
    );
  }

  return (
    <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-2xl shadow-sm overflow-hidden flex flex-col">
      <div className="p-4 border-b border-gray-100 dark:border-gray-700/50 flex items-center justify-between">
        <h2 className="text-lg font-semibold font-outfit text-gray-900 dark:text-gray-100 flex items-center gap-2">
          <span className="text-[#0066FF] dark:text-[#0071E3]">✨</span> Assistant Insights
        </h2>
        <span className="text-xs font-medium bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 px-2.5 py-1 rounded-full">
          {insights.length} Actions
        </span>
      </div>

      <div className="divide-y divide-gray-100 dark:divide-gray-700/50">
        {insights.map((insight) => (
          <div key={insight.id} className="p-4 hover:bg-white/40 dark:hover:bg-gray-800/40 transition-colors">
            <div className="flex flex-col sm:flex-row sm:items-start justify-between gap-4">
              <div className="flex-1">
                <div className="flex items-center gap-2 mb-1">
                  <h3 className="font-semibold text-gray-900 dark:text-gray-100">{insight.title}</h3>
                  {insight.urgency === 'high' && (
                    <span className="w-2 h-2 rounded-full bg-[#FF3B30] dark:bg-[#DE1B1B]"></span>
                  )}
                </div>
                <p className="text-sm text-gray-600 dark:text-gray-400 leading-relaxed">
                  {insight.description}
                </p>
              </div>
              <button
                onClick={() => handleAction(insight.id)}
                className="w-full sm:w-auto shrink-0 bg-[#0066FF] hover:bg-[#0052cc] dark:bg-[#0071E3] dark:hover:bg-[#005bbb] text-white text-sm font-medium py-2 px-4 rounded-lg transition-colors whitespace-nowrap min-h-[44px] flex items-center justify-center"
              >
                {insight.actionLabel}
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
