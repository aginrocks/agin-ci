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

export function RoleSelector<
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
                    label: 'Owner',
                    value: 'owner',
                    description:
                        'Can do everything in the organization, including deleting it and all its data',
                    icon: IconCrown,
                },
                {
                    label: 'Admin',
                    value: 'admin',
                    description: 'Can manage organization settings, members, and repositories',
                    icon: IconCrown,
                },
                {
                    label: 'Member',
                    value: 'member',
                    description:
                        'Can manage organization settings and repositories, but cannot manage members',
                    icon: IconUser,
                },
                {
                    label: 'Viewer',
                    value: 'viewer',
                    description:
                        'Can view organization settings, repositories and members, but cannot make changes',
                    icon: IconEye,
                },
            ]}
            // icon={IconServer}
        />
    );
}
