'use client';
import { createContext } from 'react';
import { ModalName, ModalPayload, ModalReturnValue } from './types';

export type ModalsContextType = {
    show: <T extends ModalName>(
        modalName: T,
        payload?: ModalPayload<T>
    ) => Promise<ModalReturnValue<T> | undefined>;
    hide: <T extends ModalName>(modalName: T, payload?: ModalReturnValue<T>) => void;
};

const initial: ModalsContextType = {
    show: async () => undefined,
    hide: () => {},
};

export const ModalsContext = createContext<ModalsContextType>(initial);
