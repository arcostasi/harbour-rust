import { useState } from 'react';
import {
  Anchor,
  BookOpen,
  Download,
  ExternalLink,
  Github,
  Globe,
  Languages,
  ShieldCheck,
  TerminalSquare,
} from 'lucide-react';
import { motion, useReducedMotion } from 'motion/react';
import socialPreview from '../../assets/harbour-rust-social-preview.png';

type Language = 'en' | 'pt-BR';

const GITHUB_URL = 'https://github.com/arcostasi/harbour-rust';
const RELEASES_URL = `${GITHUB_URL}/releases`;
const CURRENT_RELEASE_URL = `${RELEASES_URL}/tag/0.4.0-alpha`;
const DISCUSSIONS_URL = `${GITHUB_URL}/discussions`;
const ISSUES_URL = `${GITHUB_URL}/issues`;

const ASSETS = {
  linux: `${RELEASES_URL}/download/0.4.0-alpha/harbour-rust-cli-0.4.0-alpha-linux-x86_64.zip`,
  macos: `${RELEASES_URL}/download/0.4.0-alpha/harbour-rust-cli-0.4.0-alpha-macos-aarch64.zip`,
  windows: `${RELEASES_URL}/download/0.4.0-alpha/harbour-rust-cli-0.4.0-alpha-windows-x86_64.zip`,
  sha256: `${RELEASES_URL}/download/0.4.0-alpha/SHA256SUMS.txt`,
};

const DOCS = {
  en: {
    readme: `${GITHUB_URL}/blob/main/README.md`,
    roadmap: `${GITHUB_URL}/blob/main/ROADMAP.md`,
    compatibility: `${GITHUB_URL}/blob/main/COMPATIBILITY.md`,
    contributing: `${GITHUB_URL}/blob/main/CONTRIBUTING.md`,
  },
  'pt-BR': {
    readme: `${GITHUB_URL}/blob/main/README.pt-BR.md`,
    roadmap: `${GITHUB_URL}/blob/main/ROADMAP.pt-BR.md`,
    compatibility: `${GITHUB_URL}/blob/main/COMPATIBILITY.pt-BR.md`,
    contributing: `${GITHUB_URL}/blob/main/CONTRIBUTING.pt-BR.md`,
  },
};

