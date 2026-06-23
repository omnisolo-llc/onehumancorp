import React, { useState } from 'react';

interface AgentActionCardProps {
  id: string;
  department: string;
  icon?: React.ReactNode;
  headerColorClass?: string;
  bodyContent: React.ReactNode;
  isEditing: boolean;
  editContent?: string;
  onEditContentChange?: (val: string) => void;
  onApprove: (id: string, content?: string) => void;
  onReject: (id: string) => void;
  onEditToggle: (id: string, initialText?: string) => void;
  onSaveApprove: (id: string, content: string) => void;
  onCancelEdit: () => void;
  defaultEditText?: string;
  approveLabel?: string;
  rejectLabel?: string;
  approveColorClass?: string;
  testIdPrefix?: string;
  renderEditArea?: (content: string, setContent: (c: string) => void) => React.ReactNode;
  customActions?: React.ReactNode;
}

export const AgentActionCard: React.FC<AgentActionCardProps> = ({
  id,
  department,
  icon,
  headerColorClass = "text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30",
  bodyContent,
  isEditing,
  editContent = "",
  onEditContentChange,
  onApprove,
  onReject,
  onEditToggle,
  onSaveApprove,
  onCancelEdit,
  defaultEditText = "",
  approveLabel = "Approve",
  rejectLabel = "Dismiss",
  approveColorClass = "bg-green-500 hover:bg-green-600 text-white",
  testIdPrefix = "agent-action",
  renderEditArea,
  customActions
}) => {
  return (
    <div
      className="glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-5 rounded-[16px] shadow-sm flex flex-col gap-4 transition-all duration-300 w-full"
      data-testid={`${testIdPrefix}-card`}
    >
      <div className="flex items-center justify-between">
        <span className={`text-xs font-bold font-outfit uppercase tracking-wider px-2 py-1 rounded-[8px] flex items-center gap-2 ${headerColorClass}`}>
          {icon && icon}
          {department}
        </span>
      </div>

      <div className="flex flex-col gap-3">
        {bodyContent}
      </div>

      {isEditing ? (
        <div className="flex flex-col gap-3 w-full mt-2">
          {renderEditArea ? (
            renderEditArea(editContent, onEditContentChange || (() => {}))
          ) : (
            <textarea
              className="w-full min-h-[44px] p-3 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none"
              rows={4}
              value={editContent}
              onChange={(e) => onEditContentChange && onEditContentChange(e.target.value)}
              data-testid={`${testIdPrefix}-edit-textarea`}
              autoFocus
            />
          )}
          <div className="flex gap-3">
            <button
              onClick={() => onSaveApprove(id, editContent)}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center"
              data-testid={`${testIdPrefix}-save-approve`}
            >
              Save & Approve
            </button>
            <button
              onClick={onCancelEdit}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
              data-testid={`${testIdPrefix}-cancel-edit`}
            >
              Cancel
            </button>
          </div>
        </div>
      ) : (
        <div className="flex flex-col gap-3 mt-2">
          {customActions ? (
            customActions
          ) : (
            <>
              <button
                onClick={() => onApprove(id)}
                className={`w-full min-h-[44px] min-w-[44px] px-4 rounded-[8px] font-medium transition-all duration-200 shadow-md flex items-center justify-center ${approveColorClass}`}
                aria-label={approveLabel}
                data-testid={`${testIdPrefix}-approve`}
              >
                {approveLabel}
              </button>
              <div className="flex flex-col sm:flex-row gap-3 w-full">
                {onEditToggle && (
                  <button
                    onClick={() => onEditToggle(id, defaultEditText)}
                    className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                    aria-label="Edit proposal"
                    data-testid={`${testIdPrefix}-edit`}
                  >
                    Edit
                  </button>
                )}
                <button
                  onClick={() => onReject(id)}
                  className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                  aria-label={rejectLabel}
                  data-testid={`${testIdPrefix}-dismiss`}
                >
                  {rejectLabel}
                </button>
              </div>
            </>
          )}
        </div>
      )}
    </div>
  );
};
