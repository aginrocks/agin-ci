import { Header } from '@components/landing-page/header';
import { Hero } from '@components/landing-page/hero';
import { Workflows } from '@components/landing-page/workflows';
import { CodeBlock } from '@components/landing-page/workflows/code-example';
import yaml from 'yaml';

// TODO: Update with actual syntax when the backend is ready
const WORKFLOWS = {
    mobile: yaml.stringify({
        on: 'push',
        name: 'Build Mobile App',
        jobs: {
            build_ios: {
                'runs-on': 'macos-latest',
                steps: [
                    { name: 'Checkout', uses: 'actions/checkout@v2' },
                    { name: 'Install Dependencies', run: 'npm install' },
                    { name: 'Build iOS App', run: 'npm run build-ios' },
                ],
            },
        },
    }),
};

export default function Home() {
    return (
        <div
            style={{
                fontFamily: 'var(--font-inter), sans-serif',
            }}
        >
            <Header />
            <Hero />
            <Workflows
                codeBlocks={{
                    mobile: <CodeBlock lang="yaml">{WORKFLOWS.mobile}</CodeBlock>,
                }}
            />
            <div className="h-screen"></div>
        </div>
    );
}
