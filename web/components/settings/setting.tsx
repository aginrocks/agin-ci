'use client';
import { FormControl, FormField, FormItem, FormLabel, FormMessage } from '@components/ui/form';
import { Input } from '@components/ui/input';
import { Textarea } from '@components/ui/textarea';
import { Icon } from '@tabler/icons-react';
import { Control, FieldPath, FieldPathValue, FieldValues } from 'react-hook-form';

export type SettingType = 'text' | 'textarea';

export type SettingProps<
    TFieldValues extends FieldValues = FieldValues,
    TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> = {
    title: string;
    type?: SettingType;
    icon?: Icon;
    description?: string;
    formControl: Control<TFieldValues>;
    name: TName;
    defaultValue?: FieldPathValue<TFieldValues, TName>;
    placeholder?: string;
};

export function Setting<
    TFieldValues extends FieldValues = FieldValues,
    TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
    icon: Icon,
    title,
    description,
    formControl,
    name,
    defaultValue,
    type = 'text',
    placeholder,
}: SettingProps<TFieldValues, TName>) {
    return (
        <div className="mt-4">
            <FormField
                control={formControl}
                name={name}
                defaultValue={defaultValue}
                render={({ field }) => (
                    <FormItem>
                        <div className="flex items-center gap-1">
                            {Icon && <Icon className="text-muted-foreground size-4" />}
                            <FormLabel>{title}</FormLabel>
                        </div>
                        <FormControl>
                            <div className="max-w-lg">
                                {type === 'textarea' ? (
                                    <Textarea placeholder={placeholder} {...field} />
                                ) : (
                                    <Input placeholder={placeholder} {...field} />
                                )}
                            </div>
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                )}
            />
        </div>
    );
}