const translations = {
  en: {
    languageLabel: 'PT-BR',
    hero: {
      statement:
        'A modern Rust compiler project for CA-Clipper and Harbour compatibility.',
      summary:
        'Harbour Rust is a practical alpha focused on observable compatibility, a readable C backend, and a public roadmap for gradual xBase modernization.',
      ctaPrimary: 'View release',
      ctaSecondary: 'Browse source',
      ctaDocs: 'Read docs',
      quickLinks: 'English | Português do Brasil',
    },
    strips: [
      { title: 'Compatibility-first', detail: 'CA-Clipper / Harbour' },
      { title: 'C backend', detail: 'practical alpha pipeline' },
      { title: 'Open source', detail: 'community-led' },
    ],
    sections: {
      statusTitle: 'Current baseline',
      statusIntro:
        'The project has completed phases 0 through 12 of the initial roadmap and is published as 0.4.0-alpha.',
      statusItems: [
        'Lexer, parser, HIR, semantic analysis, runtime, IR, and C code generation are implemented.',
        'Procedural compatibility, arrays, STATIC, memvars, codeblocks, and selected preprocessor features are available.',
        'DBF/RDD support is present as an initial usable foundation.',
        'CLI, regression harnesses, benchmark smoke, fuzz scaffolding, and release automation are in place.',
      ],
      releaseTitle: 'Release assets',
      releaseIntro:
        'The current pre-release ships cross-platform CLI assets, benchmark output, and checksums.',
      getStartedTitle: 'Get started',
      getStartedItems: [
        {
          title: 'Download the latest pre-release',
          body: 'Use the published release assets for Linux, macOS, or Windows.',
        },
        {
          title: 'Build from source with Cargo',
          body: 'The repository remains source-first and easy to validate locally.',
        },
      ],
      docsTitle: 'Documentation',
      docsIntro:
        'Use the repository docs as the canonical source for roadmap, compatibility, and contribution policy.',
      communityTitle: 'Community',
      communityIntro:
        'Use Issues for scoped bugs and compatibility work, and Discussions for broader design and onboarding conversations.',
      footer:
        'Independent, open-source, and non-commercial as a project initiative. Released under Apache-2.0.',
    },
    labels: {
      latestRelease: 'Latest pre-release',
      allReleases: 'All releases',
      docs: 'Documentation',
      roadmap: 'Roadmap',
      compatibility: 'Compatibility',
      contributing: 'Contributing',
      issues: 'Issues',
      discussions: 'Discussions',
      sourceBuild: 'Build command',
      releaseBadge: '0.4.0-alpha',
      sourceCode: 'Source code',
      assets: {
        linux: 'Linux x86_64',
        macos: 'macOS aarch64',
        windows: 'Windows x86_64',
        sha256: 'SHA256SUMS.txt',
      },
    },
  },
  'pt-BR': {
    languageLabel: 'EN',
    hero: {
      statement:
        'Um projeto de compilador em Rust para compatibilidade com CA-Clipper e Harbour.',
      summary:
        'Harbour Rust é um alpha pragmático, focado em compatibilidade observável, backend C legível e um roadmap público para modernização gradual de sistemas xBase.',
      ctaPrimary: 'Ver release',
      ctaSecondary: 'Ver código',
      ctaDocs: 'Ler docs',
      quickLinks: 'English | Português do Brasil',
    },
    strips: [
      { title: 'Compatibilidade primeiro', detail: 'CA-Clipper / Harbour' },
      { title: 'Backend em C', detail: 'pipeline alpha pragmático' },
      { title: 'Open source', detail: 'liderado pela comunidade' },
    ],
    sections: {
      statusTitle: 'Baseline atual',
      statusIntro:
        'O projeto concluiu as fases 0 a 12 do roadmap inicial e está publicado como 0.4.0-alpha.',
      statusItems: [
        'Lexer, parser, HIR, análise semântica, runtime, IR e geração de código C estão implementados.',
        'Compatibilidade procedural, arrays, STATIC, memvars, codeblocks e parte do pré-processador já estão disponíveis.',
        'O suporte a DBF/RDD já existe como base inicial utilizável.',
        'CLI, harnesses de regressão, benchmark smoke, scaffold de fuzzing e automação de release já estão configurados.',
      ],
      releaseTitle: 'Assets da release',
      releaseIntro:
        'A pre-release atual publica binários da CLI para múltiplas plataformas, benchmark e checksums.',
      getStartedTitle: 'Como começar',
      getStartedItems: [
        {
          title: 'Baixe a pre-release mais recente',
          body: 'Use os assets publicados para Linux, macOS ou Windows.',
        },
        {
          title: 'Compile com Cargo a partir do código-fonte',
          body: 'O repositório continua sendo source-first e fácil de validar localmente.',
        },
      ],
      docsTitle: 'Documentação',
      docsIntro:
        'Use a documentação do repositório como fonte canônica para roadmap, compatibilidade e política de contribuição.',
      communityTitle: 'Comunidade',
      communityIntro:
        'Use Issues para bugs com escopo claro e compatibilidade, e Discussions para design, dúvidas e onboarding.',
      footer:
        'Projeto independente, open source e sem fins lucrativos como iniciativa de comunidade. Distribuído sob Apache-2.0.',
    },
    labels: {
      latestRelease: 'Pre-release atual',
      allReleases: 'Todas as releases',
      docs: 'Documentação',
      roadmap: 'Roadmap',
      compatibility: 'Compatibilidade',
      contributing: 'Contribuição',
      issues: 'Issues',
      discussions: 'Discussions',
      sourceBuild: 'Comando de build',
      releaseBadge: '0.4.0-alpha',
      sourceCode: 'Código-fonte',
      assets: {
        linux: 'Linux x86_64',
        macos: 'macOS aarch64',
        windows: 'Windows x86_64',
        sha256: 'SHA256SUMS.txt',
      },
    },
  },
} as const;

