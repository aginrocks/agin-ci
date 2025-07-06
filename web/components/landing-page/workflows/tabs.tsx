import { Button } from '@components/ui/button';
import { Dispatch, SetStateAction } from 'react';

const tabs = [
    {
        id: 'mobile',
        label: 'Mobile Development',
    },
    {
        id: 'rust',
        label: 'Rust Builds',
    },
    {
        id: 'docker',
        label: 'Docker Containers',
    },
];

export type WorkflowTabsProps = {
    tab: string;
    onTabChange: Dispatch<SetStateAction<string>>;
};

export function WorkflowsTabs({ tab, onTabChange }: WorkflowTabsProps) {
    return (
        <div className="flex items-center gap-0.5 mt-4">
            {tabs.map((t) => {
                const isActive = t.id === tab;

                return (
                    <Button
                        className="rounded-full px-5"
                        variant={isActive ? 'default' : 'ghost'}
                        size="lg"
                        key={t.id}
                        onClick={() => onTabChange(t.id)}
                    >
                        {t.label}
                    </Button>
                );
            })}
        </div>
    );
}
