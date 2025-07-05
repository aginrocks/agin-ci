import { Cover } from '@components/ui/cover';
import { HeroBackground } from './hero-background';

export function Hero() {
    return (
        <div className="w-full h-screen flex flex-col items-center pt-40 relative">
            <HeroBackground />
            <div className="bg-muted text-muted-foreground font-semibold px-3 py-1.5 rounded-full text-xs">
                RUST-POWERED CI/CD
            </div>
            <div className="text-5xl font-medium text-center mb-6 mt-5">
                Ship Code at <Cover>Lightning Speed</Cover>
            </div>
            <div className="text-center text-muted-foreground w-3xl">
                A next-generation CI/CD platform built in Rust, designed for developers who demand
                speed, security, and reliability. While optimized for mobile development (Android,
                iOS, React Native, Expo), our platform excels at building any application - from web
                services to desktop apps.
            </div>
        </div>
    );
}
