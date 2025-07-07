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
import { useMemo } from 'react';
import z from 'zod';
import { useForm } from 'react-hook-form';
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
import { zodResolver } from '@hookform/resolvers/zod';

export function ConfirmDeletion({
    payload,
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Root> & ModalProps<'ConfirmDeletion'>) {
    const modals = useModals();

    const formSchema = useMemo(
        () =>
            z.object({
                confirmation: z
                    .string()
                    .refine((x) => x === payload?.objectName, 'Invalid confirmation'),
            }),
        [payload?.objectName]
    );

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            confirmation: '',
        },
    });

    return (
        <Dialog {...props}>
            <DialogContent className="w-md">
                <DialogHeader>
                    <DialogTitle>{payload?.title}</DialogTitle>
                    {payload?.description && (
                        <DialogDescription>{payload?.description}</DialogDescription>
                    )}
                </DialogHeader>
                <Form {...form}>
                    <form onSubmit={form.handleSubmit(() => modals.hide('ConfirmDeletion', true))}>
                        <FormField
                            control={form.control}
                            name="confirmation"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>{`To confirm, type "${payload?.objectName}" below`}</FormLabel>
                                    <FormControl>
                                        <Input placeholder={payload?.objectName} {...field} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <div className="flex justify-end gap-2 mt-4">
                            <Button
                                variant="ghost"
                                type="button"
                                onClick={() => modals.hide('ConfirmDeletion', false)}
                            >
                                Cancel
                            </Button>
                            <Button variant="destructive" type="submit">
                                Delete
                            </Button>
                        </div>
                    </form>
                </Form>
            </DialogContent>
        </Dialog>
    );
}
