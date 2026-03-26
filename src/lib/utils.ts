import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

/**
 * Utility for merging Tailwind classes
 * Follows shadcn/ui pattern for className composition
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * Format Unix timestamp to relative time
 * @example formatRelativeTime(1234567890) => "2 days ago"
 */
export function formatRelativeTime(unix: number): string {
  const date = new Date(unix * 1000);
  const now = Date.now();
  const diff = now - date.getTime();

  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (days > 0) return `${days} day${days > 1 ? "s" : ""} ago`;
  if (hours > 0) return `${hours} hour${hours > 1 ? "s" : ""} ago`;
  if (minutes > 0) return `${minutes} minute${minutes > 1 ? "s" : ""} ago`;
  return "Just now";
}

/**
 * Get container state color class
 * DRY: Single source of truth for state colors
 */
export function getStateColor(state: string): string {
  const stateMap: Record<string, string> = {
    running: "text-green-500",
    exited: "text-red-500",
    paused: "text-yellow-500",
    restarting: "text-blue-500",
  };
  return stateMap[state.toLowerCase()] || "text-muted-foreground";
}

/**
 * Get container state background color
 */
export function getStateBg(state: string): string {
  const stateMap: Record<string, string> = {
    running: "bg-green-500/10",
    exited: "bg-red-500/10",
    paused: "bg-yellow-500/10",
    restarting: "bg-blue-500/10",
  };
  return stateMap[state.toLowerCase()] || "bg-muted";
}
