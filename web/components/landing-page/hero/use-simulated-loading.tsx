import { formatDuration } from '@lib/utils';
import { useEffect, useState } from 'react';

export type SimulatedLoadingProps = {
    delay: number;
    startSeconds: number;
};

export function useSimulatedLoading({ delay, startSeconds }: SimulatedLoadingProps) {
    const [loading, setLoading] = useState(true);
    const [seconds, setSeconds] = useState(startSeconds);

    useEffect(() => {
        const timer = setTimeout(() => {
            setLoading(false);
        }, delay * 1000);

        return () => clearTimeout(timer);
    }, [delay]);

    useEffect(() => {
        if (!loading) return;

        const interval = setInterval(() => {
            setSeconds((prev) => prev + 1);
        }, 1000);

        return () => clearInterval(interval);
    }, [loading]);

    return {
        loading,
        seconds,
        formattedSeconds: formatDuration(seconds),
        status: (loading ? 'loading' : 'success') as 'loading' | 'success',
    };
}
