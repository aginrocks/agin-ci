import defaultComponents from 'fumadocs-ui/mdx';
import { APIPage } from 'fumadocs-openapi/ui';
import { openapi } from '@/lib/source';
import type { MDXComponents } from 'mdx/types';
import { Mermaid } from './components/mermaid';

export function getMDXComponents(components?: MDXComponents): MDXComponents {
    return {
        ...defaultComponents,
        APIPage: (props) => <APIPage {...openapi.getAPIPageProps(props)} />,
        Mermaid,
        ...components,
    };
}
