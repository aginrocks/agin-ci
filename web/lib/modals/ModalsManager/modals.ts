'use client';
import { OrgRole } from '@/types/org-role';
import { Confirm } from '../Confirm';
import { ConfirmDeletion } from '../ConfirmDeletion';
import { OneTimeSecret } from '../OneTimeSecret';
import { SelectRole } from '../SelectRole';
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
    OneTimeSecret: ModalDefinition<{
        payload: {
            title: string;
            description?: string;
            secret: string;
        };
        returnValue: void;
    }>;
    SelectRole: ModalDefinition<{
        payload: {
            selectedRole: OrgRole;
            user: string;
        };
        returnValue: OrgRole;
    }>;
};

export const ModalsBinding: ModalComponentBindings = {
    ConfirmDeletion: ConfirmDeletion,
    Confirm: Confirm,
    OneTimeSecret: OneTimeSecret,
    SelectRole: SelectRole,
};
