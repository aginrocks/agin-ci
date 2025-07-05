'use client';

import { Header } from '@components/landing-page/header';
import { Hero } from '@components/landing-page/hero';

export default function Home() {
    return (
        <div
            style={{
                fontFamily: 'var(--font-inter), sans-serif',
            }}
        >
            <Header />
            <Hero />
        </div>
    );
}
