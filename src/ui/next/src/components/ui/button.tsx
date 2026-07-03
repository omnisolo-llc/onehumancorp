import * as React from "react"

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link"
  size?: "default" | "sm" | "lg" | "icon"
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = "default", size = "default", ...props }, ref) => {
    let variantClasses = ""
    switch (variant) {
      case "destructive":
        variantClasses = "bg-red-500 text-white hover:bg-red-600"
        break
      case "outline":
        variantClasses = "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
        break
      case "secondary":
        variantClasses = "bg-secondary text-secondary-foreground hover:bg-secondary/80"
        break
      case "ghost":
        variantClasses = "hover:bg-accent hover:text-accent-foreground"
        break
      case "link":
        variantClasses = "text-primary underline-offset-4 hover:underline"
        break
      default:
        variantClasses = "bg-[#0066FF] dark:bg-[#0071E3] text-white hover:bg-[#0066FF]/90 dark:hover:bg-[#0071E3]/90"
    }

    let sizeClasses = ""
    switch (size) {
      case "sm":
        sizeClasses = "h-9 px-3"
        break
      case "lg":
        sizeClasses = "h-11 px-8"
        break
      case "icon":
        sizeClasses = "h-10 w-10"
        break
      default:
        sizeClasses = "h-10 px-4 py-2"
    }

    return (
      <button
        className={
          `inline-flex items-center justify-center whitespace-nowrap rounded-[8px] text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 ${variantClasses} ${sizeClasses} ${className || ""}`
        }
        ref={ref}
        {...props}
      />
    )
  }
)
Button.displayName = "Button"

export { Button }
