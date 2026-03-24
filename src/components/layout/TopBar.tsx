import { MessageSquareMore, PanelLeftClose, RefreshCw } from "lucide-react";

type TopBarProps = {
  title: string;
  subtitle?: string;
  onRefresh?: () => void;
};

export function TopBar({ title, subtitle, onRefresh }: TopBarProps) {
  return (
    <div className="flex min-h-[92px] flex-wrap items-center justify-between gap-4 border-b border-white/8 px-4 py-5 sm:px-6 lg:px-10">
      <div className="flex items-center gap-4 sm:gap-5">
        <button
          type="button"
          className="hidden h-12 w-12 items-center justify-center rounded-2xl border border-[rgb(var(--workspace-border))] bg-[rgb(var(--panel-bg))] text-[rgb(var(--link))] shadow-[inset_0_0_0_1px_rgba(255,255,255,0.03)] lg:flex"
          title="Collapse navigation"
        >
          <PanelLeftClose className="h-5 w-5" />
        </button>

        <div>
          <div className="flex items-center gap-5">
            <h1 className="text-[28px] font-semibold tracking-[-0.05em] text-[rgb(var(--workspace-foreground))] sm:text-[38px]">
              {title}
            </h1>

            <button
              type="button"
              className="hidden items-center gap-2 text-[15px] font-medium text-[rgb(var(--link))] transition hover:text-[rgb(var(--link-strong))] md:flex"
            >
              <span>Give feedback</span>
              <MessageSquareMore className="h-4 w-4" />
            </button>
          </div>

          {subtitle ? (
            <p className="mt-1 text-[14px] text-[rgb(var(--workspace-muted))]">{subtitle}</p>
          ) : null}
        </div>
      </div>

      {onRefresh ? (
        <button
          type="button"
          onClick={onRefresh}
          className="flex h-11 items-center gap-3 rounded-2xl border border-[rgb(var(--workspace-border))] bg-[rgb(var(--panel-bg))] px-4 text-[14px] font-semibold text-[rgb(var(--workspace-foreground))] transition hover:border-[rgb(var(--link))] hover:text-[rgb(var(--link))] sm:h-12 sm:px-5"
        >
          <RefreshCw className="h-4 w-4" />
          Refresh
        </button>
      ) : null}
    </div>
  );
}
