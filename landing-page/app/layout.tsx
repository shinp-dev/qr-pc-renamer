import './globals.css';

export const metadata = {
  title: 'PC名変更｜実習室向けWindowsツール',
  description: 'QRコードまたは手入力で、実習室のWindows PC名をすばやく変更できる軽量ツールです。',
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return <html lang="ja"><body>{children}</body></html>;
}
