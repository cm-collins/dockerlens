import { MoreVertical, Play, Square, Trash2 } from "lucide-react";
import {
  useRemoveContainer,
  useStartContainer,
  useStopContainer,
  useUnpauseContainer,
} from "@/hooks/useContainers";
import { getErrorMessage } from "@/lib/errors";
import type { ContainerSummary } from "@/types/docker";
import { cn } from "@/lib/utils";

function formatMemory(bytes?: number | null) {
  if (!bytes) {
    return "N/A";
  }

  if (bytes < 1024 ** 2) {
    return `${Math.round(bytes / 1024)} KB`;
  }

  if (bytes < 1024 ** 3) {
    return `${(bytes / 1024 ** 2).toFixed(1)} MB`;
  }

  return `${(bytes / 1024 ** 3).toFixed(2)} GB`;
}

function formatPorts(container: ContainerSummary) {
  if (!container.ports.length) {
    return "—";
  }

  return container.ports
    .slice(0, 2)
    .map((port) =>
      port.host_port
        ? `${port.host_port}:${port.container_port}/${port.protocol}`
        : `${port.container_port}/${port.protocol}`,
    )
    .join(", ");
}

export function ContainerRow({
  container,
  checked,
  selected,
  onToggleChecked,
  onSelect,
  onActionError,
}: {
  container: ContainerSummary;
  checked: boolean;
  selected: boolean;
  onToggleChecked: () => void;
  onSelect: () => void;
  onActionError: (message: string) => void;
}) {
  const startMutation = useStartContainer();
  const stopMutation = useStopContainer();
  const unpauseMutation = useUnpauseContainer();
  const removeMutation = useRemoveContainer();

  const architecture = container.platform.architecture?.toUpperCase();
  const cpuValue =
    container.stats?.cpu_percent === undefined || container.stats?.cpu_percent === null
      ? "N/A"
      : container.stats.cpu_percent.toFixed(1);
  const memoryValue = formatMemory(container.stats?.memory_usage_bytes);

  const handlePrimaryAction = async (event: React.MouseEvent<HTMLButtonElement>) => {
    event.stopPropagation();
    onActionError("");

    try {
      if (container.actions.can_start) {
        await startMutation.mutateAsync(container.id);
        return;
      }

      if (container.actions.can_unpause) {
        await unpauseMutation.mutateAsync(container.id);
        return;
      }

      if (container.actions.can_stop) {
        await stopMutation.mutateAsync(container.id);
      }
    } catch (error) {
      onActionError(
        getErrorMessage(
          error,
          `Failed to update container ${container.name}. Check the Docker daemon and the container configuration.`,
        ),
      );
    }
  };

  const handleRemove = async (event: React.MouseEvent<HTMLButtonElement>) => {
    event.stopPropagation();

    if (window.confirm(`Remove container "${container.name}"? This cannot be undone.`)) {
      onActionError("");

      try {
        await removeMutation.mutateAsync(container.id);
      } catch (error) {
        onActionError(
          getErrorMessage(
            error,
            `Failed to remove container ${container.name}. Check the Docker daemon and try again.`,
          ),
        );
      }
    }
  };

  const primaryActionLabel = container.actions.can_start
    ? "Start container"
    : container.actions.can_unpause
      ? "Unpause container"
      : "Stop container";

  const primaryActionIcon =
    container.actions.can_start || container.actions.can_unpause ? Play : Square;

  return (
    <tr
      className={cn(
        "cursor-pointer border-b border-[rgb(var(--workspace-border))] transition",
        selected ? "bg-[rgb(var(--table-row-selected))]" : "hover:bg-[rgb(var(--table-row-hover))]",
      )}
      onClick={onSelect}
    >
      <td className="w-[56px] px-5 py-4" onClick={(event) => event.stopPropagation()}>
        <button
          type="button"
          onClick={onToggleChecked}
          className={cn(
            "flex h-7 w-7 items-center justify-center rounded-[10px] border transition",
            checked
              ? "border-[rgb(var(--link))] bg-[rgb(var(--link))] text-white"
              : "border-[rgb(var(--workspace-border))] bg-transparent text-transparent hover:border-[rgb(var(--link))]",
          )}
          aria-label={`Select ${container.name}`}
        >
          <span className="text-[11px] font-bold">✓</span>
        </button>
      </td>

      <td className="w-[48px] px-2">
        <div
          className={cn(
            "mx-auto h-3.5 w-3.5 rounded-full border",
            container.state === "running"
              ? "border-emerald-300/70 bg-emerald-400 shadow-[0_0_18px_rgba(57,214,153,0.35)]"
              : container.state === "paused"
                ? "border-amber-300/70 bg-amber-400"
                : "border-white/20 bg-transparent",
          )}
        />
      </td>

      <td className="min-w-[320px] px-4 py-4">
        <div className="flex items-center gap-3">
          <span className="truncate text-[18px] font-medium text-[rgb(var(--workspace-foreground))]">
            {container.name}
          </span>
          {architecture ? (
            <span className="rounded-full bg-[rgb(var(--warning-bg))] px-3 py-1 text-[12px] font-bold uppercase tracking-[0.08em] text-[rgb(var(--warning-fg))]">
              {architecture}
            </span>
          ) : null}
        </div>
      </td>

      <td className="w-[220px] px-4 py-4 font-mono text-[17px] text-[rgb(var(--workspace-soft))]">
        {container.short_id}
      </td>

      <td className="w-[260px] px-4 py-4">
        <span className="truncate text-[17px] text-[rgb(var(--link))]">{container.image}</span>
      </td>

      <td className="w-[220px] px-4 py-4 font-mono text-[15px] text-[rgb(var(--workspace-soft))]">
        {formatPorts(container)}
      </td>

      <td className="w-[120px] px-4 py-4 text-[17px] text-[rgb(var(--workspace-soft))]">
        {cpuValue}
      </td>

      <td className="w-[190px] px-4 py-4 text-[17px] text-[rgb(var(--workspace-soft))]">
        {memoryValue}
      </td>

      <td className="w-[170px] px-4 py-4" onClick={(event) => event.stopPropagation()}>
        <div className="flex items-center justify-end gap-2">
          <button
            type="button"
            onClick={handlePrimaryAction}
            disabled={
              startMutation.isPending || stopMutation.isPending || unpauseMutation.isPending
            }
            className="flex h-11 w-11 items-center justify-center rounded-[14px] border border-[rgb(var(--link))/0.28] bg-[rgb(var(--workspace-card))] text-[rgb(var(--link))] transition hover:bg-[rgb(var(--link))/0.12] disabled:opacity-50"
            title={primaryActionLabel}
          >
            {primaryActionIcon === Play ? (
              <Play className="h-5 w-5 fill-current" />
            ) : (
              <Square className="h-4 w-4 fill-current" />
            )}
          </button>

          <button
            type="button"
            className="flex h-11 w-11 items-center justify-center rounded-[14px] border border-[rgb(var(--workspace-border))] bg-[rgb(var(--workspace-card))] text-[rgb(var(--workspace-soft))] transition hover:text-[rgb(var(--workspace-foreground))]"
            title="More actions coming next"
          >
            <MoreVertical className="h-5 w-5" />
          </button>

          <button
            type="button"
            onClick={handleRemove}
            disabled={removeMutation.isPending || !container.actions.can_remove}
            className="flex h-11 w-11 items-center justify-center rounded-[14px] border border-[rgb(var(--danger))/0.24] bg-[rgb(var(--workspace-card))] text-[rgb(var(--danger))] transition hover:bg-[rgb(var(--danger))/0.12] disabled:opacity-50"
            title="Remove container"
          >
            <Trash2 className="h-5 w-5" />
          </button>
        </div>
      </td>
    </tr>
  );
}
