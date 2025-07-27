import { useQueryClient } from '@tanstack/react-query';
import { AdditionalParams } from '.';
import { $api } from '@lib/providers/api';
import { toast } from 'sonner';

export function useCreateRunnerMutation(params: AdditionalParams) {
    const queryClient = useQueryClient();

    const mutation = $api.useMutation('post', '/api/system/runners', {
        onSuccess: (data, options) => {
            toast.success('Runner created successfully');
            queryClient.invalidateQueries({
                queryKey: ['get', '/api/system/runners'],
            });
            params?.onSuccess?.();
        },
        onError: (error, options) => {
            toast.error('Failed to create runner', {
                description: error.error,
            });
            params?.onError?.();
        },
    });

    return mutation;
}

export function useEditRunnerMutation(params: AdditionalParams) {
    const queryClient = useQueryClient();

    const mutation = $api.useMutation('patch', '/api/system/runners/{runner_id}', {
        onSuccess: (data, options) => {
            queryClient.invalidateQueries({
                queryKey: ['get', '/api/system/runners'],
            });
            params?.onSuccess?.();
        },
        onError: (error, options) => {
            toast.error('Failed to edit runner', {
                description: error.error,
            });
            params?.onError?.();
        },
    });

    return mutation;
}
