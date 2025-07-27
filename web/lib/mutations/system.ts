import { useQuery, useQueryClient } from '@tanstack/react-query';
import { AdditionalParams } from '.';
import { $api } from '@lib/providers/api';
import { toast } from 'sonner';

export function useChangeSystemRoleMutation(params: AdditionalParams) {
    const queryClient = useQueryClient();

    const currentUser = useQuery($api.queryOptions('get', '/api/user'));

    const mutation = $api.useMutation('patch', '/api/system/users/{user_id}', {
        onSuccess: (data, options) => {
            toast.success('User updated successfully');
            if (options.params.path.user_id === currentUser.data?._id) {
                queryClient.invalidateQueries({
                    queryKey: ['get', '/api/user'],
                });
                queryClient.invalidateQueries({
                    queryKey: ['get', '/api/god'],
                });
                queryClient.invalidateQueries({
                    queryKey: ['get', '/api/organizations'],
                });
            }
            queryClient.invalidateQueries({
                queryKey: ['get', '/api/system/users'],
            });
            params?.onSuccess?.();
        },
        onError: (error) => {
            toast.error('Failed to update user', {
                description: error.error,
            });
            params?.onError?.();
        },
    });

    return mutation;
}
