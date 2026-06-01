import React from 'react';

interface GlassCardProps extends React.HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
}

export const GlassCard: React.FC<GlassCardProps> = ({ children, className = '', ...props }) => {
  return (
    <div className={`mac-glass-container rounded-[16px] shadow-lg ${className}`} {...props}>
      {children}
    </div>
  );
};

interface GlassButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'outline';
  isLoading?: boolean;
}

export const GlassButton: React.FC<GlassButtonProps> = ({
  children,
  variant = 'primary',
  isLoading = false,
  className = '',
  disabled,
  ...props
}) => {
  const baseStyles = "w-full p-4 rounded-[8px] font-bold shadow-md active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2";

  const variants = {
    primary: "bg-[#0066FF] text-white hover:bg-[#0052cc]",
    secondary: "bg-white/70 dark:bg-white/10 backdrop-blur-md text-[#1D1D1F] dark:text-[#F5F5F7] border border-white/50 dark:border-white/10 hover:bg-white/90 dark:hover:bg-white/20",
    outline: "bg-transparent border border-[#0066FF] text-[#0066FF] hover:bg-[#0066FF]/10",
  };

  return (
    <button
      className={`${baseStyles} ${variants[variant]} ${className}`}
      disabled={disabled || isLoading}
      {...props}
    >
      {isLoading ? (
        <>
          <svg className="animate-spin h-5 w-5 text-current" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span>Processing...</span>
        </>
      ) : children}
    </button>
  );
};

interface GlassInputProps extends React.InputHTMLAttributes<HTMLInputElement | HTMLTextAreaElement> {
  label?: string;
  isTextArea?: boolean;
}

export const GlassInput: React.FC<GlassInputProps> = ({
  label,
  isTextArea = false,
  className = '',
  id,
  ...props
}) => {
  const Component = isTextArea ? 'textarea' : 'input';
  const inputId = id || (label ? label.toLowerCase().replace(/\s+/g, '-') : undefined);

  return (
    <div className="w-full">
      {label && (
        <label htmlFor={inputId} className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1 ml-1">
          {label}
        </label>
      )}
      <Component
        id={inputId}
        className={`w-full p-4 rounded-[12px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner focus:ring-2 focus:ring-[#0066FF]/20 ${isTextArea ? 'h-32 resize-none' : ''} ${className}`}
        {...(props as any)}
      />
    </div>
  );
};
export * from './StepProgress';
