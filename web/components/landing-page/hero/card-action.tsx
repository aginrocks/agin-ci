import { Spinner } from '@components/ui/spinner';
import { IconCircleCheckFilled } from '@tabler/icons-react';
import { Children, ReactNode } from 'react';

export type CardActionProps = {
    title: string;
    timing?: string;
    children?: ReactNode;
    status?: 'loading' | 'success';
};

export function CardAction({ title, timing, children, status }: CardActionProps) {
    const icon =
        status === 'loading' ? (
            <Spinner size="xs" className="text-muted-foreground" />
        ) : status === 'success' ? (
            <IconCircleCheckFilled className="text-green-600" />
        ) : undefined;

    return (
        <div className="flex-1 px-4 py-3">
            <div className="flex items-center gap-2 justify-between">
                <div className="flex items-center gap-2">
                    {icon && <div className="[&_svg:not([class*='size-'])]:size-5">{icon}</div>}
                    <div className="flex flex-col">
                        <span className="font-semibold">{title}</span>
                    </div>
                </div>
                {timing && <span className="text-sm text-muted-foreground">{timing}</span>}
            </div>
            {children}
        </div>
    );
}
