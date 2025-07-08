import { $api } from '@lib/providers/api';
import { AdditionalParams, invalidateProject } from '.';
import { toast } from 'sonner';
import { useModals } from '@lib/modals/ModalsManager';
import { useQueryClient } from '@tanstack/react-query';

export function useWebhookSecretMutation(params: AdditionalParams) {
    const modals = useModals();
    const queryClient = useQueryClient();

    const mutation = $api.useMutation(
        'get',
        '/api/organizations/{org_slug}/projects/{project_slug}/regenerate-webhook-secret',
        {
            onSuccess: (data, options) => {
                modals.show('OneTimeSecret', {
                    title: 'Webhook Secret',
                    description:
                        'This is your new webhook secret. Please copy it now, as it will not be shown again.',
                    secret: data.webhook_secret,
                });
                invalidateProject(
                    queryClient,
                    options.params.path.org_slug,
                    options.params.path.project_slug
                );
                params?.onSuccess?.();
            },
            onError: (error) => {
                toast.error('Failed to generate webhook secrets', {
                    description: error.error,
                });
                params?.onError?.();
            },
        }
    );

    return mutation;
}
