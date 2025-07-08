'use client';
import { paths } from '@/types/api';
import { PageHeader } from '@components/page-header';
import { Wizard } from '@components/wizards/wizard';
import { WizardPage } from '@components/wizards/wizard-page';
import { IconCheck, IconCube, IconGitMerge, IconLink } from '@tabler/icons-react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@components/ui/form';
import { Input } from '@components/ui/input';
import { Button } from '@components/ui/button';
import { $api } from '@lib/providers/api';
import { useRouter } from 'next/navigation';
import { useEffect } from 'react';
import { slugify } from '@lib/utils';
import { toast } from 'sonner';
import { useOrg } from '@lib/hooks';
import { REPO_URL, WEBHOOKS_SUPPORTED } from '@lib/constants';

const RESERVED_SLUGS = ['new', 'edit', 'delete', 'create'];

const isValidSlug = (s: string): boolean => {
    return s
        .split('')
        .every((c) => (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c === '-' || c === '_');
};

export const formSchema = z.object({
    name: z.string().min(1, 'Name is required').max(32, 'Name must be at most 32 characters long'),
    slug: z
        .string()
        .min(1, 'Slug is required')
        .max(32, 'Slug must be at most 32 characters long')
        .refine(
            isValidSlug,
            'Slug can only contain lowercase letters, numbers, hyphens, and underscores'
        )
        .refine((s) => !RESERVED_SLUGS.includes(s), 'This slug is reserved and cannot be used'),
    repository: z.object({
        url: z.string(),
        source: z.enum(['github', 'gitea', 'genericgit']),
    }),
}) satisfies z.ZodType<
    paths['/api/organizations/{org_slug}/projects']['post']['requestBody']['content']['application/json']
> satisfies z.ZodType<
    paths['/api/organizations/{org_slug}/projects/{project_slug}']['patch']['requestBody']['content']['application/json']
>;

export type FormSchema = z.infer<typeof formSchema>;

export default function Page() {
    const router = useRouter();
    const { thisOrg } = useOrg();
    const form = useForm<FormSchema>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: '',
            slug: '',
            repository: {
                url: '',
                source: 'genericgit',
            },
        },
    });

    const name = form.watch('name');
    useEffect(() => {
        if ('slug' in form.formState.dirtyFields) return;

        form.setValue('slug', slugify(name), {
            shouldDirty: false,
            shouldTouch: false,
        });
    }, [name]);

    const create = $api.useMutation('post', '/api/organizations/{org_slug}/projects', {
        onSuccess: () => {
            router.replace(`/app/orgs/${thisOrg?.slug}/projects/${form.getValues('slug')}`);
        },
        onError: (error) => {
            toast.error('Failed to create project', {
                description: error.error,
            });
        },
    });

    return (
        <>
            <PageHeader
                path={[
                    {
                        label: thisOrg?.name,
                        href: `/app/orgs/${thisOrg?.slug}`,
                    },
                    {
                        label: 'New Project',
                    },
                ]}
            />
            <Form {...form}>
                <Wizard>
                    <WizardPage
                        pageNumber={0}
                        icon={IconCube}
                        title="Create Project"
                        description="Project corresponds to a single repository and allows you to group multiple workflows together."
                        beforeNext={async () => await form.trigger('name')}
                    >
                        <FormField
                            control={form.control}
                            name="name"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Name</FormLabel>
                                    <FormControl>
                                        <Input placeholder="Example Project" {...field} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                    </WizardPage>
                    <WizardPage
                        pageNumber={1}
                        icon={IconLink}
                        title="Choose a Slug"
                        description="The slug will be visible in the URL and can be used in the API."
                        beforeNext={async () => await form.trigger('slug')}
                    >
                        <FormField
                            control={form.control}
                            name="slug"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Slug</FormLabel>
                                    <FormControl>
                                        <Input placeholder="example-project" {...field} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                    </WizardPage>
                    <WizardPage
                        pageNumber={2}
                        icon={IconGitMerge}
                        title="Connect Repository"
                        description="Connect your project to a Git repository. You will be able to set up webhooks for automated workflow runs later."
                        swapNextButton={
                            <Button
                                disabled={create.isPending}
                                onClick={() =>
                                    form.handleSubmit(() => {
                                        const values = form.getValues();
                                        let sourceHostname = '';
                                        try {
                                            sourceHostname = new URL(values.repository.source)
                                                ?.hostname;
                                        } catch {}

                                        create.mutate({
                                            body: {
                                                ...values,
                                                repository: {
                                                    ...values.repository,
                                                    source:
                                                        sourceHostname === 'github.com'
                                                            ? 'github'
                                                            : sourceHostname === 'codeberg.org'
                                                              ? 'gitea'
                                                              : 'genericgit',
                                                },
                                            },
                                            params: {
                                                path: {
                                                    org_slug: thisOrg?.slug!,
                                                },
                                            },
                                        });
                                    })()
                                }
                            >
                                <IconCheck />
                                Done
                            </Button>
                        }
                    >
                        <FormField
                            control={form.control}
                            name="repository.url"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Repository URL</FormLabel>
                                    <FormControl>
                                        <Input placeholder={`${REPO_URL}.git`} {...field} />
                                    </FormControl>
                                    <FormDescription>
                                        You can provide any valid Git clone URL, however we
                                        recommend using {WEBHOOKS_SUPPORTED.slice(0, -1).join(', ')}{' '}
                                        or {WEBHOOKS_SUPPORTED.slice(-1)} for automated workflow
                                        runs. If your provider is not supported, feel free to{' '}
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
                                    </FormDescription>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                    </WizardPage>
                </Wizard>
            </Form>
        </>
    );
}
