import { useLayoutEffect, useRef, useState } from 'react';
import gsap from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';
import {
  Anchor,
  BookOpen,
  Download,
  ExternalLink,
  Globe,
  Languages,
  ShieldCheck,
  TerminalSquare,
} from 'lucide-react';
import socialPreview from '../../assets/harbour-rust-social-preview.png';

type Language = 'en' | 'pt-BR';

gsap.registerPlugin(ScrollTrigger);

const GITHUB_URL = 'https://github.com/arcostasi/harbour-rust';
const RELEASES_URL = `${GITHUB_URL}/releases`;
const CURRENT_RELEASE = '0.5.0-alpha';
const CURRENT_RELEASE_URL = `${RELEASES_URL}/tag/${CURRENT_RELEASE}`;
const DISCUSSIONS_URL = `${GITHUB_URL}/discussions`;
const ISSUES_URL = `${GITHUB_URL}/issues`;

const ASSETS = {
  linux: `${RELEASES_URL}/download/${CURRENT_RELEASE}/harbour-rust-cli-${CURRENT_RELEASE}-linux-x86_64.zip`,
  macos: `${RELEASES_URL}/download/${CURRENT_RELEASE}/harbour-rust-cli-${CURRENT_RELEASE}-macos-aarch64.zip`,
  windows: `${RELEASES_URL}/download/${CURRENT_RELEASE}/harbour-rust-cli-${CURRENT_RELEASE}-windows-x86_64.zip`,
  sha256: `${RELEASES_URL}/download/${CURRENT_RELEASE}/SHA256SUMS.txt`,
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
        'The project has completed phases 0 through 14 and ships the first phase 15 compatibility slice as 0.5.0-alpha.',
      statusItems: [
        'Lexer, parser, HIR, semantic analysis, runtime, IR, and C code generation are implemented.',
        'Procedural compatibility, arrays, STATIC, memvars, codeblocks, and selected advanced preprocessor features are available.',
        'DBF/RDD support is present as an initial usable foundation.',
        'CLI, regression harnesses, benchmark smoke, fuzz scaffolding, release automation, and three-platform validation are in place.',
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
      releaseBadge: CURRENT_RELEASE,
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
        'O projeto concluiu as fases 0 a 14 e publica o primeiro slice de compatibilidade da fase 15 como 0.5.0-alpha.',
      statusItems: [
        'Lexer, parser, HIR, análise semântica, runtime, IR e geração de código C estão implementados.',
        'Compatibilidade procedural, arrays, STATIC, memvars, codeblocks e recursos avançados selecionados do pré-processador já estão disponíveis.',
        'O suporte a DBF/RDD já existe como base inicial utilizável.',
        'CLI, harnesses de regressão, benchmark smoke, scaffold de fuzzing, automação de release e validação em três plataformas já estão configurados.',
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
      releaseBadge: CURRENT_RELEASE,
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
      className="link-tile js-reveal group block border-t border-white/12 py-5 transition-colors hover:border-[color:var(--accent)]"
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
  const rootRef = useRef<HTMLDivElement>(null);
  const [language, setLanguage] = useState<Language>('en');
  const t = translations[language];
  const docs = DOCS[language];

  useLayoutEffect(() => {
    const root = rootRef.current;

    if (!root) {
      return undefined;
    }

    const cleanup: Array<() => void> = [];
    const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

    const context = gsap.context(() => {
      if (reduceMotion) {
        gsap.set('.js-reveal, .strip__item, .hero__media, .hero__content > div', {
          clearProps: 'all',
        });
        return;
      }

      const heroTimeline = gsap.timeline({
        defaults: { ease: 'power3.out' },
      });

      heroTimeline
        .from('.topbar', { y: -18, opacity: 0, duration: 0.55 })
        .from('.hero__media', { y: 30, opacity: 0, scale: 0.975, duration: 0.95 }, '-=0.2')
        .from(
          '.hero__content .eyebrow, .hero__content h1, .hero__summary, .hero__actions .button',
          { y: 18, opacity: 0, duration: 0.62, stagger: 0.075 },
          '-=0.62',
        )
        .from('.strip__item', { y: 18, opacity: 0, duration: 0.5, stagger: 0.09 }, '-=0.25');

      gsap.to('.hero__media img', {
        yPercent: -4,
        scale: 1.025,
        ease: 'none',
        scrollTrigger: {
          trigger: '.hero',
          start: 'top top',
          end: 'bottom top',
          scrub: true,
        },
      });

      gsap.to('.hero__beam', {
        xPercent: 9,
        opacity: 0.74,
        duration: 3.6,
        ease: 'sine.inOut',
        repeat: -1,
        yoyo: true,
      });

      gsap.to('.hero__ring', {
        rotate: 360,
        duration: 28,
        ease: 'none',
        repeat: -1,
      });

      gsap.utils.toArray<HTMLElement>('.js-reveal').forEach((element) => {
        gsap.from(element, {
          y: 26,
          opacity: 0,
          duration: 0.72,
          ease: 'power3.out',
          scrollTrigger: {
            trigger: element,
            start: 'top 84%',
            once: true,
          },
        });
      });

      gsap.utils.toArray<HTMLElement>('.button, .link-tile, .community-links a').forEach((element) => {
        const handleEnter = () => gsap.to(element, { y: -3, duration: 0.22, ease: 'power2.out' });
        const handleLeave = () => gsap.to(element, { y: 0, duration: 0.3, ease: 'power2.out' });

        element.addEventListener('mouseenter', handleEnter);
        element.addEventListener('mouseleave', handleLeave);
        cleanup.push(() => {
          element.removeEventListener('mouseenter', handleEnter);
          element.removeEventListener('mouseleave', handleLeave);
        });
      });
    }, root);

    return () => {
      cleanup.forEach((removeListener) => removeListener());
      context.revert();
    };
  }, []);

  return (
    <div ref={rootRef} className="site-shell">
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
            <div className="hero__frame">
              <span className="hero__beam" aria-hidden="true" />
              <span className="hero__ring" aria-hidden="true" />
              <img src={socialPreview} alt="Harbour Rust social preview" />
            </div>
          </div>

          <div className="hero__content">
            <div>
              <p className="eyebrow">{t.labels.releaseBadge}</p>
              <h1>{t.hero.statement}</h1>
              <p className="hero__summary">{t.hero.summary}</p>

              <div className="hero__actions">
                <a className="button button--primary" href={CURRENT_RELEASE_URL} target="_blank" rel="noreferrer">
                  <Download className="h-4 w-4" />
                  {t.hero.ctaPrimary}
                </a>
                <a className="button button--ghost" href={GITHUB_URL} target="_blank" rel="noreferrer">
                  <TerminalSquare className="h-4 w-4" />
                  {t.hero.ctaSecondary}
                </a>
                <a className="button button--ghost" href={docs.readme} target="_blank" rel="noreferrer">
                  <BookOpen className="h-4 w-4" />
                  {t.hero.ctaDocs}
                </a>
              </div>
            </div>
          </div>
        </section>

        <section className="strip">
          <div className="strip__inner">
            {t.strips.map((item) => (
              <div key={item.title} className="strip__item">
                <h2>{item.title}</h2>
                <p>{item.detail}</p>
              </div>
            ))}
          </div>
        </section>

        <section className="content-section js-reveal">
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

        <section className="content-section content-section--alt js-reveal">
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

        <section className="content-section js-reveal">
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

        <section className="content-section content-section--alt js-reveal">
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

        <section className="content-section js-reveal">
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
