import { useWindowEvent } from '@mantine/hooks';
import { useCallback, useState } from 'react';

export function useModifier(modifier: string) {
    const [pressed, setPressed] = useState(false);

    const handleKeyDown = useCallback((event: KeyboardEvent) => {
        if (event.key === modifier) {
            setPressed(true);
        }
    }, []);

    const handleKeyUp = useCallback((event: KeyboardEvent) => {
        if (event.key === modifier) {
            setPressed(false);
        }
    }, []);

    useWindowEvent('keydown', handleKeyDown);
    useWindowEvent('keyup', handleKeyUp);

    return pressed;
}
