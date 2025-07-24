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
import { IconExclamationCircle } from '@tabler/icons-react';

export type FormSchema = {
    role: OrgRole;
};

export function SelectRole({
    payload,
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Root> & ModalProps<'SelectRole'>) {
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
                    {role === 'owner' && (
                        <Alert variant="destructive">
                            <IconExclamationCircle />
                            <AlertTitle>This action is irreversible!</AlertTitle>
                            <AlertDescription>
                                If you grant ownership to another user, you will be demoted to an
                                admin and lose some of your permissions.
                            </AlertDescription>
                        </Alert>
                    )}
                    <Form {...form}>
                        <RoleSelector formControl={form.control} name="role" />
                    </Form>
                </div>
                <div className="flex justify-end gap-2 mt-2">
                    <Button
                        onClick={() => modals.hide('SelectRole', form.getValues('role'))}
                        variant={role === 'owner' ? 'destructive' : 'default'}
                    >
                        {role === 'owner' ? 'Transfer Ownership' : 'Chnage Role'}
                    </Button>
                </div>
            </DialogContent>
        </Dialog>
    );
}
