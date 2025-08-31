'use client';
import { formSchema, FormSchema } from '@/app/app/(global)/orgs/new/schema';
import { PageHeader } from '@components/page-header';
import { SettingsSection } from '@components/settings/section';
import { Setting } from '@components/settings/setting';
import { SettingAction } from '@components/settings/setting-action';
import { Button } from '@components/ui/button';
import { Form } from '@components/ui/form';
import { zodResolver } from '@hookform/resolvers/zod';
import { useOrg } from '@lib/hooks';
import { useModals } from '@lib/modals/ModalsManager';
import { useOrgMutation } from '@lib/mutations';
import { $api } from '@lib/providers/api';
import { IconLink, IconMail, IconPencil } from '@tabler/icons-react';
import { useQueryClient } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { useCallback, useRef } from 'react';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';

export default function Page() {
    const { thisOrg, thisOrgSlug } = useOrg();
    const router = useRouter();
    const queryClient = useQueryClient();
    const modals = useModals();

    const generalForm = useForm<FormSchema>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: '',
            description: '',
            slug: '',
            avatar_email: '',
        },
        values: thisOrg && {
            name: thisOrg.name,
            description: thisOrg.description,
            slug: thisOrg.slug,
            avatar_email: thisOrg.avatar_email || '',
        },
    });

    const modifyingSlug = useRef<boolean>(false);
    const newSlug = useRef<string | null>(null);

    const { mutate } = useOrgMutation({
        onSuccess: () => {
            if (modifyingSlug.current && newSlug.current) {
                router.push(`/app/orgs/${newSlug.current}/settings`);
            }
        },
    });

    const deleteOrg = $api.useMutation('delete', '/api/organizations/{org_slug}', {
        onSuccess: () => {
            toast.success('Organization deleted successfully');
            queryClient.invalidateQueries({
                queryKey: ['get', '/api/organizations'],
            });
            router.push('/app/orgs');
        },
        onError: (error) => {
            toast.error('Failed to delete organization', {
                description: error.error,
            });
        },
    });

    const onDelete = useCallback(async () => {
        const confirmed = await modals.show('ConfirmDeletion', {
            title: 'Delete Organization',
            description: `Are you sure you want to delete the organization "${thisOrg?.name}"? This action cannot be undone.`,
            objectName: thisOrgSlug || '',
        });

        if (!confirmed) return;

        deleteOrg.mutate({
            params: {
                path: {
                    org_slug: thisOrgSlug,
                },
            },
        });
    }, [thisOrg?.name, thisOrgSlug]);

    return (
        <>
            <PageHeader
                path={[
                    {
                        label: thisOrg?.name,
                        href: `/app/orgs/${thisOrg?.slug}`,
                    },
                    {
                        label: 'Settings',
                    },
                ]}
            />
            <div className="flex-1 p-4 pt-0 flex justify-center max-w-full">
                <div className="w-full max-w-xl lg:pt-4">
                    <Form {...generalForm}>
                        <form
                            onSubmit={generalForm.handleSubmit((v) => {
                                modifyingSlug.current = v.slug !== thisOrgSlug;
                                newSlug.current = v.slug;

                                mutate({
                                    params: { path: { org_slug: thisOrgSlug } },
                                    body: {
                                        ...v,
                                        avatar_email: v.avatar_email || undefined, // Ensure empty string is not sent
                                    },
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
                                <Setting
                                    title="Description"
                                    formControl={generalForm.control}
                                    name="description"
                                    icon={IconPencil}
                                    placeholder="Description..."
                                    type="textarea"
                                />
                                <Setting
                                    title="Gravatar email"
                                    formControl={generalForm.control}
                                    name="avatar_email"
                                    icon={IconMail}
                                    placeholder="gravatar@example.com"
                                />
                                <Button className="mt-3" type="submit">
                                    Save settings
                                </Button>
                            </SettingsSection>
                        </form>
                    </Form>
                    <SettingsSection title="Danger Zone">
                        <SettingAction
                            title="Delete Organization"
                            description="This action cannot be undone."
                            rightSection={
                                <Button variant="destructive" onClick={onDelete}>
                                    Delete Organization
                                </Button>
                            }
                        />
                    </SettingsSection>
                </div>
            </div>
        </>
    );
}
