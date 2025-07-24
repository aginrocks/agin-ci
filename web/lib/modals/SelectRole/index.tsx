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

    return (
        <Dialog {...props}>
            <DialogContent className="w-md">
                <DialogHeader>
                    <DialogTitle>Change role</DialogTitle>
                    <DialogDescription>Changing {payload?.user}'s role</DialogDescription>
                </DialogHeader>
                <Form {...form}>
                    <RoleSelector formControl={form.control} name="role" />
                </Form>
                <div className="flex justify-end gap-2 mt-2">
                    <Button
                        onClick={() => modals.hide('SelectRole', form.getValues('role'))}
                        variant="default"
                    >
                        Done
                    </Button>
                </div>
            </DialogContent>
        </Dialog>
    );
}
