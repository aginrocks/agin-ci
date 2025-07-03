import { DOCS_URL, REPO_URL } from '@lib/constants';
import { IconBook, IconBrandGithub } from '@tabler/icons-react';

export const navSecondary = [
    {
        title: 'Source Code',
        url: REPO_URL,
        icon: IconBrandGithub,
    },
    {
        title: 'Documentation',
        url: DOCS_URL,
        icon: IconBook,
    },
];
