'use client';
import { formSchema, FormSchema } from '@/app/app/(orgs)/orgs/[org_slug]/projects/new/page';
import { SettingsSection } from '@components/settings/section';
import { Setting, SettingLikeHeader } from '@components/settings/setting';
import { Button } from '@components/ui/button';
import { Form } from '@components/ui/form';
import { zodResolver } from '@hookform/resolvers/zod';
import { REPO_URL, WEBHOOKS_SUPPORTED } from '@lib/constants';
import { useOrg, useProject } from '@lib/hooks';
import { useModals } from '@lib/modals/ModalsManager';
import { useProjectKeysMutation, useProjectMutation } from '@lib/mutations';
import {
    IconBrandGit,
    IconBrandGithub,
    IconCopy,
    IconGitBranch,
    IconKey,
    IconLink,
    IconRefresh,
    IconServer,
} from '@tabler/icons-react';
import { useCallback } from 'react';
import { useForm } from 'react-hook-form';

export default function Page() {
    const { thisOrgSlug } = useOrg();
    const { thisProject, thisProjectSlug } = useProject();
    const modals = useModals();

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
                                label: 'Forgejo',
                                value: 'forgejo',
                                description:
                                    'Automatic workflow runs, build statuses directly in your Forgejo instance',
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
                        <div className="flex gap-2 mt-1">
                            {thisProject?.repository.deploy_key_generated && (
                                <Button variant="outline" type="button">
                                    <IconCopy />
                                    Copy
                                </Button>
                            )}
                            <Button variant="outline" type="button" onClick={generateDeployKey}>
                                <IconRefresh />
                                {thisProject?.repository.deploy_key_generated
                                    ? 'Regenerate'
                                    : 'Generate'}
                            </Button>
                        </div>
                    </div>
                    <Button className="mt-3" type="submit">
                        Save settings
                    </Button>
                </SettingsSection>
            </form>
        </Form>
    );
}
