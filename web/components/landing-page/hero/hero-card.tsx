import { cn } from '@lib/utils';

export type HeroCardProps = React.ComponentProps<'div'>;

export function HeroCard({ className, children, ...props }: HeroCardProps) {
    return (
        <div
            className={cn(
                'bg-neutral-100 dark:bg-neutral-800/20 backdrop-blur-xs border border-border rounded-xl flex flex-col hover:dark:border-white/15 transition-all duration-300 cursor-pointer',
                className
            )}
        >
            {children}
        </div>
    );
}
