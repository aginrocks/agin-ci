'use client';
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
} from '@components/ui/dialog';
import * as DialogPrimitive from '@radix-ui/react-dialog';
import * as React from 'react';
import { ModalProps, useModals } from '../ModalsManager';
import { Button } from '@components/ui/button';
import { OrgRole } from '@/types/org-role';
import { useForm } from 'react-hook-form';
import { RoleSelector } from '@components/role-selector';
import { Form } from '@components/ui/form';
import { Alert, AlertDescription, AlertTitle } from '@components/ui/alert';
import {
    IconBrandApple,
    IconBrandUbuntu,
    IconBrandWindows,
    IconExclamationCircle,
    IconPencil,
    IconServer,
} from '@tabler/icons-react';
import z from 'zod';
import { paths } from '@/types/api';
import { zodResolver } from '@hookform/resolvers/zod';
import { Setting } from '@components/settings/setting';

export const formSchema = z.object({
    display_name: z
        .string()
        .min(1, 'Display name is required')
        .max(32, 'Display name must be at most 32 characters long'),
    host_os_type: z.enum(['linux', 'macos', 'windows', 'unknown']),
}) satisfies z.ZodType<
    paths['/api/system/runners']['post']['requestBody']['content']['application/json']
>;

export type FormSchema = z.infer<typeof formSchema>;

export function EditRunner({
    payload,
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Root> & ModalProps<'EditRunner'>) {
    const modals = useModals();

    const form = useForm<FormSchema>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            host_os_type: 'linux',
            display_name: '',
        },
    });

    return (
        <Dialog {...props}>
            <DialogContent className="w-md">
                <DialogHeader>
                    <DialogTitle>Add a Runner</DialogTitle>
                    <DialogDescription>Choose runner's host OS and enter a name.</DialogDescription>
                </DialogHeader>
                <Form {...form}>
                    <div>
                        <Setting
                            formControl={form.control}
                            name="display_name"
                            type="text"
                            icon={IconPencil}
                            title="Runner Name"
                            className="m-0"
                            placeholder="Enter a name for the runner"
                        />
                        <Setting
                            formControl={form.control}
                            name="host_os_type"
                            type="select"
                            options={[
                                {
                                    label: 'Linux',
                                    value: 'linux',
                                    description: 'Can run jobs on Linux and Windows (soon!)',
                                    icon: IconBrandUbuntu,
                                },
                                {
                                    label: 'macOS',
                                    value: 'macos',
                                    description: 'Can run jobs on Linux and macOS (soon!)',
                                    icon: IconBrandApple,
                                },
                                {
                                    label: 'Windows',
                                    value: 'windows',
                                    description: 'Can run jobs only on Linux',
                                    icon: IconBrandWindows,
                                },
                            ]}
                            icon={IconServer}
                            title="Host OS"
                        />
                    </div>
                </Form>
                <div className="flex justify-end gap-2 mt-2">
                    <Button onClick={() => modals.hide('EditRunner')} variant="default">
                        Add Runner
                    </Button>
                </div>
            </DialogContent>
        </Dialog>
    );
}
