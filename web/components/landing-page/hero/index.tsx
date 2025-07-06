import { Cover } from '@components/ui/cover';
import { HeroBackground } from './hero-background';
import { HeroGraphics } from './hero-graphics';

export function Hero() {
    return (
        <div className="w-full h-screen flex flex-col items-center py-40 relative">
            <HeroBackground />
            <div className="dark:bg-neutral-900 bg-neutral-100 text-muted-foreground font-semibold px-3 py-1.5 rounded-full text-xs">
                RUST-POWERED CI/CD
            </div>
            <div className="text-5xl font-medium text-center my-5">
                Ship Code at <Cover>Lightning Speed</Cover>
            </div>
            <div className="text-center text-muted-foreground w-3xl leading-relaxed">
                A next-generation CI/CD platform built in Rust, designed for developers who demand
                speed, security, and reliability. While optimized for mobile development (Android,
                iOS, React Native, Expo), our platform excels at building any application - from web
                services to desktop apps.
            </div>
            <div className="flex-1 flex items-center">
                <HeroGraphics />
            </div>
        </div>
    );
}
