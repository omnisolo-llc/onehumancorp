import React from 'react';

export interface PromoterCardProps {
    productName: string;
    imageUrl: string;
    draftCopy: string;
    onApprove: () => void;
    onEdit: () => void;
}

export const PromoterCard: React.FC<PromoterCardProps> = ({
    productName,
    imageUrl,
    draftCopy,
    onApprove,
    onEdit,
}) => {
    return (
        <div className="w-full max-w-[375px] mx-auto rounded-[16px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] p-4 mb-4 flex flex-col gap-3 shadow-sm">
            <div className="flex justify-between items-center">
                <div className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] flex items-center gap-2">
                    <span className="text-xl">✨</span> The Promoter
                </div>
                <div className="text-xs text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60">
                    New Product
                </div>
            </div>

            <div className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80">
                New product detected! Schedule a post for <span className="font-semibold">{productName}</span> to drive sales?
            </div>

            {imageUrl && (
                <div className="w-full h-48 rounded-[8px] overflow-hidden bg-black/5 dark:bg-white/5 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
                    <img src={imageUrl} alt={productName} className="w-full h-full object-cover" />
                </div>
            )}

            <div className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-3 rounded-[8px] whitespace-pre-wrap">
                "{draftCopy}"
            </div>

            <div className="flex flex-col gap-2 mt-2">
                <button
                    onClick={onApprove}
                    data-testid="promoter-approve-btn"
                    className="w-full min-h-[44px] bg-[#0066FF] hover:bg-blue-600 text-white rounded-[8px] font-medium transition-colors"
                >
                    Schedule Post
                </button>
                <button
                    onClick={onEdit}
                    data-testid="promoter-edit-btn"
                    className="w-full min-h-[44px] bg-white/50 dark:bg-gray-800/50 hover:bg-white/80 dark:hover:bg-gray-700/50 text-[#1D1D1F] dark:text-[#F5F5F7] rounded-[8px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] font-medium transition-colors"
                >
                    Edit Variants
                </button>
            </div>
        </div>
    );
};
