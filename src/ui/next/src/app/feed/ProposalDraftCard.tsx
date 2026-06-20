import React, { useState } from 'react';

interface Props {
  item: any;
  onApprove: (id: string, customEdits?: string) => Promise<void>;
  onDismiss: (id: string) => Promise<void>;
  isProcessing: boolean;
}

export function ProposalDraftCard({ item, onApprove, onDismiss, isProcessing }: Props) {
  const [edits, setEdits] = useState("");
  const [isEditing, setIsEditing] = useState(false);
  const payload = item.context_payload || item.proposed_action || {};

  const handleApprove = () => {
    onApprove(item.id, edits || undefined);
  };

  return (
    <div className="bg-white/80 dark:bg-zinc-800/80 backdrop-blur-xl rounded-2xl p-4 shadow-lg border border-white/20 dark:border-white/10 w-[343px] md:w-full mx-auto flex flex-col gap-3">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-semibold tracking-tight text-zinc-900 dark:text-zinc-100 flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-blue-500 shadow-[0_0_8px_rgba(59,130,246,0.8)]"></span>
          Proposal Draft Ready
        </h3>
        <span className="text-xs font-medium px-2 py-1 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 rounded-full">
          Sales
        </span>
      </div>

      <div className="text-sm text-zinc-600 dark:text-zinc-400">
        <p><strong>Client:</strong> {payload.client_name} ({payload.client_email})</p>
        <p><strong>Service:</strong> {payload.service}</p>
        <p><strong>Estimated Price:</strong> ${payload.suggested_price?.toFixed(2) || "0.00"}</p>
      </div>

      <div className="p-3 bg-zinc-50 dark:bg-zinc-900/50 rounded-xl text-sm text-zinc-800 dark:text-zinc-300">
        {isEditing ? (
           <textarea
             className="w-full bg-transparent border border-zinc-200 dark:border-zinc-700 rounded-lg p-2 text-sm focus:ring-2 focus:ring-blue-500 focus:outline-none"
             rows={4}
             value={edits !== "" ? edits : payload.generated_response || ''}
             onChange={(e) => setEdits(e.target.value)}
           />
        ) : (
           <p className="line-clamp-4">{payload.generated_response}</p>
        )}
      </div>

      <div className="flex flex-col gap-2 mt-2">
        {isEditing ? (
          <button
            onClick={() => setIsEditing(false)}
            className="text-xs font-medium text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 text-left"
          >
            Cancel Edit
          </button>
        ) : (
          <div className="flex gap-2">
             <button
                onClick={() => { setIsEditing(true); setEdits(payload.generated_response || ''); }}
                className="text-xs font-medium text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 text-left"
              >
                Quick Edit Draft
             </button>
             <button
                onClick={() => onApprove(item.id, "Add Rush Fee")}
                className="text-xs font-medium text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 text-left ml-auto"
                disabled={isProcessing}
              >
                + Add Rush Fee
             </button>
          </div>
        )}
      </div>

      <div className="flex gap-3 pt-2">
        <button
          onClick={handleApprove}
          disabled={isProcessing}
          className="flex-1 bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 font-medium py-2.5 px-4 rounded-xl text-sm transition-transform active:scale-[0.98] disabled:opacity-50"
        >
          {isProcessing ? 'Sending...' : 'Approve & Send'}
        </button>
        <button
          onClick={() => onDismiss(item.id)}
          disabled={isProcessing}
          className="flex-1 bg-zinc-100 dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100 font-medium py-2.5 px-4 rounded-xl text-sm transition-transform active:scale-[0.98] disabled:opacity-50"
        >
          Dismiss
        </button>
      </div>
    </div>
  );
}
