'use client';
import { Confirm } from '../Confirm';
import { ConfirmDeletion } from '../ConfirmDeletion';
import { ModalComponentBindings, ModalDefinition } from './types';

export type Modals = {
    ConfirmDeletion: ModalDefinition<{
        payload: {
            title: string;
            description?: string;
            objectName: string;
        };
        returnValue: boolean;
    }>;
    Confirm: ModalDefinition<{
        payload: {
            title: string;
            description?: string;
            cancelText?: string;
            confirmText?: string;
            destructive?: boolean;
        };
        returnValue: boolean;
    }>;
};

export const ModalsBinding: ModalComponentBindings = {
    ConfirmDeletion: ConfirmDeletion,
    Confirm: Confirm,
};
