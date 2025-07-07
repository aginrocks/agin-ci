export type SettingsSectionProps = {
    title: string;
    description?: string;
    children?: React.ReactNode;
};

export function SettingsSection({ title, description, children }: SettingsSectionProps) {
    return (
        <div className="mb-4">
            <div className="border-b border-muted pb-2.5">
                <div className="text-xl font-medium">{title}</div>
                {description && <div className="text-xs text-muted-foreground">{description}</div>}
            </div>
            {children}
        </div>
    );
}
