export type SettingActionProps = {
    title: string;
    description?: string;
    rightSection?: React.ReactNode;
};

export function SettingAction({ title, description, rightSection }: SettingActionProps) {
    return (
        <div className="p-4 rounded-lg flex justify-between items-center border mt-4">
            <div className="flex-1 flex flex-col gap-0.5">
                <div className="font-medium">{title}</div>
                {description && <div className="text-xs text-muted-foreground">{description}</div>}
            </div>
            <div>{rightSection}</div>
        </div>
    );
}
