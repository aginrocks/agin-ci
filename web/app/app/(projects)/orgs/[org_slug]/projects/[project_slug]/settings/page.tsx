'use client';
import { formSchema, FormSchema } from '@/app/app/(orgs)/orgs/[org_slug]/projects/new/page';
import { SettingsSection } from '@components/settings/section';
import { Setting } from '@components/settings/setting';
import { SettingAction } from '@components/settings/setting-action';
import { Button } from '@components/ui/button';
import { Form } from '@components/ui/form';
import { zodResolver } from '@hookform/resolvers/zod';
import { useOrg, useProject } from '@lib/hooks';
import { useModals } from '@lib/modals/ModalsManager';
import { $api } from '@lib/providers/api';
import { IconLink, IconPencil } from '@tabler/icons-react';
import { useQueryClient } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { useCallback, useRef } from 'react';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';

export default function Page() {
    const { thisOrgSlug } = useOrg();
    const { thisProject, thisProjectSlug } = useProject();
    const modals = useModals();

    const modifyingSlug = useRef<boolean>(false);
    const newSlug = useRef<string | null>(null);

    const router = useRouter();
    const queryClient = useQueryClient();

    const generalForm = useForm<FormSchema>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: '',
            slug: '',
        },
        values: thisProject && {
            name: thisProject.name,
            slug: thisProject.slug,
            repository: thisProject.repository,
        },
    });

    const { mutate } = $api.useMutation(
        'patch',
        '/api/organizations/{org_slug}/projects/{project_slug}',
        {
            onSuccess: () => {
                toast.success('Project settings updated successfully');
                queryClient.invalidateQueries({
                    queryKey: [
                        'get',
                        '/api/organizations/{org_slug}/projects',
                        {
                            params: {
                                path: {
                                    org_slug: thisOrgSlug,
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
                                    org_slug: thisOrgSlug,
                                    project_slug: thisProjectSlug,
                                },
                            },
                        },
                    ],
                });
                if (modifyingSlug.current && newSlug.current) {
                    router.push(`/app/orgs/${thisOrgSlug}/projects/${newSlug.current}/settings`);
                }
            },
            onError: (error) => {
                toast.error('Failed to update project settings', {
                    description: error.error,
                });
            },
        }
    );

    const deleteProject = $api.useMutation(
        'delete',
        '/api/organizations/{org_slug}/projects/{project_slug}',
        {
            onSuccess: () => {
                toast.success('Project deleted successfully');
                queryClient.invalidateQueries({
                    queryKey: [
                        'get',
                        '/api/organizations/{org_slug}/projects',
                        {
                            params: {
                                path: {
                                    org_slug: thisOrgSlug,
                                },
                            },
                        },
                    ],
                });
                router.push(`/app/orgs/${thisOrgSlug}/projects`);
            },
            onError: (error) => {
                toast.error('Failed to delete project', {
                    description: error.error,
                });
            },
        }
    );

    const onDelete = useCallback(async () => {
        const confirmed = await modals.show('ConfirmDeletion', {
            title: 'Delete Organization',
            description: `Are you sure you want to delete the project "${thisProject?.name}"? This action cannot be undone.`,
            objectName: `${thisOrgSlug}/${thisProjectSlug}`,
        });

        if (!confirmed) return;

        deleteProject.mutate({
            params: {
                path: {
                    org_slug: thisOrgSlug,
                    project_slug: thisProjectSlug,
                },
            },
        });
    }, [thisProject?.name, thisOrgSlug, thisProjectSlug]);

    return (
        <>
            <Form {...generalForm}>
                <form
                    className="flex-1"
                    onSubmit={generalForm.handleSubmit((v) => {
                        modifyingSlug.current = v.slug !== thisProjectSlug;
                        newSlug.current = v.slug;

                        mutate({
                            params: {
                                path: { org_slug: thisOrgSlug, project_slug: thisProjectSlug },
                            },
                            body: v,
                        });
                    })}
                >
                    <SettingsSection title="General">
                        <Setting
                            title="Name"
                            formControl={generalForm.control}
                            name="name"
                            placeholder="Acme Inc."
                            icon={IconPencil}
                        />
                        <Setting
                            title="Slug"
                            formControl={generalForm.control}
                            name="slug"
                            placeholder="acme-inc"
                            icon={IconLink}
                        />
                        <Button className="mt-3" type="submit">
                            Save settings
                        </Button>
                    </SettingsSection>
                </form>
                <SettingsSection title="Danger Zone">
                    <SettingAction
                        title="Delete Project"
                        description="This action cannot be undone."
                        rightSection={
                            <Button variant="destructive" onClick={onDelete}>
                                Delete Project
                            </Button>
                        }
                    />
                </SettingsSection>
            </Form>
        </>
    );
}
