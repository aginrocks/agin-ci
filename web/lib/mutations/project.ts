import { paths } from '@/types/api';
import { $api } from '@lib/providers/api';
import { useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { AdditionalParams } from '.';

export function useProjectMutation(params: AdditionalParams) {
    const queryClient = useQueryClient();

    const mutation = $api.useMutation(
        'patch',
        '/api/organizations/{org_slug}/projects/{project_slug}',
        {
            onSuccess: (data, options) => {
                toast.success('Project settings updated successfully');
                queryClient.invalidateQueries({
                    queryKey: [
                        'get',
                        '/api/organizations/{org_slug}/projects',
                        {
                            params: {
                                path: {
                                    org_slug: options.params.path.org_slug,
                                },
                            },
                        },
                    ],
                });
                queryClient.invalidateQueries({
                    queryKey: [
                        'get',
                        '/api/organizations/{org_slug}/projects/{project_slug}',
                        {
                            params: {
                                path: {
                                    org_slug: options.params.path.org_slug,
                                    project_slug: options.params.path.project_slug,
                                },
                            },
                        },
                    ],
                });
                params?.onSuccess?.();
            },
            onError: (error) => {
                toast.error('Failed to update project settings', {
                    description: error.error,
                });
                params?.onError?.();
            },
        }
    );

    return mutation;
}
