import { Modals } from './modals';
import * as DialogPrimitive from '@radix-ui/react-dialog';

export type DefaultModalDefinition = {
    payload: unknown;
    returnValue: unknown;
};
export type ModalDefinition<T extends DefaultModalDefinition = DefaultModalDefinition> = {
    payload: T['payload'] | undefined;
    returnValue: T['returnValue'] | undefined;
};

export type ModalName = keyof Modals;
export type Modal<T extends ModalName> = Modals[T];

export type ModalPayload<T extends ModalName> = Modals[T]['payload'];
export type ModalReturnValue<T extends ModalName> = Modals[T]['returnValue'];

export type ModalProps<T extends ModalName> = React.ComponentProps<typeof DialogPrimitive.Root> & {
    payload: ModalPayload<T>;
    modalName: T;
};

export type ModalComponentBindings = {
    [T in ModalName]: React.FC<ModalProps<T>>;
};

export type ModalState = 'visible' | 'closing';

export type ModalStoreItem<T extends ModalName> = {
    name: T;
    state: ModalState;
    payload?: ModalPayload<T>;
    resolve: (value: ModalReturnValue<T> | undefined) => void;
};

export type ModalStore = {
    [T in ModalName]?: ModalStoreItem<T>;
};
