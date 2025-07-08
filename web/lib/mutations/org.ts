import { QueryClient, useQueryClient } from '@tanstack/react-query';
import { AdditionalParams } from '.';
import { $api } from '@lib/providers/api';
import { toast } from 'sonner';

export function invalidateOrg(queryClient: QueryClient, slug: string) {
    queryClient.invalidateQueries({
        queryKey: ['get', '/api/organizations'],
    });
    queryClient.invalidateQueries({
        queryKey: [
            'get',
            '/api/organizations/{org_slug}',
            {
                params: {
                    path: {
                        org_slug: slug,
                    },
                },
            },
        ],
    });
}

export function useOrgMutation(params: AdditionalParams) {
    const queryClient = useQueryClient();

    const mutation = $api.useMutation('patch', '/api/organizations/{org_slug}', {
        onSuccess: (data, options) => {
            toast.success('Organization settings updated successfully');
            invalidateOrg(queryClient, options.params.path.org_slug);
            params?.onSuccess?.();
        },
        onError: (error) => {
            toast.error('Failed to update organization settings', {
                description: error.error,
            });
            params?.onError?.();
        },
    });

    return mutation;
}

export function useRemoveMemberMutation(params: AdditionalParams) {
    const queryClient = useQueryClient();

    const mutation = $api.useMutation(
        'delete',
        '/api/organizations/{org_slug}/members/{member_id}',
        {
            onSuccess: (data, options) => {
                toast.success('Member removed successfully');
                invalidateOrg(queryClient, options.params.path.org_slug);
                queryClient.invalidateQueries({
                    queryKey: [
                        'get',
                        '/api/organizations/{org_slug}/members',
                        {
                            params: {
                                path: {
                                    org_slug: options.params.path.org_slug,
                                },
                            },
                        },
                    ],
                });
                params?.onSuccess?.();
            },
            onError: (error) => {
                toast.error('Failed to remove member', {
                    description: error.error,
                });
                params?.onError?.();
            },
        }
    );

    return mutation;
}
