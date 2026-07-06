import React, { useEffect } from 'react';
import { AnimatePresence, motion } from 'framer-motion';

interface BottomSheetProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  children: React.ReactNode;
}

export const BottomSheet: React.FC<BottomSheetProps> = ({ isOpen, onClose, title, children }) => {
  // Prevent body scroll when open
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
    return () => {
      document.body.style.overflow = '';
    };
  }, [isOpen]);

  return (
    <AnimatePresence>
      {isOpen && (
        <>
          {/* Backdrop */}
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={onClose}
            className="fixed inset-0 bg-black/40 z-[100] backdrop-blur-sm"
            data-testid="bottom-sheet-backdrop"
          />
          {/* Sheet */}
          <motion.div
            initial={{ y: "100%" }}
            animate={{ y: 0 }}
            exit={{ y: "100%" }}
            transition={{ type: "spring", damping: 25, stiffness: 200 }}
            className="fixed bottom-0 left-0 right-0 z-[101] bg-[rgba(255,255,255,0.85)] dark:bg-[rgba(22,22,26,0.85)] backdrop-blur-[40px] saturate-[210%] border-t border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-t-3xl shadow-[0_-10px_40px_rgba(0,0,0,0.1)] max-h-[85vh] overflow-y-auto flex flex-col"
            data-testid="bottom-sheet"
          >
            <div className="flex justify-center pt-3 pb-2 w-full shrink-0" onClick={onClose} style={{ cursor: 'pointer' }}>
              <div className="w-12 h-1.5 bg-gray-300 dark:bg-gray-600 rounded-full" />
            </div>
            {title && (
              <div className="px-6 pb-2 shrink-0">
                <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">{title}</h2>
              </div>
            )}
            <div className="p-6 pt-2 overflow-y-auto">
              {children}
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
};
