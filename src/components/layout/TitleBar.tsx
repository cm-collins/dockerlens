import { Bell, Boxes, Grid3x3, HelpCircle, Moon, Search, Settings2, Sun } from "lucide-react";
import { useTheme } from "@/components/theme-provider";
import { cn } from "@/lib/utils";

function ThemeToggleButton({
  active,
  onClick,
  icon: Icon,
  label,
}: {
  active: boolean;
  onClick: () => void;
  icon: typeof Sun;
  label: string;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={cn(
        "flex h-9 w-9 items-center justify-center rounded-xl text-white/70 transition hover:text-white",
        active && "bg-white/12 text-white shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08)]",
      )}
      title={label}
    >
      <Icon className="h-4 w-4" />
    </button>
  );
}

function HeaderIconButton({
  icon: Icon,
  label,
  badge,
}: {
  icon: typeof Bell;
  label: string;
  badge?: number;
}) {
  return (
    <button
      type="button"
      className="relative flex h-10 w-10 items-center justify-center rounded-2xl text-white/80 transition hover:bg-white/10 hover:text-white"
      title={label}
    >
      <Icon className="h-5 w-5" />
      {badge ? (
        <span className="absolute -right-0.5 -top-0.5 flex h-5 min-w-5 items-center justify-center rounded-full bg-[rgb(var(--danger))] px-1 text-[10px] font-bold text-white">
          {badge}
        </span>
      ) : null}
    </button>
  );
}

export function TitleBar() {
  const { theme, setTheme } = useTheme();

  return (
    <header className="flex min-h-[84px] flex-wrap items-center justify-between gap-4 border-b border-white/10 bg-[linear-gradient(90deg,rgb(var(--shell-blue-start)),rgb(var(--shell-blue-end)))] px-4 py-4 text-white shadow-[0_14px_40px_rgba(4,13,39,0.38)] lg:px-8">
      <div className="flex min-w-0 items-center gap-4 lg:gap-5">
        <div className="flex items-center gap-3">
          <div className="flex h-11 w-11 items-center justify-center rounded-2xl bg-white/12 shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08)]">
            <Boxes className="h-6 w-6" />
          </div>

          <div className="flex min-w-0 items-center gap-3">
            <div>
              <div className="text-[11px] font-semibold uppercase tracking-[0.34em] text-white/70 sm:text-[14px]">
                DockerLens
              </div>
              <div className="text-[20px] font-semibold tracking-[-0.045em] sm:text-[26px]">docker.desktop</div>
            </div>
            <span className="hidden rounded-full bg-white/14 px-3 py-1 text-[12px] font-semibold uppercase tracking-[0.12em] text-white/90 sm:inline-flex">
              Personal
            </span>
          </div>
        </div>
      </div>

      <div className="order-3 flex basis-full justify-center lg:order-none lg:mx-8 lg:flex-1 lg:basis-auto">
        <div className="flex h-12 w-full max-w-[590px] items-center gap-4 rounded-[18px] border border-white/12 bg-white/10 px-4 shadow-[inset_0_1px_0_rgba(255,255,255,0.08)] backdrop-blur-xl sm:h-14 sm:px-5">
          <Search className="h-5 w-5 text-white/75" />
          <input
            type="text"
            readOnly
            value=""
            placeholder="Search"
            className="flex-1 bg-transparent text-[14px] font-medium text-white placeholder:text-white/65 focus:outline-none sm:text-[15px]"
          />
          <span className="hidden rounded-xl border border-white/18 bg-white/8 px-3 py-1.5 text-[12px] font-semibold tracking-[0.12em] text-white/85 md:inline-flex">
            ⌘K
          </span>
        </div>
      </div>

      <div className="flex items-center gap-2 self-start lg:self-auto">
        <HeaderIconButton icon={HelpCircle} label="Help" />
        <div className="hidden sm:block">
          <HeaderIconButton icon={Bell} label="Notifications" badge={2} />
        </div>
        <div className="hidden xl:block">
          <HeaderIconButton icon={Boxes} label="Containers" />
        </div>
        <div className="hidden lg:block">
          <HeaderIconButton icon={Settings2} label="Settings" />
        </div>
        <div className="hidden xl:block">
          <HeaderIconButton icon={Grid3x3} label="Apps" />
        </div>

        <div className="ml-2 flex items-center gap-1 rounded-[18px] border border-white/10 bg-white/8 p-1.5">
          <ThemeToggleButton
            active={theme === "light"}
            onClick={() => setTheme("light")}
            icon={Sun}
            label="Light mode"
          />
          <ThemeToggleButton
            active={theme === "dark"}
            onClick={() => setTheme("dark")}
            icon={Moon}
            label="Dark mode"
          />
        </div>

        <button
          type="button"
          className="ml-1 hidden rounded-2xl bg-white px-5 py-3 text-[14px] font-semibold text-[rgb(var(--shell-blue-end))] shadow-[0_12px_30px_rgba(0,0,0,0.18)] transition hover:-translate-y-px lg:inline-flex"
        >
          Sign in
        </button>
      </div>
    </header>
  );
}
