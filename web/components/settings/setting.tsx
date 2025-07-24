'use client';
import {
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@components/ui/form';
import { Input } from '@components/ui/input';
import { Textarea } from '@components/ui/textarea';
import { Icon } from '@tabler/icons-react';
import { Control, FieldPath, FieldPathValue, FieldValues } from 'react-hook-form';
import { SettingAction } from './setting-action';
import { ReactNode } from 'react';
import { cn } from '@lib/utils';

export type SelectOption = {
    icon?: Icon;
    label: string;
    value: string;
    description?: string;
};

export type SettingProps<
    TFieldValues extends FieldValues = FieldValues,
    TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> = {
    title?: string;
    icon?: Icon;
    description?: ReactNode;
    formControl: Control<TFieldValues>;
    name: TName;
    defaultValue?: FieldPathValue<TFieldValues, TName>;
    placeholder?: string;
} & ({ type?: 'text' | 'textarea' } | { type: 'select'; options: SelectOption[] });

export type SettingLikeHeaderProps = React.ComponentProps<'div'> & {
    title: string;
    description?: ReactNode;
    icon?: Icon;
};

export function SettingLikeHeader({
    title,
    description,
    icon: Icon,
    className,
}: SettingLikeHeaderProps) {
    return (
        <div className="mb-2">
            <div
                className={cn(
                    'flex items-center gap-1 text-sm leading-none font-medium select-none',
                    className
                )}
            >
                {Icon && <Icon className="text-muted-foreground size-4" />}
                <div>{title}</div>
            </div>
            {description && <div className="mt-0.5 text-muted-foreground">{description}</div>}
        </div>
    );
}

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
    className,
    ...props
}: SettingProps<TFieldValues, TName> & React.ComponentProps<'div'>) {
    return (
        <div className={cn('mt-4', className)} {...props}>
            <FormField
                control={formControl}
                name={name}
                defaultValue={defaultValue}
                render={({ field }) => (
                    <FormItem>
                        {title && (
                            <div className="flex items-center gap-1">
                                {Icon && <Icon className="text-muted-foreground size-4" />}
                                <FormLabel>{title}</FormLabel>
                            </div>
                        )}
                        <FormControl>
                            <div className="max-w-xl">
                                {type === 'textarea' ? (
                                    <Textarea placeholder={placeholder} {...field} />
                                ) : type === 'text' ? (
                                    <Input placeholder={placeholder} {...field} />
                                ) : type === 'select' ? (
                                    <div className="flex flex-col gap-3 mt-1">
                                        {'options' in props
                                            ? props.options.map((option) => (
                                                  <SettingAction
                                                      key={option.value}
                                                      title={option.label}
                                                      description={option.description}
                                                      icon={option.icon}
                                                      clickable
                                                      className="m-0"
                                                      selected={field.value === option.value}
                                                      onClick={() =>
                                                          field.onChange({
                                                              target: {
                                                                  value: option.value,
                                                              },
                                                          })
                                                      }
                                                  />
                                              ))
                                            : []}
                                    </div>
                                ) : (
                                    <></>
                                )}
                            </div>
                        </FormControl>
                        {description && <FormDescription>{description}</FormDescription>}
                        <FormMessage />
                    </FormItem>
                )}
            />
        </div>
    );
}
