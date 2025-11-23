import ShortenerInterface from "./components/ShortenerInterface";
import { Toaster } from "./components/ui/sonner";
import { ThemeProvider } from "next-themes";
import { Github, ArrowLeft} from "lucide-react";
import { ThemeToggle } from "./components/ui/theme-toggle";

const App = () => {
  // Replace with your actual URLs
  const PORTFOLIO_URL = "https://manpreet.tech"; 
  const REPO_URL = "https://github.com/manpreet113/shortun";

  return (
    <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
      <div className="min-h-screen bg-background bg-grid-pattern flex flex-col font-sans text-foreground selection:bg-primary selection:text-primary-foreground">
        
        <header className="fixed top-0 left-0 right-0 z-50 border-b border-border/40 bg-background/80 backdrop-blur-sm">
          <div className="max-w-6xl mx-auto px-6 h-16 flex items-center justify-between">
            {/* Brand / Home Link */}
            <a 
              href={PORTFOLIO_URL}
              className="group flex items-center gap-2 text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
            >
              <ArrowLeft className="w-4 h-4 transition-transform group-hover:-translate-x-1" />
              <span className="font-mono text-foreground opacity-0 transition-opacity group-hover:opacity-100">
                cd
              </span>
              <span>~/portfolio</span>
            </a>

            {/* Right Side Actions */}
            <nav className="flex items-center gap-6">
              <a 
                href={REPO_URL}
                target="_blank"
                rel="noopener noreferrer"
                className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-2 group"
              >
                <Github className="w-4 h-4" />
                <span className="hidden sm:inline">Source</span>
                <span className="absolute -bottom-1 left-0 w-0 h-px bg-foreground transition-all group-hover:w-full" />
              </a>
              <div className="border-l rounded-full border-border pl-4">
                <ThemeToggle />
              </div>
            </nav>
          </div>
        </header>

        {/* Main Content Area */}
        <main className="flex-1 flex items-center justify-center relative pt-16 p-6">
          {/* Background Fade for depth */}
          <div className="absolute inset-0 bg-background/50 pointer-events-none" />
          
          <div className="w-full relative z-10">
            <div className="text-center mb-12 space-y-4">
              <div className="inline-flex items-center justify-center px-3 py-1 rounded-full border border-border bg-secondary/50 text-xs font-mono text-muted-foreground mb-2">
                <span className="mr-2">●</span> v1.0.0 release
              </div>
              
              <h1 className="text-4xl md:text-7xl font-bold tracking-tighter">
                Shortun
              </h1>
              
              <p className="text-muted-foreground font-mono text-sm md:text-base max-w-lg mx-auto leading-relaxed">
                <span className="text-primary">function</span> <span className="text-foreground">shorten</span>(url: <span className="text-primary">String</span>) 
                <span className="text-muted-foreground">{" -> "}</span>
                <span className="text-primary">Result</span>&lt;Slug&gt;
              </p>
            </div>
            
            <ShortenerInterface />
          </div>
        </main>

        {/* Footer */}
        <footer className="py-8 text-center text-sm text-muted-foreground border-t border-border/40">
          <div className="font-mono text-xs mb-2">
            ──────────────────────────────────────────────────
          </div>
          <p>
            Built with <span className="text-foreground font-medium">Rust</span>, <span className="text-foreground font-medium">Axum</span> & <span className="text-foreground font-medium">React</span>
          </p>
        </footer>

        <Toaster />
      </div>
    </ThemeProvider>
  );
};

export default App;