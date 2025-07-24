import { Control, FieldPath, FieldValues } from 'react-hook-form';
import { Setting } from './settings/setting';
import { IconCrown, IconEye, IconUser } from '@tabler/icons-react';

export type RoleSelectorProps<
    TFieldValues extends FieldValues = FieldValues,
    TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> = {
    formControl: Control<TFieldValues>;
    name: TName;
};

export function ServerRoleSelector<
    TFieldValues extends FieldValues = FieldValues,
    TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({ name, formControl }: RoleSelectorProps<TFieldValues, TName>) {
    return (
        <Setting
            formControl={formControl}
            name={name}
            type="select"
            className="m-0"
            options={[
                {
                    label: 'Admin',
                    value: 'admin',
                    description: 'Can do everything on the server',
                    icon: IconCrown,
                },
                {
                    label: 'User',
                    value: 'user',
                    description: 'A regular user',
                    icon: IconUser,
                },
                {
                    label: 'Read Only',
                    value: 'readonly',
                    description: 'Read-only access to the server',
                    icon: IconEye,
                },
            ]}
        />
    );
}
