import { cn } from '@lib/utils';

export type HeroCardProps = React.ComponentProps<'div'>;

export function HeroCard({ className, children, ...props }: HeroCardProps) {
    return (
        <div
            className={cn(
                'bg-neutral-400/20 backdrop-blur-md border border-white/10 rounded-xl flex flex-col',
                className
            )}
        >
            {children}
        </div>
    );
}
