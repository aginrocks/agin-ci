'use client';
import { cn } from '@lib/utils';
import React, { createContext, Dispatch, SetStateAction, useContext, useState } from 'react';
import { AnimatePresence, motion } from 'motion/react';

export type WizardProps = React.ComponentProps<'div'> & {
    children?: React.ReactNode;
};

type WizardContextProps = {
    currentPage: number;
    setCurrentPage: Dispatch<SetStateAction<number>>;
    direction: 1 | -1;
    setDirection: Dispatch<SetStateAction<1 | -1>>;
};

export const WizardContext = createContext<WizardContextProps | null>(null);

export function useWizard() {
    const ctx = useContext(WizardContext);
    return {
        ...ctx,
        next: () => {
            ctx?.setDirection?.(1);
            setTimeout(() => ctx?.setCurrentPage((p) => p + 1), 0);
        },
        prev: () => {
            ctx?.setDirection?.(-1);
            setTimeout(() => ctx?.setCurrentPage((p) => p - 1), 0);
        },
    };
}

export function Wizard({ children, className, ...props }: WizardProps) {
    const [currentPage, setCurrentPage] = useState(0);
    const [direction, setDirection] = useState<1 | -1>(1);

    const childrenArray = React.Children.toArray(children);
    const currentPageComponent = childrenArray.find((child) => {
        if (React.isValidElement(child) && typeof child.type !== 'string') {
            const props = child.props as { pageNumber?: number };
            return props.pageNumber === currentPage;
        }
        return false;
    });

    return (
        <WizardContext.Provider value={{ currentPage, setCurrentPage, direction, setDirection }}>
            <div
                className={cn(
                    'grid place-items-center flex-1 relative overflow-hidden max-h-full',
                    className
                )}
                {...props}
            >
                <AnimatePresence initial={false}>
                    {currentPageComponent && (
                        <motion.div
                            initial={{
                                opacity: 0,
                                transform: `translateY(${300 * direction}px)`,
                                transition: { duration: 0.3 },
                            }}
                            animate={{
                                opacity: 1,
                                transform: 'translateY(0px)',
                                transition: { duration: 0.3 },
                            }}
                            exit={{
                                opacity: 0,
                                transform: `translateY(${-300 * direction}px)`,
                                transition: { duration: 0.3 },
                            }}
                            className="col-start-1 row-start-1 max-h-full"
                            key={currentPage}
                        >
                            {currentPageComponent}
                        </motion.div>
                    )}
                </AnimatePresence>
            </div>
        </WizardContext.Provider>
    );
}
