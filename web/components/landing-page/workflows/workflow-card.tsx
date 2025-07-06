import { Card } from '@components/ui/card';
import { Icon, IconBrandAndroid, IconBrandApple } from '@tabler/icons-react';

export type WorkflowCardProps = {
    description: string;
    icons?: Icon[];
};

export function WorkflowCard({ description, icons }: WorkflowCardProps) {
    return (
        <Card className="p-4">
            <div>
                {icons && (
                    <div className="flex text-muted-foreground gap-1 mb-2">
                        {icons?.map((IconComponent, index) => (
                            <IconComponent key={index} className="size-6" stroke={1.5} />
                        ))}
                    </div>
                )}
                <div className="text-muted-foreground">{description}</div>
            </div>
        </Card>
    );
}
