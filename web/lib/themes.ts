import {
    Icon,
    IconCloud,
    IconCoffee,
    IconMoon,
    IconMoonStars,
    IconSun,
    IconSunMoon,
} from '@tabler/icons-react';

type Theme = {
    type: 'theme';
    icon?: Icon;
    label: string;
    className: string;
};

type Separator = {
    type: 'separator';
};

export const THEMES: (Theme | Separator)[] = [
    {
        type: 'theme',
        icon: IconSun,
        label: 'Light',
        className: 'light',
    },
    {
        type: 'theme',
        icon: IconMoon,
        label: 'Dark',
        className: 'dark',
    },
    {
        type: 'theme',
        icon: IconSunMoon,
        label: 'System',
        className: 'system',
    },
    {
        type: 'separator',
    },
    {
        type: 'theme',
        label: 'Latte',
        className: 'latte-mauve',
        icon: IconCoffee,
    },
    {
        type: 'theme',
        label: 'Frappe',
        className: 'frappe-mauve',
        icon: IconCloud,
    },
    {
        type: 'theme',
        label: 'Macchiato',
        className: 'macchiato-mauve',
        icon: IconMoon,
    },
    {
        type: 'theme',
        label: 'Mocha',
        className: 'mocha-mauve',
        icon: IconMoonStars,
    },
];
