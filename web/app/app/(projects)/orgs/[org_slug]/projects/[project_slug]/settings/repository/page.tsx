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
import { REPO_URL, WEBHOOKS_SUPPORTED } from '@lib/constants';
import { useOrg, useProject } from '@lib/hooks';
import { useModals } from '@lib/modals/ModalsManager';
import { useProjectKeysMutation, useProjectMutation } from '@lib/mutations';
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
    const deployKeyClipboard = useClipboard({ timeout: 3000 });
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
    const keys = useProjectKeysMutation({});

    const repoProvider = repositoryForm.watch('repository.source');
    const webhookUrl = repoProvider
        ? `${typeof window !== 'undefined' ? window.location.origin : ''}/api/webhook-handler/${repoProvider}`
        : '';

    const generateDeployKey = useCallback(async () => {
        const regenerating = thisProject?.repository.deploy_key_generated;

        const confirmed = await modals.show('Confirm', {
            title: `${regenerating ? 'Reg' : 'G'}enerate Deploy Key`,
            description: regenerating
                ? "Are you sure you want to generate a new deploy key? This will replace the existing one. You can't undo this action."
                : "Deploy Keys allow you to grant Agin CI read access to your repository. You don't need to generate a deploy key if your repository is public.",
            confirmText: regenerating ? 'Regenerate' : 'Generate',
        });
        if (!confirmed) return;

        keys.mutate({
            params: {
                path: {
                    org_slug: thisOrgSlug,
                    project_slug: thisProjectSlug,
                },
            },
        });
    }, [thisProject?.repository.deploy_key_generated, thisOrgSlug, thisProjectSlug]);

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
                                You can provide any valid Git clone URL, however we recommend using{' '}
                                {WEBHOOKS_SUPPORTED.slice(0, -1).join(', ')} or{' '}
                                {WEBHOOKS_SUPPORTED.slice(-1)} for automated workflow runs. If your
                                provider is not supported, feel free to{' '}
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
                        <SettingLikeHeader title="Deploy Key" className="mt-4" icon={IconKey} />
                        <SettingAction
                            description="Deploy Keys allow you to grant Agin CI read access to your repository."
                            className="mt-3"
                            rightSection={
                                <div className="flex gap-2">
                                    {thisProject?.repository.deploy_key_generated && (
                                        <Button
                                            variant="outline"
                                            type="button"
                                            onClick={() => {
                                                deployKeyClipboard.copy(
                                                    thisProject.repository.deploy_public_key
                                                );
                                                toast.success('Deploy key copied to clipboard');
                                            }}
                                        >
                                            {deployKeyClipboard.copied ? (
                                                <IconCheck />
                                            ) : (
                                                <IconCopy />
                                            )}
                                            Copy
                                        </Button>
                                    )}
                                    <Button
                                        variant="outline"
                                        type="button"
                                        onClick={generateDeployKey}
                                    >
                                        <IconRefresh />
                                        {thisProject?.repository.deploy_key_generated
                                            ? 'Regenerate'
                                            : 'Generate'}
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
                                            // onClick={generateDeployKey}
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
