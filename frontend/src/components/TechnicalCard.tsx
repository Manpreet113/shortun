import { cn } from "@/lib/utils";

interface TechnicalCardProps extends React.HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
}

export const TechnicalCard = ({ children, className, ...props }: TechnicalCardProps) => {
  return (
    <div className={cn("relative p-1", className)} {...props}>
      {/* Top Left Corner */}
      <div className="absolute top-0 left-0 w-2 h-2 border-t-2 border-l-2 border-primary" />
      {/* Top Right Corner */}
      <div className="absolute top-0 right-0 w-2 h-2 border-t-2 border-r-2 border-primary" />
      {/* Bottom Left Corner */}
      <div className="absolute bottom-0 left-0 w-2 h-2 border-b-2 border-l-2 border-primary" />
      {/* Bottom Right Corner */}
      <div className="absolute bottom-0 right-0 w-2 h-2 border-b-2 border-r-2 border-primary" />
      
      {/* Content container with subtle border */}
      <div className="bg-card border border-border/50 p-6 h-full">
        {children}
      </div>
    </div>
  );
};