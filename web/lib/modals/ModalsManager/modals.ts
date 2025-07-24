'use client';
import { OrgRole } from '@/types/org-role';
import { Confirm } from '../Confirm';
import { ConfirmDeletion } from '../ConfirmDeletion';
import { OneTimeSecret } from '../OneTimeSecret';
import { SelectRole } from '../SelectRole';
import { ModalComponentBindings, ModalDefinition } from './types';
import { SelectServerRole } from '../SelectServerRole';
import { ServerRole } from '@/types/server-role';

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
    SelectServerRole: ModalDefinition<{
        payload: {
            selectedRole: ServerRole;
            user: string;
        };
        returnValue: ServerRole;
    }>;
};

export const ModalsBinding: ModalComponentBindings = {
    ConfirmDeletion: ConfirmDeletion,
    Confirm: Confirm,
    OneTimeSecret: OneTimeSecret,
    SelectRole: SelectRole,
    SelectServerRole: SelectServerRole,
};
