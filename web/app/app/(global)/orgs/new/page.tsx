'use client';
import { paths } from '@/types/api';
import { PageHeader } from '@components/page-header';
import { Wizard } from '@components/wizards/wizard';
import { WizardPage } from '@components/wizards/wizard-page';
import { IconBuildings, IconCheck, IconLink, IconPencil, IconX } from '@tabler/icons-react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@components/ui/form';
import { Input } from '@components/ui/input';
import { Button } from '@components/ui/button';
import { Textarea } from '@components/ui/textarea';
import { $api } from '@lib/providers/api';
import { useRouter } from 'next/navigation';
import { useEffect } from 'react';
import { slugify } from '@lib/utils';
import { toast } from 'sonner';

const RESERVED_SLUGS = ['new', 'edit', 'delete', 'create'];

const isValidSlug = (s: string): boolean => {
    return s
        .split('')
        .every((c) => (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c === '-' || c === '_');
};

export const formSchema = z.object({
    name: z.string().min(1, 'Name is required').max(32, 'Name must be at most 32 characters long'),
    description: z.string().max(2048, 'Description must be at most 2048 characters long'),
    slug: z
        .string()
        .min(1, 'Slug is required')
        .max(32, 'Slug must be at most 32 characters long')
        .refine(
            isValidSlug,
            'Slug can only contain lowercase letters, numbers, hyphens, and underscores'
        )
        .refine((s) => !RESERVED_SLUGS.includes(s), 'This slug is reserved and cannot be used'),
    avatar_email: z
        .string()
        .email('Invalid email address')
        .max(64, 'Email must be at most 64 characters long')
        .or(z.literal(''))
        .optional(),
}) satisfies z.ZodType<
    paths['/api/organizations']['post']['requestBody']['content']['application/json']
> satisfies z.ZodType<
    paths['/api/organizations/{org_slug}']['patch']['requestBody']['content']['application/json']
>;

export type FormSchema = z.infer<typeof formSchema>;

export default function Page() {
    const router = useRouter();
    const form = useForm<FormSchema>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: '',
            description: '',
            slug: '',
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

    const create = $api.useMutation('post', '/api/organizations', {
        onSuccess: () => {
            router.replace(`/app/orgs/${form.getValues('slug')}`);
        },
        onError: (error) => {
            toast.error('Failed to create organization', {
                description: error.error,
            });
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
                        label: 'New Organization',
                    },
                ]}
            />
            <Form {...form}>
                <Wizard>
                    <WizardPage
                        pageNumber={0}
                        icon={IconBuildings}
                        title="Create Organization"
                        description="Organizations allow you to group multiple projects together and share secrets and permissions across them."
                        beforeNext={async () => await form.trigger('name')}
                    >
                        <FormField
                            control={form.control}
                            name="name"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Name</FormLabel>
                                    <FormControl>
                                        <Input placeholder="Acme Inc." {...field} />
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
                                        <Input placeholder="acme-inc" {...field} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                    </WizardPage>
                    <WizardPage
                        pageNumber={2}
                        icon={IconPencil}
                        title="Write a Description"
                        description="The description will be shown in the organization details view."
                        swapNextButton={
                            <Button
                                disabled={create.isPending}
                                onClick={() =>
                                    form.handleSubmit(() =>
                                        create.mutate({
                                            body: form.getValues(),
                                        })
                                    )()
                                }
                            >
                                <IconCheck />
                                Done
                            </Button>
                        }
                    >
                        <FormField
                            control={form.control}
                            name="description"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Description</FormLabel>
                                    <FormControl>
                                        <Textarea
                                            placeholder="Description..."
                                            rows={4}
                                            {...field}
                                        />
                                    </FormControl>
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