function LinkTile({
  href,
  title,
  body,
}: {
  href: string;
  title: string;
  body: string;
}) {
  return (
    <a
      href={href}
      target="_blank"
      rel="noreferrer"
      className="group block border-t border-white/12 py-5 transition-colors hover:border-[color:var(--accent)]"
    >
      <div className="flex items-start justify-between gap-4">
        <div>
          <h3 className="text-lg font-semibold text-white">{title}</h3>
          <p className="mt-2 max-w-xl text-sm leading-7 text-slate-400">{body}</p>
        </div>
        <ExternalLink className="mt-1 h-4 w-4 shrink-0 text-slate-500 transition-colors group-hover:text-[color:var(--accent)]" />
      </div>
    </a>
  );
}

export default function App() {
  const [language, setLanguage] = useState<Language>('en');
  const reduceMotion = useReducedMotion();
  const t = translations[language];
  const docs = DOCS[language];

  return (
    <div className="site-shell">
      <header className="topbar">
        <div className="topbar__inner">
          <a className="brand" href={GITHUB_URL} target="_blank" rel="noreferrer">
            <Anchor className="h-5 w-5 text-[color:var(--accent)]" />
            <span>harbour-rust</span>
          </a>

          <div className="topbar__actions">
            <a href={CURRENT_RELEASE_URL} target="_blank" rel="noreferrer">
              {t.labels.latestRelease}
            </a>
            <a href={RELEASES_URL} target="_blank" rel="noreferrer">
              {t.labels.allReleases}
            </a>
            <a href={docs.readme} target="_blank" rel="noreferrer">
              {t.labels.docs}
            </a>
            <button
              type="button"
              className="language-toggle"
              onClick={() => setLanguage((current) => (current === 'en' ? 'pt-BR' : 'en'))}
            >
              <Languages className="h-4 w-4" />
              {t.languageLabel}
            </button>
          </div>
        </div>
      </header>

      <main>
        <section className="hero">
          <div className="hero__media">
            <motion.img
              src={socialPreview}
              alt="Harbour Rust social preview"
              initial={reduceMotion ? false : { opacity: 0, scale: 1.02, y: 18 }}
              animate={reduceMotion ? undefined : { opacity: 1, scale: 1, y: 0 }}
              transition={{ duration: 0.8, ease: 'easeOut' }}
            />
          </div>

          <div className="hero__content">
            <motion.div
              initial={reduceMotion ? false : { opacity: 0, y: 18 }}
              animate={reduceMotion ? undefined : { opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.1, ease: 'easeOut' }}
            >
              <p className="eyebrow">{t.labels.releaseBadge}</p>
              <h1>{t.hero.statement}</h1>
              <p className="hero__summary">{t.hero.summary}</p>

              <div className="hero__actions">
                <a className="button button--primary" href={CURRENT_RELEASE_URL} target="_blank" rel="noreferrer">
                  <Download className="h-4 w-4" />
                  {t.hero.ctaPrimary}
                </a>
                <a className="button button--ghost" href={GITHUB_URL} target="_blank" rel="noreferrer">
                  <Github className="h-4 w-4" />
                  {t.hero.ctaSecondary}
                </a>
                <a className="button button--ghost" href={docs.readme} target="_blank" rel="noreferrer">
                  <BookOpen className="h-4 w-4" />
                  {t.hero.ctaDocs}
                </a>
              </div>
            </motion.div>
          </div>
        </section>

        <section className="strip">
          <div className="strip__inner">
            {t.strips.map((item, index) => (
              <motion.div
                key={item.title}
                className="strip__item"
                initial={reduceMotion ? false : { opacity: 0, y: 20 }}
                whileInView={reduceMotion ? undefined : { opacity: 1, y: 0 }}
                viewport={{ once: true, amount: 0.4 }}
                transition={{ duration: 0.45, delay: reduceMotion ? 0 : index * 0.08 }}
              >
                <h2>{item.title}</h2>
                <p>{item.detail}</p>
              </motion.div>
            ))}
          </div>
        </section>

        <section className="content-section">
          <div className="content-grid">
            <div className="content-copy">
              <p className="eyebrow">Status</p>
              <h2>{t.sections.statusTitle}</h2>
              <p className="section-intro">{t.sections.statusIntro}</p>
            </div>
            <ul className="status-list">
              {t.sections.statusItems.map((item) => (
                <li key={item}>{item}</li>
              ))}
            </ul>
          </div>
        </section>

        <section className="content-section content-section--alt">
          <div className="content-grid">
            <div className="content-copy">
              <p className="eyebrow">Release</p>
              <h2>{t.sections.releaseTitle}</h2>
              <p className="section-intro">{t.sections.releaseIntro}</p>
            </div>
            <div>
              <LinkTile href={ASSETS.linux} title={t.labels.assets.linux} body="CLI asset for Linux x86_64." />
              <LinkTile href={ASSETS.macos} title={t.labels.assets.macos} body="CLI asset for macOS Apple Silicon." />
              <LinkTile href={ASSETS.windows} title={t.labels.assets.windows} body="CLI asset for Windows x86_64." />
              <LinkTile href={ASSETS.sha256} title={t.labels.assets.sha256} body="Checksums for the published release assets." />
            </div>
          </div>
        </section>

        <section className="content-section">
          <div className="content-grid">
            <div className="content-copy">
              <p className="eyebrow">Start</p>
              <h2>{t.sections.getStartedTitle}</h2>
              <p className="section-intro">{t.sections.getStartedItems[0].body}</p>
            </div>
            <div className="stacked-pane">
              <div className="stacked-pane__block">
                <h3>{t.sections.getStartedItems[0].title}</h3>
                <p>{t.sections.getStartedItems[0].body}</p>
                <a href={CURRENT_RELEASE_URL} target="_blank" rel="noreferrer">
                  {t.labels.latestRelease}
                </a>
              </div>
              <div className="stacked-pane__block">
                <h3>{t.sections.getStartedItems[1].title}</h3>
                <p>{t.sections.getStartedItems[1].body}</p>
                <pre>
                  <code>{`git clone ${GITHUB_URL}
cd harbour-rust
cargo build --release -p harbour-rust-cli`}</code>
                </pre>
              </div>
            </div>
          </div>
        </section>

        <section className="content-section content-section--alt">
          <div className="content-grid">
            <div className="content-copy">
              <p className="eyebrow">Docs</p>
              <h2>{t.sections.docsTitle}</h2>
              <p className="section-intro">{t.sections.docsIntro}</p>
            </div>
            <div>
              <LinkTile href={docs.readme} title={t.labels.docs} body="Repository overview, setup, release status, and quick links." />
              <LinkTile href={docs.roadmap} title={t.labels.roadmap} body="Phases, release milestones, and near-term planning." />
              <LinkTile href={docs.compatibility} title={t.labels.compatibility} body="Current compatibility baseline and documented limitations." />
              <LinkTile href={docs.contributing} title={t.labels.contributing} body="Contribution rules, review expectations, and workflow guidance." />
            </div>
          </div>
        </section>

        <section className="content-section">
          <div className="content-grid">
            <div className="content-copy">
              <p className="eyebrow">Community</p>
              <h2>{t.sections.communityTitle}</h2>
              <p className="section-intro">{t.sections.communityIntro}</p>
            </div>
            <div className="community-links">
              <a href={ISSUES_URL} target="_blank" rel="noreferrer">
                <ShieldCheck className="h-5 w-5" />
                <div>
                  <strong>{t.labels.issues}</strong>
                  <span>Scoped bugs, compatibility reports, and concrete work items.</span>
                </div>
              </a>
              <a href={DISCUSSIONS_URL} target="_blank" rel="noreferrer">
                <Globe className="h-5 w-5" />
                <div>
                  <strong>{t.labels.discussions}</strong>
                  <span>Questions, onboarding, and broader technical conversations.</span>
                </div>
              </a>
            </div>
          </div>
        </section>
      </main>

      <footer className="site-footer">
        <div className="site-footer__inner">
          <div className="brand">
            <Anchor className="h-5 w-5 text-[color:var(--accent)]" />
            <span>harbour-rust</span>
          </div>
          <p>{t.sections.footer}</p>
          <a href={GITHUB_URL} target="_blank" rel="noreferrer">
            {t.labels.sourceCode}
          </a>
        </div>
      </footer>
    </div>
  );
}
