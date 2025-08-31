'use client';
import { formSchema, FormSchema } from '@/app/app/(orgs)/orgs/[org_slug]/projects/new/page';
import { SettingsSection } from '@components/settings/section';
import { Setting, SettingLikeHeader } from '@components/settings/setting';
import { SettingAction } from '@components/settings/setting-action';
import { Alert, AlertDescription, AlertTitle } from '@components/ui/alert';
import { Button } from '@components/ui/button';
import { Form } from '@components/ui/form';
import { Input } from '@components/ui/input';
import { zodResolver } from '@hookform/resolvers/zod';
import { DISPLAY_NAME, REPO_URL, WEBHOOKS_SUPPORTED } from '@lib/constants';
import { useOrg, useProject } from '@lib/hooks';
import { useModals } from '@lib/modals/ModalsManager';
import { useSetTokenMutation, useProjectMutation, useWebhookSecretMutation } from '@lib/mutations';
import { useClipboard } from '@mantine/hooks';
import {
    IconBrandGit,
    IconBrandGithub,
    IconCheck,
    IconCopy,
    IconExclamationCircle,
    IconGitBranch,
    IconKey,
    IconLink,
    IconPencil,
    IconRefresh,
    IconServer,
    IconWebhook,
} from '@tabler/icons-react';
import { useCallback } from 'react';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';

