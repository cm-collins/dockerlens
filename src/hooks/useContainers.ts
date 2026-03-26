import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { docker } from "@/lib/tauri";

const QUERY_KEY = ["containers"] as const;

export function useContainers() {
    return useQuery({
        queryKey: QUERY_KEY,
        queryFn: docker.listContainers,
        refetchInterval: 5_000,
        staleTime: 2_000,
    });
}

export function useStartContainer() {
    const client = useQueryClient();
    return useMutation({
        mutationFn: docker.startContainer,
        onSuccess: () => client.invalidateQueries({ queryKey: QUERY_KEY }),
    });
}

export function useStopContainer() {
    const client = useQueryClient();
    return useMutation({
        mutationFn: docker.stopContainer,
        onSuccess: () => client.invalidateQueries({ queryKey: QUERY_KEY }),
    });
}

export function useRestartContainer() {
    const client = useQueryClient();
    return useMutation({
        mutationFn: docker.restartContainer,
        onSuccess: () => client.invalidateQueries({ queryKey: QUERY_KEY }),
    });
}

export function usePauseContainer() {
    const client = useQueryClient();
    return useMutation({
        mutationFn: docker.pauseContainer,
        onSuccess: () => client.invalidateQueries({ queryKey: QUERY_KEY }),
    });
}

export function useUnpauseContainer() {
    const client = useQueryClient();
    return useMutation({
        mutationFn: docker.unpauseContainer,
        onSuccess: () => client.invalidateQueries({ queryKey: QUERY_KEY }),
    });
}

export function useRemoveContainer() {
    const client = useQueryClient();
    return useMutation({
        mutationFn: (id: string) => docker.removeContainer(id, true, false),
        onSuccess: () => client.invalidateQueries({ queryKey: QUERY_KEY }),
    });
}