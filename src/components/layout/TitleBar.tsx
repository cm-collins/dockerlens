import { useTheme } from "@/components/theme-provider";
import { Moon, Sun, Monitor } from "lucide-react";
import { cn } from "@/lib/utils";

export function TitleBar({ title }: { title: string }) {
  const { theme, setTheme } = useTheme();

  return (
    <div className="h-12 flex items-center justify-between px-4 border-b bg-card/30 backdrop-blur-xl">
      {/* macOS Traffic Lights */}
      <div className="flex items-center gap-2">
        <div className="flex gap-2">
          <div className="w-3 h-3 rounded-full bg-red-500" />
          <div className="w-3 h-3 rounded-full bg-yellow-500" />
          <div className="w-3 h-3 rounded-full bg-green-500" />
        </div>
      </div>

      {/* Title */}
      <div className="absolute left-1/2 -translate-x-1/2 text-sm font-medium text-muted-foreground">
        {title}
      </div>

      {/* Theme Toggle */}
      <div className="flex items-center gap-1 bg-muted/50 rounded-lg p-1">
        <ThemeButton
          active={theme === "light"}
          onClick={() => setTheme("light")}
          icon={Sun}
          label="Light"
        />
        <ThemeButton
          active={theme === "dark"}
          onClick={() => setTheme("dark")}
          icon={Moon}
          label="Dark"
        />
        <ThemeButton
          active={theme === "system"}
          onClick={() => setTheme("system")}
          icon={Monitor}
          label="System"
        />
      </div>
    </div>
  );
}

// Extracted theme button component (DRY)
function ThemeButton({ 
  active, 
  onClick, 
  icon: Icon, 
  label 
}: { 
  active: boolean; 
  onClick: () => void; 
  icon: any; 
  label: string;
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "p-1.5 rounded-md transition-all",
        active && "bg-background shadow-sm"
      )}
      title={label}
    >
      <Icon className="w-3.5 h-3.5" />
    </button>
  );
}