export default function Page() {
    const { thisOrgSlug } = useOrg();
    const { thisProject, thisProjectSlug } = useProject();
    const modals = useModals();
    const webhookClipboard = useClipboard({ timeout: 3000 });

    const repositoryForm = useForm<FormSchema>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: '',
            slug: '',
            repository: {
                url: '',
            },
        },
        values: thisProject && {
            name: thisProject.name,
            slug: thisProject.slug,
            repository: thisProject.repository,
        },
    });

    const { mutate } = useProjectMutation({});
    const tokenMutation = useSetTokenMutation({});
    const webhookSecret = useWebhookSecretMutation({});

    const repoProvider = repositoryForm.watch('repository.source');
    const webhookUrl =
        repoProvider && thisProject?._id
            ? `${typeof window !== 'undefined' ? window.location.origin : ''}/api/webhooks/${thisProject?._id}/${repoProvider}`
            : '';

    const generateWebhookSecret = useCallback(async () => {
        const regenerating = thisProject?.repository.webhook_secret_generated;

        if (regenerating) {
            const confirmed = await modals.show('Confirm', {
                title: 'Regenerate Webhook Secret',
                description:
                    'Are you sure you want to regenerate the webhook secret? All existing webhooks will stop working until you update them with the new secret.',
                confirmText: 'Regenerate',
            });
            if (!confirmed) return;
        }

        webhookSecret.mutate({
            params: {
                path: {
                    org_slug: thisOrgSlug,
                    project_slug: thisProjectSlug,
                },
            },
        });
    }, [thisProject?.repository.webhook_secret_generated, thisOrgSlug, thisProjectSlug]);

    const setAccessTokenAsk = useCallback(async () => {
        const token = await modals.show('EnterToken', {
            isUpdating: thisProject?.repository.access_token_set || false,
        });

        if (!token) return;

        tokenMutation.mutate({
            params: {
                path: {
                    org_slug: thisOrgSlug,
                    project_slug: thisProjectSlug,
                },
            },
            body: { access_token: token },
        });
    }, [thisProject?.repository.access_token_set, thisOrgSlug, thisProjectSlug]);

    return (
        <Form {...repositoryForm}>
            <form
                className="flex-1"
                onSubmit={repositoryForm.handleSubmit((v) => {
                    mutate({
                        params: {
                            path: { org_slug: thisOrgSlug, project_slug: thisProjectSlug },
                        },
                        body: v,
                    });
                })}
            >
                <SettingsSection title="Repository">
                    <Setting
                        title="Repository Provider"
                        formControl={repositoryForm.control}
                        name="repository.source"
                        type="select"
                        options={[
                            {
                                label: 'GitHub',
                                value: 'github',
                                description:
                                    'Automatic workflow runs, build statuses directly in GitHub',
                                icon: IconBrandGithub,
                            },
                            {
                                label: 'Forgejo / Gitea',
                                value: 'gitea',
                                description:
                                    'Automatic workflow runs, build statuses directly in your Forgejo or Gitea instance',
                                // TODO: Add Forgejo icon
                                icon: IconBrandGit,
                            },
                            {
                                label: 'Generic Git',
                                value: 'genericgit',
                                description:
                                    "Use any SSH Git repository. You'll need to run workflows manually form the CLI or admin panel.",
                                icon: IconBrandGit,
                            },
                        ]}
                        icon={IconServer}
                    />
                    <Setting
                        title="Repository URL"
                        formControl={repositoryForm.control}
                        name="repository.url"
                        type="text"
                        placeholder={`${REPO_URL}.git`}
                        description={
                            <>
                                You can provide a Git clone URL to a{' '}
                                {WEBHOOKS_SUPPORTED.slice(0, -1).join(', ')} or{' '}
                                {WEBHOOKS_SUPPORTED.slice(-1)} instance. If your provider is not
                                supported, feel free to{' '}
                                <a
                                    href={`${REPO_URL}/issues/new`}
                                    target="_blank"
                                    className="underline hover:text-foreground transition-all"
                                >
                                    open an issue
                                </a>{' '}
                                or a{' '}
                                <a
                                    href={`${REPO_URL}/pulls`}
                                    target="_blank"
                                    className="underline hover:text-foreground transition-all"
                                >
                                    pull request
                                </a>
                                .
                            </>
                        }
                        icon={IconLink}
                    />
                    <div>
                        <SettingLikeHeader title="Access Token" className="mt-4" icon={IconKey} />
                        <SettingAction
                            description={`Access Tokens allow you to grant ${DISPLAY_NAME} access to your repository. Access token is ${thisProject?.repository.access_token_set ? '' : 'not '} currently set.`}
                            className="mt-3"
                            rightSection={
                                <div className="flex gap-2">
                                    <Button
                                        variant="outline"
                                        type="button"
                                        onClick={setAccessTokenAsk}
                                    >
                                        <IconPencil />
                                        {thisProject?.repository.access_token_set
                                            ? 'Change Token'
                                            : 'Set Token'}
                                    </Button>
                                </div>
                            }
                        />
                    </div>
                    <Button className="mt-3" type="submit">
                        Save settings
                    </Button>
                </SettingsSection>
                {repoProvider !== 'genericgit' && (
                    <SettingsSection title="Webhooks">
                        <>
                            {thisProject?.repository.webhook_secret_generated === false && (
                                <Alert className="mt-4" variant="warning">
                                    <IconExclamationCircle />
                                    <AlertTitle>Finish setting up webhooks</AlertTitle>
                                    <AlertDescription>
                                        Webhooks won't work unless you generate a webhook secret and
                                        set up the webhook in your repository provider.
                                    </AlertDescription>
                                </Alert>
                            )}
                            <SettingLikeHeader
                                title="Webhook URL"
                                className="mt-4"
                                icon={IconWebhook}
                            />
                            <div className="flex gap-2">
                                <Input readOnly value={webhookUrl ?? ''} />
                                <Button
                                    variant="outline"
                                    size="icon"
                                    type="button"
                                    onClick={() => {
                                        webhookClipboard.copy(webhookUrl);
                                        toast.success('Webhook URL copied to clipboard');
                                    }}
                                >
                                    {webhookClipboard.copied ? <IconCheck /> : <IconCopy />}
                                </Button>
                            </div>
                            <SettingLikeHeader
                                title="Webhook Secret"
                                className="mt-4"
                                icon={IconKey}
                            />
                            <SettingAction
                                description="Webhook secrets are used to verify that the webhook payloads are sent by your repository provider and not tampered with."
                                className="mt-3"
                                rightSection={
                                    <div className="flex gap-2">
                                        <Button
                                            variant="outline"
                                            type="button"
                                            onClick={generateWebhookSecret}
                                        >
                                            <IconRefresh />
                                            {thisProject?.repository.webhook_secret_generated
                                                ? 'Regenerate'
                                                : 'Generate'}
                                        </Button>
                                    </div>
                                }
                            />
                        </>
                    </SettingsSection>
                )}
            </form>
        </Form>
    );
}
