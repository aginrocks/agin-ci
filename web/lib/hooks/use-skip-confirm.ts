import { useModifier } from './use-modifier';

export function useSkipConfirm() {
    const modPressed = useModifier('Shift');

    return modPressed;
}
