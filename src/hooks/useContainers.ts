import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { docker } from "@/lib/tauri";
import type {
  BulkContainerAction,
  ContainerListQuery,
  ContainerListResponse,
} from "@/types/docker";

const CONTAINERS_KEY = ["containers"] as const;
const DEFAULT_LIST_RESPONSE: ContainerListResponse = {
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

function normalizeListQuery(query: ContainerListQuery = {}): Required<ContainerListQuery> {
  return {
    all: query.all ?? true,
    only_running: query.only_running ?? false,
    search: query.search ?? null,
    limit: query.limit ?? null,
  };
}

export function useContainersResponse(query: ContainerListQuery = {}) {
  const normalizedQuery = normalizeListQuery(query);

  return useQuery({
    queryKey: [...CONTAINERS_KEY, "list", normalizedQuery],
    queryFn: () => docker.listContainersResponse(normalizedQuery),
    placeholderData: DEFAULT_LIST_RESPONSE,
    refetchInterval: 5_000,
    staleTime: 2_000,
  });
}

export function useContainers(query: ContainerListQuery = {}) {
  return useQuery({
    queryKey: [...CONTAINERS_KEY, "items", normalizeListQuery(query)],
    queryFn: async () => (await docker.listContainersResponse(normalizeListQuery(query))).items,
    placeholderData: [],
    refetchInterval: 5_000,
    staleTime: 2_000,
  });
}

export function useContainersOverview() {
  return useQuery({
    queryKey: [...CONTAINERS_KEY, "overview"],
    queryFn: docker.getContainersOverview,
    staleTime: 2_000,
    refetchInterval: 5_000,
  });
}

export function useContainerDetail(id: string | null) {
  return useQuery({
    queryKey: [...CONTAINERS_KEY, "detail", id],
    queryFn: () => docker.getContainerDetail(id ?? ""),
    enabled: Boolean(id),
    staleTime: 2_000,
  });
}

function useContainerMutation<TVariables>(
  mutationFn: (variables: TVariables) => Promise<void>,
) {
  const client = useQueryClient();

  return useMutation({
    mutationFn,
    onSuccess: () => client.invalidateQueries({ queryKey: CONTAINERS_KEY }),
  });
}

export function useStartContainer() {
  return useContainerMutation(docker.startContainer);
}

export function useStopContainer() {
  return useContainerMutation(docker.stopContainer);
}

export function useRestartContainer() {
  return useContainerMutation(docker.restartContainer);
}

export function usePauseContainer() {
  return useContainerMutation(docker.pauseContainer);
}

export function useUnpauseContainer() {
  return useContainerMutation(docker.unpauseContainer);
}

export function useRemoveContainer() {
  return useContainerMutation((id: string) => docker.removeContainer(id, true, false));
}

export function useApplyContainerAction() {
  const client = useQueryClient();

  return useMutation({
    mutationFn: ({ ids, action }: { ids: string[]; action: BulkContainerAction }) =>
      docker.applyContainerAction(ids, action),
    onSuccess: () => client.invalidateQueries({ queryKey: CONTAINERS_KEY }),
  });
}
