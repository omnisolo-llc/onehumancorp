import React from 'react';

export const LedgerStatementModal = ({ isOpen, onClose, statement }: any) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
      <div className="bg-white dark:bg-zinc-900 rounded-[16px] shadow-xl w-full max-w-lg overflow-hidden border border-zinc-200 dark:border-zinc-800">
        <div className="p-4 border-b border-zinc-200 dark:border-zinc-800 flex justify-between items-center">
          <h2 className="text-lg font-semibold text-zinc-900 dark:text-zinc-50">Ledger Statement</h2>
          <button onClick={onClose} className="text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300">
            &times;
          </button>
        </div>
        <div className="p-4 max-h-[60vh] overflow-y-auto">
          {statement && statement.length > 0 ? (
            <ul className="space-y-3">
              {statement.map((item: any) => (
                <li key={item.id} className="flex justify-between items-center p-3 bg-zinc-50 dark:bg-zinc-800/50 rounded-lg">
                  <div>
                    <p className="text-sm font-medium text-zinc-900 dark:text-zinc-100">{item.description}</p>
                    <p className="text-xs text-zinc-500">{new Date(item.date).toLocaleDateString()}</p>
                  </div>
                  <div className={`text-sm font-semibold ${item.type === 'CREDIT' ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'}`}>
                    {item.type === 'CREDIT' ? '+' : '-'}${Math.abs(item.amount).toFixed(2)}
                  </div>
                </li>
              ))}
            </ul>
          ) : (
            <p className="text-sm text-zinc-500 text-center py-8">No recent activity.</p>
          )}
        </div>
      </div>
    </div>
  );
};
