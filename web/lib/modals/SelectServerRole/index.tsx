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
import { useForm } from 'react-hook-form';
import { RoleSelector } from '@components/role-selector';
import { Form } from '@components/ui/form';
import { Alert, AlertDescription, AlertTitle } from '@components/ui/alert';
import { IconExclamationCircle } from '@tabler/icons-react';
import { ServerRole } from '@/types/server-role';
import { ServerRoleSelector } from '@components/server-role-selector';

export type FormSchema = {
    role: ServerRole;
};

export function SelectServerRole({
    payload,
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Root> & ModalProps<'SelectServerRole'>) {
    const modals = useModals();

    const form = useForm<FormSchema>({
        defaultValues: {
            role: payload?.selectedRole,
        },
    });

    const role = form.watch('role');

    return (
        <Dialog {...props}>
            <DialogContent className="w-md">
                <DialogHeader>
                    <DialogTitle>Change role</DialogTitle>
                    <DialogDescription>Changing {payload?.user}'s role</DialogDescription>
                </DialogHeader>
                <div className="flex flex-col gap-2">
                    {role === 'admin' && (
                        <Alert variant="warning">
                            <IconExclamationCircle />
                            <AlertTitle>Granting this role is dangerous</AlertTitle>
                            <AlertDescription>
                                The admin role grants full access to the server, including the
                                ability to change the roles of every member (including you!)
                            </AlertDescription>
                        </Alert>
                    )}
                    <Form {...form}>
                        <ServerRoleSelector formControl={form.control} name="role" />
                    </Form>
                </div>
                <div className="flex justify-end gap-2 mt-2">
                    <Button
                        onClick={() => modals.hide('SelectServerRole', form.getValues('role'))}
                        variant="default"
                    >
                        Change Role
                    </Button>
                </div>
            </DialogContent>
        </Dialog>
    );
}
