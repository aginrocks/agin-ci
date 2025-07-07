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

export function Confirm({
    payload,
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Root> & ModalProps<'Confirm'>) {
    const modals = useModals();

    return (
        <Dialog {...props}>
            <DialogContent className="w-md">
                <DialogHeader>
                    <DialogTitle>{payload?.title}</DialogTitle>
                    {payload?.description && (
                        <DialogDescription>{payload?.description}</DialogDescription>
                    )}
                </DialogHeader>
                <div className="flex justify-end gap-2 mt-2">
                    <Button variant="ghost" onClick={() => modals.hide('Confirm', false)}>
                        {payload?.cancelText || 'Cancel'}
                    </Button>
                    <Button onClick={() => modals.hide('Confirm', true)}>
                        {payload?.confirmText || 'OK'}
                    </Button>
                </div>
            </DialogContent>
        </Dialog>
    );
}
