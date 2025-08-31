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

const formSchema = z.object({
    token: z.string(),
});

export function EnterToken({
    payload,
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Root> & ModalProps<'EnterToken'>) {
    const modals = useModals();

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            token: '',
        },
    });

    return (
        <Dialog {...props}>
            <DialogContent className="w-md">
                <DialogHeader>
                    <DialogTitle>Enter an Access Token</DialogTitle>
                    <DialogDescription>
                        You can generate the token in your repository provider's settings. You won't
                        be able to see the token again after you enter it.
                    </DialogDescription>
                </DialogHeader>
                <Form {...form}>
                    <form onSubmit={form.handleSubmit((v) => modals.hide('EnterToken', v.token))}>
                        <FormField
                            control={form.control}
                            name="token"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Access Token</FormLabel>
                                    <FormControl>
                                        <Input
                                            placeholder="Paste access token here..."
                                            type="password"
                                            {...field}
                                        />
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
                            <Button type="submit">
                                {payload?.isUpdating ? 'Update' : 'Set'} Token
                            </Button>
                        </div>
                    </form>
                </Form>
            </DialogContent>
        </Dialog>
    );
}
