import React from 'react';

export function MemberAnalytics({ usageLogs }: { usageLogs: any[] }) {
  return (
    <div className="app-panel-body p-6">
      {usageLogs.length === 0 ? (
        <p className="text-sm text-gray-500 text-center py-4 font-outfit">No workspace member usage logged yet.</p>
      ) : (
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-800 text-sm font-outfit">
            <thead>
              <tr className="text-xs font-bold uppercase tracking-wider text-gray-400 text-left">
                <th className="pb-3 pr-4">Username</th>
                <th className="pb-3 px-4">Feature</th>
                <th className="pb-3 px-4 text-right">Tokens Used</th>
                <th className="pb-3 pl-4 text-right">Computed Cost</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-100 dark:divide-gray-800/50">
              {usageLogs.map((log, index) => (
                <tr key={index} className="text-gray-700 dark:text-gray-300">
                  <td className="py-3 pr-4 font-semibold">{log.username}</td>
                  <td className="py-3 px-4 text-xs font-mono bg-gray-50 dark:bg-gray-800/30 rounded inline-block my-1">{log.feature}</td>
                  <td className="py-3 px-4 text-right font-mono">{log.tokens_used.toLocaleString()}</td>
                  <td className="py-3 pl-4 text-right font-mono text-green-600 dark:text-green-400">${log.computed_cost.toFixed(4)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
