import { useQueryClient } from '@tanstack/react-query';
import { AdditionalParams } from '.';
import { $api } from '@lib/providers/api';
import { toast } from 'sonner';

export function useGodModeMutation(params: AdditionalParams) {
    const queryClient = useQueryClient();

    const mutation = $api.useMutation('patch', '/api/god', {
        onSuccess: (data, options) => {
            toast.success(`God Mode ${data.enabled ? 'enabled' : 'disabled'}`);
            queryClient.invalidateQueries({
                queryKey: ['get', '/api/god'],
            });
            queryClient.invalidateQueries({
                queryKey: ['get', '/api/organizations'],
            });
            params?.onSuccess?.();
        },
        onError: (error, options) => {
            toast.error(`Failed to ${options.body.enable ? 'enable' : 'disable'} God Mode`, {
                description: error.error,
            });
            params?.onError?.();
        },
    });

    return mutation;
}
