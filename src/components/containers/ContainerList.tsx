import { useDeferredValue, useEffect, useState } from "react";
import { LayoutPanelLeft, Search } from "lucide-react";
import { ContainerRow } from "@/components/containers/ContainerRow";
import {
  useApplyContainerAction,
  useContainersResponse,
} from "@/hooks/useContainers";
import { getErrorMessage } from "@/lib/errors";
import { useContainersStore } from "@/store/containers.store";

function formatBytes(value: number) {
  if (!value) {
    return "0 B";
  }

  const units = ["B", "KB", "MB", "GB", "TB"];
  const index = Math.min(Math.floor(Math.log(value) / Math.log(1024)), units.length - 1);
  const scaled = value / 1024 ** index;
  return `${scaled.toFixed(index === 0 ? 0 : 2)} ${units[index]}`;
}

function OverviewCard({
  title,
  value,
  caption,
}: {
  title: string;
  value: string;
  caption: string;
}) {
  return (
    <div className="rounded-[28px] border border-[rgb(var(--workspace-border))] bg-[rgb(var(--panel-bg))] px-6 py-5 shadow-[inset_0_1px_0_rgba(255,255,255,0.03)]">
      <div className="text-[15px] font-medium text-[rgb(var(--workspace-soft))]">{title}</div>
      <div className="mt-4 text-[36px] font-semibold tracking-[-0.05em] text-[rgb(var(--workspace-foreground))]">
        {value}
      </div>
      <p className="mt-2 text-[14px] text-[rgb(var(--workspace-muted))]">{caption}</p>
    </div>
  );
}

