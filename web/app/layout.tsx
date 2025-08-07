import type { Metadata } from 'next';
import { Geist, Geist_Mono, Inter, Poppins } from 'next/font/google';
import './globals.css';
import { ThemeProvider } from 'next-themes';
import Query from '@lib/providers/Query';
import { DISPLAY_NAME, TAGLINE } from '@lib/constants';
import { Toaster } from '@components/ui/sonner';
import { ModalsManagerProvider } from '@lib/modals/ModalsManager';

const geistSans = Geist({
    variable: '--font-geist-sans',
    subsets: ['latin'],
});

const geistMono = Geist_Mono({
    variable: '--font-geist-mono',
    subsets: ['latin'],
});

const inter = Inter({
    variable: '--font-inter',
    subsets: ['latin'],
});

const poppins = Poppins({
    variable: '--font-poppins',
    subsets: ['latin'],
    weight: ['400', '500', '600', '700'],
});

export const metadata: Metadata = {
    title: `Dashboard • ${DISPLAY_NAME}`,
    description: TAGLINE,
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <Query>
            <html lang="en" suppressHydrationWarning>
                <body
                    className={`${geistSans.variable} ${geistMono.variable} ${inter.variable} ${poppins.variable} antialiased`}
                >
                    <ThemeProvider
                        attribute="class"
                        defaultTheme="system"
                        enableSystem
                        disableTransitionOnChange
                        themes={[
                            'light',
                            'dark',
                            'mocha-mauve',
                            'frappe-mauve',
                            'latte-mauve',
                            'macchiato-mauve',
                        ]}
                    >
                        <ModalsManagerProvider>
                            {children}
                            <Toaster />
                        </ModalsManagerProvider>
                    </ThemeProvider>
                </body>
            </html>
        </Query>
    );
}
