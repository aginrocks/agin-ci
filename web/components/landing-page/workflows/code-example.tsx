import { codeToHtml } from 'shiki';

export type CodeBlockProps = {
    children: string;
    lang: string;
};

export async function CodeBlock({ children, lang }: CodeBlockProps) {
    const out = await codeToHtml(children, {
        lang,
        theme: 'vesper',
    });

    return (
        <div
            dangerouslySetInnerHTML={{ __html: out }}
            className="bg-[#101010] px-4 py-3 rounded-md border"
        />
    );
}