export function ContainerList() {
  const { selectedId, setSelectedId } = useContainersStore();
  const [search, setSearch] = useState("");
  const [onlyRunning, setOnlyRunning] = useState(false);
  const [selectedIds, setSelectedIds] = useState<string[]>([]);
  const [actionError, setActionError] = useState("");
  const deferredSearch = useDeferredValue(search.trim());
  const bulkAction = useApplyContainerAction();
  const { data, isLoading, isError } = useContainersResponse({
    all: true,
    only_running: onlyRunning,
    search: deferredSearch || null,
  });
  const listResponse = data ?? {
    items: [],
    overview: {
      total: 0,
      running: 0,
      paused: 0,
      exited: 0,
      total_cpu_percent: 0,
      total_memory_usage_bytes: 0,
      total_memory_limit_bytes: 0,
      generated_at_ms: 0,
    },
    total_count: 0,
    filtered_count: 0,
    generated_at_ms: 0,
  };

  useEffect(() => {
    if (selectedId && !listResponse.items.some((item) => item.id === selectedId)) {
      setSelectedId(null);
    }

    setSelectedIds((current) =>
      current.filter((id) => listResponse.items.some((item) => item.id === id)),
    );
  }, [listResponse.items, selectedId, setSelectedId]);

  const allVisibleSelected =
    listResponse.items.length > 0 && selectedIds.length === listResponse.items.length;

  const toggleSelection = (id: string) => {
    setSelectedIds((current) =>
      current.includes(id) ? current.filter((value) => value !== id) : [...current, id],
    );
  };

  const toggleAllVisible = () => {
    setSelectedIds(allVisibleSelected ? [] : listResponse.items.map((item) => item.id));
  };

  const handleBulkAction = async (type: "start" | "stop" | "remove") => {
    if (!selectedIds.length) {
      return;
    }

    setActionError("");

    try {
      await bulkAction.mutateAsync({
        ids: selectedIds,
        action:
          type === "remove"
            ? { type: "remove", force: true, remove_volumes: false }
            : { type },
      });

      setSelectedIds([]);
    } catch (error) {
      setActionError(
        getErrorMessage(
          error,
          "Bulk action failed. Check the Docker daemon state and the selected containers.",
        ),
      );
    }
  };

  return (
    <section className="flex min-h-0 flex-1 flex-col px-4 pb-5 pt-5 sm:px-6 lg:px-10 lg:pb-8 lg:pt-8">
      <div className="grid grid-cols-1 gap-4 xl:grid-cols-[minmax(0,1fr)_minmax(0,1fr)_auto] xl:gap-6">
        <OverviewCard
          title="Container CPU usage"
          value={`${listResponse.overview.total_cpu_percent.toFixed(2)}%`}
          caption={
            listResponse.overview.running
              ? `${listResponse.overview.running} running container${listResponse.overview.running === 1 ? "" : "s"}`
              : "No containers are running."
          }
        />
        <OverviewCard
          title="Container memory usage"
          value={formatBytes(listResponse.overview.total_memory_usage_bytes)}
          caption={
            listResponse.overview.total_memory_limit_bytes
              ? `${formatBytes(listResponse.overview.total_memory_limit_bytes)} available memory tracked`
              : "No memory limit reported yet."
          }
        />
        <div className="flex items-end justify-start xl:justify-end xl:pb-2">
          <button
            type="button"
            className="text-[18px] font-medium text-[rgb(var(--link))] transition hover:text-[rgb(var(--link-strong))]"
          >
            Show charts
          </button>
        </div>
      </div>

      <div className="mt-6 flex flex-col gap-5 xl:mt-8 xl:flex-row xl:items-center xl:justify-between xl:gap-6">
        <div className="flex flex-col gap-4 xl:flex-row xl:items-center xl:gap-5">
          <div className="flex h-14 w-full items-center gap-4 rounded-[20px] border border-[rgb(var(--workspace-border))] bg-[rgb(var(--panel-bg))] px-5 xl:w-[420px]">
            <Search className="h-5 w-5 text-[rgb(var(--workspace-muted))]" />
            <input
              type="text"
              value={search}
              onChange={(event) => setSearch(event.target.value)}
              placeholder="Search"
              className="flex-1 bg-transparent text-[16px] text-[rgb(var(--workspace-foreground))] placeholder:text-[rgb(var(--workspace-muted))] focus:outline-none"
            />
          </div>

          <button
            type="button"
            className="hidden h-14 w-14 items-center justify-center rounded-[18px] border border-[rgb(var(--workspace-border))] bg-[rgb(var(--panel-bg))] text-[rgb(var(--link))] md:flex"
            title="Change table density"
          >
            <LayoutPanelLeft className="h-6 w-6" />
          </button>

          <label className="flex items-center gap-4 text-[15px] text-[rgb(var(--workspace-foreground))] sm:text-[16px]">
            <button
              type="button"
              onClick={() => setOnlyRunning((value) => !value)}
              className={`relative flex h-9 w-16 items-center rounded-full border transition ${
                onlyRunning
                  ? "border-[rgb(var(--link))] bg-[rgb(var(--link))]"
                  : "border-[rgb(var(--workspace-border))] bg-[rgb(var(--workspace-card))]"
              }`}
              aria-label="Only show running containers"
            >
              <span
                className={`h-7 w-7 rounded-full bg-white shadow transition ${
                  onlyRunning ? "translate-x-8" : "translate-x-1"
                }`}
              />
            </button>
            <span>Only show running containers</span>
          </label>
        </div>

        {selectedIds.length ? (
          <div className="flex flex-wrap items-center gap-3">
            <span className="text-[14px] text-[rgb(var(--workspace-muted))]">
              {selectedIds.length} selected
            </span>
            <button
              type="button"
              onClick={() => handleBulkAction("start")}
              className="rounded-2xl border border-[rgb(var(--link))/0.28] bg-[rgb(var(--workspace-card))] px-4 py-2.5 text-[14px] font-semibold text-[rgb(var(--link))]"
            >
              Start
            </button>
            <button
              type="button"
              onClick={() => handleBulkAction("stop")}
              className="rounded-2xl border border-[rgb(var(--workspace-border))] bg-[rgb(var(--workspace-card))] px-4 py-2.5 text-[14px] font-semibold text-[rgb(var(--workspace-foreground))]"
            >
              Stop
            </button>
            <button
              type="button"
              onClick={() => handleBulkAction("remove")}
              className="rounded-2xl border border-[rgb(var(--danger))/0.24] bg-[rgb(var(--workspace-card))] px-4 py-2.5 text-[14px] font-semibold text-[rgb(var(--danger))]"
            >
              Remove
            </button>
          </div>
        ) : null}
      </div>

      <div className="mt-6 flex min-h-0 flex-1 flex-col overflow-hidden rounded-[24px] border border-[rgb(var(--workspace-border))] bg-[rgb(var(--table-bg))] shadow-[0_28px_80px_rgba(3,9,20,0.32)] lg:rounded-[30px]">
        {actionError ? (
          <div className="border-b border-[rgb(var(--danger))/0.25] bg-[rgb(var(--danger))/0.12] px-6 py-4 text-[14px] text-[rgb(var(--danger))]">
            {actionError}
          </div>
        ) : null}

        <div className="min-h-0 flex-1 overflow-auto">
          <table className="w-full border-separate border-spacing-0">
            <thead className="sticky top-0 z-10 bg-[rgb(var(--table-bg))]">
              <tr className="text-left text-[15px] font-semibold text-[rgb(var(--workspace-foreground))]">
                <th className="w-[56px] border-b border-[rgb(var(--workspace-border))] px-5 py-4">
                  <button
                    type="button"
                    onClick={toggleAllVisible}
                    className={`flex h-7 w-7 items-center justify-center rounded-[10px] border transition ${
                      allVisibleSelected
                        ? "border-[rgb(var(--link))] bg-[rgb(var(--link))] text-white"
                        : "border-[rgb(var(--workspace-border))] bg-transparent text-transparent hover:border-[rgb(var(--link))]"
                    }`}
                    aria-label="Select all visible containers"
                  >
                    <span className="text-[11px] font-bold">✓</span>
                  </button>
                </th>
                <th className="w-[48px] border-b border-[rgb(var(--workspace-border))] py-4" />
                <th className="border-b border-[rgb(var(--workspace-border))] px-4 py-4">Name</th>
                <th className="border-b border-[rgb(var(--workspace-border))] px-4 py-4">Container ID</th>
                <th className="border-b border-[rgb(var(--workspace-border))] px-4 py-4">Image</th>
                <th className="border-b border-[rgb(var(--workspace-border))] px-4 py-4">Port(s)</th>
                <th className="border-b border-[rgb(var(--workspace-border))] px-4 py-4">CPU (%)</th>
                <th className="border-b border-[rgb(var(--workspace-border))] px-4 py-4">Memory usage</th>
                <th className="border-b border-[rgb(var(--workspace-border))] px-4 py-4 text-right">
                  Actions
                </th>
              </tr>
            </thead>

            <tbody>
              {listResponse.items.map((container) => (
                <ContainerRow
                  key={container.id}
                  container={container}
                  checked={selectedIds.includes(container.id)}
                  selected={selectedId === container.id}
                  onToggleChecked={() => toggleSelection(container.id)}
                  onSelect={() => setSelectedId(container.id)}
                  onActionError={setActionError}
                />
              ))}
            </tbody>
          </table>

          {!isLoading && !listResponse.items.length ? (
            <div className="flex h-[280px] items-center justify-center">
              <div className="text-center">
                <div className="text-[20px] font-semibold text-[rgb(var(--workspace-foreground))]">
                  No containers found
                </div>
                <div className="mt-2 text-[14px] text-[rgb(var(--workspace-muted))]">
                  Adjust the search or wait for Docker to report active workloads.
                </div>
              </div>
            </div>
          ) : null}

          {isLoading ? (
            <div className="flex h-[280px] items-center justify-center text-[15px] text-[rgb(var(--workspace-muted))]">
              Loading containers...
            </div>
          ) : null}

          {isError ? (
            <div className="flex h-[280px] items-center justify-center text-[15px] text-[rgb(var(--danger))]">
              Failed to load containers from Docker.
            </div>
          ) : null}
        </div>

        <div className="flex flex-wrap items-center justify-between gap-3 border-t border-[rgb(var(--workspace-border))] bg-[rgb(var(--table-footer-bg))] px-4 py-4 sm:px-6">
          <div className="hidden h-1.5 w-64 rounded-full bg-[rgb(var(--workspace-border))] sm:block">
            <div className="h-1.5 w-40 rounded-full bg-[rgb(var(--link))/0.65]" />
          </div>

          <div className="text-[16px] text-[rgb(var(--workspace-foreground))]">
            Showing {listResponse.filtered_count} items
          </div>
        </div>
      </div>
    </section>
  );
}
