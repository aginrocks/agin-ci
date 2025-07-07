'use client';
import { formSchema, FormSchema } from '@/app/app/(orgs)/orgs/[org_slug]/projects/new/page';
import { SettingsSection } from '@components/settings/section';
import { Setting } from '@components/settings/setting';
import { Button } from '@components/ui/button';
import { Form } from '@components/ui/form';
import { zodResolver } from '@hookform/resolvers/zod';
import { REPO_URL, WEBHOOKS_SUPPORTED } from '@lib/constants';
import { useOrg, useProject } from '@lib/hooks';
import { useProjectMutation } from '@lib/mutations';
import {
    IconBrandGit,
    IconBrandGithub,
    IconGitBranch,
    IconLink,
    IconServer,
} from '@tabler/icons-react';
import { useForm } from 'react-hook-form';

export default function Page() {
    const { thisOrgSlug } = useOrg();
    const { thisProject, thisProjectSlug } = useProject();

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
                    <Button className="mt-3" type="submit">
                        Save settings
                    </Button>
                </SettingsSection>
            </form>
        </Form>
    );
}
