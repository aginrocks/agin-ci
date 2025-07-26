import { cn } from '@lib/utils';
import clsx from 'clsx';

export type StatusBadgeProps = {
    variant: 'success' | 'disabled' | 'error';
    children: React.ReactNode;
};

export function StatusBadge({ variant, children }: StatusBadgeProps) {
    return (
        <span className="flex items-center gap-1.5 text-xs font-medium">
            <div
                className={cn(
                    'w-2 h-2 rounded-full',
                    clsx({
                        'bg-green-500': variant === 'success',
                        'bg-gray-500': variant === 'disabled',
                        'bg-red-500': variant === 'error',
                    })
                )}
            />
            {children}
        </span>
    );
}
