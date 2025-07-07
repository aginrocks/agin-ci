'use client';
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
};

export const ModalsBinding: ModalComponentBindings = {
    ConfirmDeletion: ConfirmDeletion,
};
