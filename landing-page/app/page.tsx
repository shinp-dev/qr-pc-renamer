const DOWNLOAD_URL = 'https://github.com/shinp-dev/qr-pc-renamer/releases/latest/download/pc-renamer.zip';

export default function Home() {
  return (
    <>
      <header className="site-header">
        <a className="brand" href="#top" aria-label="PC名変更 トップ">PC名変更</a>
        <a className="header-link" href="#how-to">使い方</a>
      </header>
      <main id="top">
        <section className="hero section-wrap">
          <div className="hero-copy">
            <p className="eyebrow">実習室・教室のPC管理に</p>
            <h1>PC名の変更を、<br /><span>もっと手早く。</span></h1>
            <p className="lead">QRコードを読み取るだけで、Windows PCのコンピュータ名を変更できます。読み取り後の修正にも対応した、管理作業向けのシンプルなツールです。</p>
            <div className="hero-actions">
              <a className="button button-dark" href={DOWNLOAD_URL}>最新版ZIPをダウンロード</a>
              <a className="text-link" href="#how-to">使い方を見る <span aria-hidden="true">↓</span></a>
            </div>
          </div>
          <div className="hero-art"><img src="/assets/pc-renamer-icon-lp.png" alt="PC名変更：QRコードを使ってPC名を変更するアイコン" /></div>
        </section>

        <section className="section section-muted" aria-labelledby="features-title">
          <div className="section-wrap">
            <p className="eyebrow">できること</p>
            <h2 id="features-title">実習室の入れ替え作業を、シンプルに。</h2>
            <div className="feature-grid">
              <article className="feature-card"><span className="feature-number">01</span><h3>QRコードを一括作成</h3><p>PC名一覧から、PCごとのQR画像をまとめて作成できます。</p></article>
              <article className="feature-card"><span className="feature-number">02</span><h3>読み取り後の修正</h3><p>QRコードをうまく読み取れない場合や、読み取ったPC名を修正したい場合は、手入力に切り替えられます。</p></article>
              <article className="feature-card"><span className="feature-number">03</span><h3>変更履歴を保存</h3><p>変更前と変更後のPC名を、日時付きのログファイルに記録します。</p></article>
            </div>
          </div>
        </section>

        <section className="section" aria-labelledby="qr-prep-title">
          <div className="section-wrap narrow-wrap">
            <p className="eyebrow">事前準備</p><h2 id="qr-prep-title">先にQR画像をまとめて用意</h2>
            <ol className="steps">
              <li><span className="step-number">1</span><div><h3>QR一括生成ツールを実行</h3><p>ZIPに含まれる <code>qr-batch-generator.exe</code> を実行し、作成された <code>input.txt</code> にPC名を1行ずつ入力して、もう一度実行します。</p></div></li>
              <li><span className="step-number">2</span><div><h3>作成した画像を確認</h3><p>指定フォルダに、PCごとのQR画像が連番で作成されます。</p></div></li>
              <li><span className="step-number">3</span><div><h3>Google Driveへ入れる</h3><p>作成したQR画像をGoogle Driveの作業用フォルダにアップロードします。</p></div></li>
            </ol>
            <p className="prep-note">PC名変更作業では、スマホでQR画像を1枚ずつ表示し、対象PCのカメラに画面をかざして読み取ります。</p>
          </div>
        </section>

        <section className="section" id="how-to" aria-labelledby="how-to-title">
          <div className="section-wrap narrow-wrap">
            <p className="eyebrow">PCごとの作業</p><h2 id="how-to-title">スマホのQR画像を使って変更</h2>
            <ol className="steps">
              <li><span className="step-number">1</span><div><h3>ZIPをダウンロードして展開</h3><p>GitHub ReleasesからZIPをダウンロードし、任意の場所へ展開します。PC名変更、起動用BAT、QR一括生成の3ファイルが入っています。</p></div></li>
              <li><span className="step-number">2</span><div><h3>run_renamer_qr.batを起動</h3><p>展開したフォルダのBATファイルを起動し、UAC画面で「はい」を押します。</p></div></li>
              <li><span className="step-number">3</span><div><h3>Google Driveで次のQR画像を表示</h3><p>スマホで作業対象のQR画像を1枚表示します。</p></div></li>
              <li><span className="step-number">4</span><div><h3>スマホをPCのカメラにかざす</h3><p>表示したQR画像を対象PCのカメラに読み取らせます。うまく読めない場合は手入力に切り替えられます。</p></div></li>
              <li><span className="step-number">5</span><div><h3>内容を確認して変更</h3><p>現在のPC名と新しいPC名を確認して、変更を実行します。</p></div></li>
              <li><span className="step-number">6</span><div><h3>次のQR画像へ進む</h3><p>変更後、Google Driveで次の画像を表示して、同じ作業をPCごとに繰り返します。</p></div></li>
              <li><span className="step-number">7</span><div><h3>最後にPCを再起動</h3><p>変更を完全に反映するため、作業後にPCを再起動します。</p></div></li>
            </ol>
          </div>
        </section>

        <section className="permission section-muted" aria-labelledby="permission-title"><div className="section-wrap permission-inner"><div><p className="eyebrow">権限について</p><h2 id="permission-title">管理者アカウントなら、<br />「はい」を押すだけ。</h2></div><div className="permission-text"><p><strong>管理者アカウントでログイン中：</strong><br />BATファイルを起動し、UAC画面で「はい」を押します。</p><p><strong>標準ユーザーでログイン中：</strong><br />管理者アカウントのユーザー名・パスワードが必要です。</p><p><strong>注意：</strong><br />Active Directoryドメイン参加PCでは使用できません。</p></div></div></section>

        <section className="download section-wrap" id="download" aria-labelledby="download-title"><div className="download-box"><img src="/assets/pc-renamer-icon-lp.png" alt="" className="download-icon" /><div><p className="eyebrow">PC名変更</p><h2 id="download-title">実習室のPC管理を、少しラクに。</h2><p>最新版のZIPをダウンロードして、展開後にBATファイルを実行してください。</p></div><a className="button button-light" href={DOWNLOAD_URL}>ZIPをダウンロード</a></div></section>
      </main>
      <footer className="site-footer"><div className="section-wrap footer-inner"><span>PC名変更</span><span>Windows向けツール</span></div></footer>
    </>
  );
}
