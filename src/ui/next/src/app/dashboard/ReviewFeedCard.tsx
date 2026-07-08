import React, { useState } from 'react';




interface Review {
  id: string;
  rating: number;
  content: string;
  source: string;
  createdAtUnix: number;
}

interface ReviewResponse {
  id: string;
  draftedContent: string;
  status: string;
}

interface ReviewFeedCardProps {
  review: Review;
  response: ReviewResponse;
  onApprove: (responseId: string, updatedContent: string) => Promise<void>;
  onDismiss: (responseId: string) => Promise<void>;
}


const Card = ({ children, className }: any) => <div className={`rounded-xl border bg-white text-gray-950 shadow-sm ${className}`}>{children}</div>;
const CardHeader = ({ children, className }: any) => <div className={`flex flex-col space-y-1.5 p-6 ${className}`}>{children}</div>;
const CardTitle = ({ children, className }: any) => <h3 className={`font-semibold leading-none tracking-tight ${className}`}>{children}</h3>;
const CardContent = ({ children, className }: any) => <div className={`p-6 pt-0 ${className}`}>{children}</div>;
const CardFooter = ({ children, className }: any) => <div className={`flex items-center p-6 pt-0 ${className}`}>{children}</div>;
const Button = ({ children, variant, size, className, disabled, onClick }: any) => {
  const baseStyle = "inline-flex items-center min-h-[44px] min-w-[44px] justify-center rounded-md text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50";
  const sizeStyle = size === "sm" ? "h-11 px-3 text-xs" : "h-11 px-4 py-2";
  let vStyle = "bg-gray-900 text-gray-50 hover:bg-gray-900/90";
  if (variant === "outline") vStyle = "border border-gray-200 bg-white hover:bg-gray-100 hover:text-gray-900";
  if (variant === "ghost") vStyle = "hover:bg-gray-100 hover:text-gray-900";
  return <button disabled={disabled} onClick={onClick} className={`${baseStyle} ${sizeStyle} ${vStyle} ${className}`}>{children}</button>;
};
const Textarea = ({ value, onChange, className }: any) => <textarea value={value} onChange={onChange} className={`flex min-h-[60px] w-full rounded-md border border-gray-200 bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-gray-500 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-950 disabled:cursor-not-allowed disabled:opacity-50 ${className}`} />;
export function ReviewFeedCard({ review, response, onApprove, onDismiss }: ReviewFeedCardProps) {
  const [content, setContent] = useState(response.draftedContent);
  const [isEditing, setIsEditing] = useState(false);
  const [loading, setLoading] = useState(false);

  const handleApprove = async () => {
    setLoading(true);
    await onApprove(response.id, content);
    setLoading(false);
  };

  const handleDismiss = async () => {
    setLoading(true);
    await onDismiss(response.id);
    setLoading(false);
  };

  return (
    <Card className="rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 mb-4 shadow-sm">
      <CardHeader>
        <CardTitle className="text-sm font-semibold flex items-center justify-between">
          <span className="flex items-center gap-2">
            <span className="text-orange-600 bg-orange-100 p-1 rounded-full w-6 h-6 flex items-center justify-center">⭐️</span>
            New {review.rating}-Star Review ({review.source})
          </span>
          <span className="text-xs text-gray-500 font-normal">
            {new Date(review.createdAtUnix * 1000).toLocaleDateString()}
          </span>
        </CardTitle>
      </CardHeader>
      <CardContent className="text-sm">
        <p className="italic text-gray-700 mb-4">&quot;{review.content}&quot;</p>

        <div className="bg-white p-3 rounded-md border border-gray-100 shadow-sm">
          <p className="text-xs font-semibold text-gray-500 mb-2 uppercase tracking-wide">AI Drafted Reply</p>
          {isEditing ? (
            <Textarea
              value={content}
              onChange={(e) => setContent(e.target.value)}
              className="text-sm w-full min-h-[80px]"
            />
          ) : (
            <p className="text-gray-800">{content}</p>
          )}
        </div>
      </CardContent>
      <CardFooter className="flex justify-end gap-2">
        <Button variant="ghost" size="sm" onClick={handleDismiss} disabled={loading}>
          Dismiss
        </Button>
        {!isEditing && (
          <Button variant="outline" size="sm" onClick={() => setIsEditing(true)} disabled={loading}>
            Edit
          </Button>
        )}
        <Button size="sm" onClick={handleApprove} disabled={loading} className="bg-orange-600 hover:bg-orange-700 text-white">
          Approve & Post
        </Button>
      </CardFooter>
    </Card>
  );
}
