'use client';
import { formSchema, FormSchema } from '@/app/app/(global)/orgs/new/page';
import { PageHeader } from '@components/page-header';
import { SettingsSection } from '@components/settings/section';
import { Setting } from '@components/settings/setting';
import { Form } from '@components/ui/form';
import { zodResolver } from '@hookform/resolvers/zod';
import { useOrg } from '@lib/hooks';
import { IconLink, IconPencil } from '@tabler/icons-react';
import { useForm } from 'react-hook-form';

export default function Page() {
    const { thisOrg } = useOrg();

    const generalForm = useForm<FormSchema>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: '',
            description: '',
            slug: '',
        },
    });

    return (
        <>
            <PageHeader
                path={[
                    {
                        label: 'Organizations',
                        href: '/app/orgs',
                    },
                    {
                        label: thisOrg?.name,
                        href: `/app/orgs/${thisOrg?.slug}`,
                    },
                    {
                        label: 'Settings',
                    },
                ]}
            />
            <div className="flex-1 p-4 pt-0">
                <Form {...generalForm}>
                    <SettingsSection title="General" description="Name, description, etc.">
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
                            name="name"
                            icon={IconPencil}
                            placeholder="Description..."
                            type="textarea"
                        />
                    </SettingsSection>
                </Form>
            </div>
        </>
    );
}
