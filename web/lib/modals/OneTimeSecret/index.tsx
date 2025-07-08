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
import { Input } from '@components/ui/input';
import { useClipboard } from '@mantine/hooks';
import { IconCheck, IconCopy } from '@tabler/icons-react';

export function OneTimeSecret({
    payload,
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Root> & ModalProps<'OneTimeSecret'>) {
    const modals = useModals();

    const clipboard = useClipboard();

    return (
        <Dialog {...props}>
            <DialogContent className="w-md">
                <DialogHeader>
                    <DialogTitle>{payload?.title}</DialogTitle>
                    {payload?.description && (
                        <DialogDescription>{payload?.description}</DialogDescription>
                    )}
                </DialogHeader>
                <div className="flex gap-2">
                    <Input readOnly value={payload?.secret} />
                    <Button
                        variant="outline"
                        size="icon"
                        onClick={() => clipboard.copy(payload?.secret)}
                    >
                        {clipboard.copied ? <IconCheck /> : <IconCopy />}
                    </Button>
                </div>
                <div className="flex justify-end gap-2">
                    <Button onClick={() => modals.hide('OneTimeSecret')}>Done</Button>
                </div>
            </DialogContent>
        </Dialog>
    );
}
