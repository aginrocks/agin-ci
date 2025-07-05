import { cn } from '@lib/utils';

export function HeroBackground() {
    return (
        <div className="absolute inset-0 flex h-full w-full items-center justify-center">
            <div
                className={cn(
                    'absolute inset-0 -z-2',
                    '[background-size:30px_30px]',
                    '[background-image:radial-gradient(#323232_1px,transparent_1px)]'
                )}
            />
            <div className="pointer-events-none absolute inset-0 flex items-center justify-center bg-white [mask-image:radial-gradient(ellipse_at_center,transparent_20%,black)] dark:bg-background -z-1"></div>
        </div>
    );
}
