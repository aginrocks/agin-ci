'use client';
import { Button } from '@components/ui/button';
import { cn } from '@lib/utils';
import { Icon, IconArrowRight } from '@tabler/icons-react';
import { useWizard } from './wizard';
import { ReactElement, useCallback } from 'react';
import { useHotkeys } from '@mantine/hooks';

export type WizardPageProps = React.ComponentProps<'div'> & {
    pageNumber: number;
    icon?: Icon;
    title?: string;
    description?: string;
    children?: React.ReactNode;
    swapNextButton?: ReactElement<any, any>;
    beforeNext?: () => boolean | void | Promise<boolean | void>;
};

export function WizardPage({
    children,
    pageNumber,
    className,
    icon: Icon,
    title,
    description,
    swapNextButton,
    beforeNext,
    ...props
}: WizardPageProps) {
    const wizard = useWizard();

    const onNext = useCallback(
        async (e: React.MouseEvent<HTMLButtonElement> | KeyboardEvent) => {
            e.preventDefault();
            if (beforeNext) {
                const canContinue = await beforeNext();
                if (canContinue === false) return;
            }
            wizard.next();
        },
        [beforeNext, wizard]
    );

    useHotkeys([['Enter', swapNextButton ? swapNextButton.props.onClick : onNext]], ['TEXTAREA']);

    return (
        <div className={cn('w-lg mx-4', className)} {...props}>
            <div className="flex flex-col gap-3 mb-4">
                {Icon && <Icon className="text-muted-foreground size-12" />}
                <div>
                    {title && <div className="text-2xl font-bold">{title}</div>}
                    {description && (
                        <div className="text-sm text-muted-foreground mt-1">{description}</div>
                    )}
                </div>
            </div>
            {children}
            <div className="flex justify-end mt-4 gap-2">
                {pageNumber !== 0 && (
                    <Button
                        onClick={(e) => {
                            e.preventDefault();
                            wizard.prev();
                        }}
                        variant="ghost"
                    >
                        Back
                    </Button>
                )}
                {swapNextButton || (
                    <Button onClick={onNext}>
                        Next
                        <IconArrowRight />
                    </Button>
                )}
            </div>
        </div>
    );
}
