import * as React from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "../../lib/utils";

const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-lg text-sm font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-400 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950 disabled:pointer-events-none disabled:opacity-40",
  {
    variants: {
      variant: {
        default:
          "bg-white text-zinc-900 shadow-sm hover:bg-zinc-100 active:scale-[0.98]",
        destructive:
          "bg-red-600 text-white hover:bg-red-700 active:scale-[0.98]",
        outline:
          "border border-zinc-700 bg-transparent text-zinc-200 hover:bg-zinc-800 hover:text-zinc-100 active:scale-[0.98]",
        secondary:
          "bg-zinc-800 text-zinc-100 hover:bg-zinc-700 active:scale-[0.98]",
        ghost:
          "shadow-none text-zinc-500 hover:bg-zinc-800/60 hover:text-zinc-200",
        link: "shadow-none text-zinc-300 underline-offset-4 hover:underline hover:text-zinc-100",
      },
      size: {
        default: "h-10 px-6 py-2 min-w-[120px]",
        sm: "h-9 px-4 text-sm",
        lg: "h-11 px-10 min-w-[160px]",
        icon: "h-10 w-10 min-w-0",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  },
);

export interface ButtonProps
  extends
    React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, ...props }, ref) => {
    return (
      <button
        className={cn(buttonVariants({ variant, size, className }))}
        ref={ref}
        {...props}
      />
    );
  },
);
Button.displayName = "Button";

export { Button, buttonVariants };
