import { paths } from '@/types/api';
import { $api } from '@lib/providers/api';
import { QueryClient, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { AdditionalParams } from '.';

export function invalidateProject(
    queryClient: QueryClient,
    org_slug: string,
    project_slug: string
) {
    queryClient.invalidateQueries({
        queryKey: [
            'get',
            '/api/organizations/{org_slug}/projects',
            {
                params: {
                    path: {
                        org_slug,
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
                        org_slug,
                        project_slug,
                    },
                },
            },
        ],
    });
}

export function useProjectMutation(params: AdditionalParams) {
    const queryClient = useQueryClient();

    const mutation = $api.useMutation(
        'patch',
        '/api/organizations/{org_slug}/projects/{project_slug}',
        {
            onSuccess: (data, options) => {
                toast.success('Project settings updated successfully');
                invalidateProject(
                    queryClient,
                    options.params.path.org_slug,
                    options.params.path.project_slug
                );
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

export function useProjectKeysMutation(params: AdditionalParams) {
    const queryClient = useQueryClient();

    const mutation = $api.useMutation(
        'get',
        '/api/organizations/{org_slug}/projects/{project_slug}/regenerate-keys',
        {
            onSuccess: (data, options) => {
                toast.success('Deploy Key generated successfully');
                invalidateProject(
                    queryClient,
                    options.params.path.org_slug,
                    options.params.path.project_slug
                );
                params?.onSuccess?.();
            },
            onError: (error) => {
                toast.error('Failed to generate the Deploy Key', {
                    description: error.error,
                });
                params?.onError?.();
            },
        }
    );

    return mutation;
}
