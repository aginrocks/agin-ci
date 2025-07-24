import { useQueryClient } from '@tanstack/react-query';
import { AdditionalParams } from '.';
import { $api } from '@lib/providers/api';
import { toast } from 'sonner';

export function useChangeSystemRoleMutation(params: AdditionalParams) {
    const queryClient = useQueryClient();

    const mutation = $api.useMutation('patch', '/api/system/users/{user_id}', {
        onSuccess: (data, options) => {
            toast.success('Project settings updated successfully');
            queryClient.invalidateQueries({
                queryKey: ['get', '/api/system/users'],
            });
            queryClient.invalidateQueries({
                queryKey: ['get', '/api/user'],
            });
            params?.onSuccess?.();
        },
        onError: (error) => {
            toast.error('Failed to update project settings', {
                description: error.error,
            });
            params?.onError?.();
        },
    });

    return mutation;
}
