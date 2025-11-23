import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { TechnicalCard } from "@/components/TechnicalCard";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import { Copy, ArrowRight, BarChart2, Link as LinkIcon, Terminal } from "lucide-react";
import { api } from "@/lib/api";
import { toast } from "sonner";

const ShortenerInterface = () => {
  const [url, setUrl] = useState("");
  const [result, setResult] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [statsId, setStatsId] = useState("");
  const [stats, setStats] = useState<number | null>(null);

  const handleShorten = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!url) return;
    
    setLoading(true);
    try {
      // Matches backend struct: struct CreateRequest { url: String }
      const { data } = await api.post("/api/shorten", { url });
      setResult(data.slug);
      toast.success("Link shortened successfully");
    } catch (err) {
      toast.error("Connection refused. Is the backend running?");
    } finally {
      setLoading(false);
    }
  };

  const handleStats = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!statsId) return;

    setLoading(true);
    try {
      // Extract ID if user pastes full URL, otherwise use as is
      const cleanId = statsId.split("/").pop() || statsId;
      const { data } = await api.get(`/${cleanId}/stats`);
      setStats(data.clicks);
    } catch (err) {
      setStats(null);
      toast.error("404: URL identifier not found");
    } finally {
      setLoading(false);
    }
  };

  const copyToClipboard = () => {
    if (result) {
      navigator.clipboard.writeText(result);
      toast.success("Copied to clipboard");
    }
  };

  return (
    <div className="max-w-xl w-full mx-auto p-4">
      {/* Terminal Header styling matching your Hero component */}
      <div className="mb-6 font-mono text-sm text-muted-foreground flex items-center gap-2">
        <span className="text-foreground">$</span> 
        <span>./shortun --interactive</span>
        <span className="animate-terminal-cursor inline-block w-2 h-4 bg-foreground align-middle ml-1" />
      </div>

      <TechnicalCard className="bg-card/80 backdrop-blur-sm">
        <Tabs defaultValue="shorten" className="w-full">
          <TabsList className="grid w-full grid-cols-2 mb-6">
            <TabsTrigger value="shorten" onClick={() => { setResult(null); setUrl(""); }}>
              <LinkIcon className="w-4 h-4 mr-2" /> Shorten
            </TabsTrigger>
            <TabsTrigger value="stats" onClick={() => { setStats(null); setStatsId(""); }}>
              <BarChart2 className="w-4 h-4 mr-2" /> Stats
            </TabsTrigger>
          </TabsList>

          {/* SHORTEN TAB */}
          <TabsContent value="shorten" className="space-y-6">
            <form onSubmit={handleShorten} className="space-y-4">
              <div className="space-y-2">
                <label className="text-xs font-mono uppercase text-muted-foreground">Target URL</label>
                <div className="flex gap-2">
                  <Input 
                    value={url}
                    onChange={(e) => setUrl(e.target.value)}
                    placeholder="https://..." 
                    className="font-mono bg-background/50"
                    autoFocus
                  />
                  <Button type="submit" disabled={loading}>
                    {loading ? <Terminal className="w-4 h-4 animate-spin" /> : <ArrowRight className="w-4 h-4" />}
                  </Button>
                </div>
              </div>
            </form>

            {result && (
              <div className="mt-6 p-4 border border-border bg-secondary/20 rounded-md animate-in fade-in slide-in-from-bottom-2">
                <div className="text-xs font-mono text-muted-foreground mb-1">Generated Slug:</div>
                <div className="flex items-center justify-between gap-2">
                  <code className="text-primary font-mono text-lg tracking-tight">{result}</code>
                  <Button variant="ghost" size="icon" onClick={copyToClipboard} className="hover:bg-background">
                    <Copy className="w-4 h-4" />
                  </Button>
                </div>
              </div>
            )}
          </TabsContent>

          {/* STATS TAB */}
          <TabsContent value="stats" className="space-y-6">
             <form onSubmit={handleStats} className="space-y-4">
              <div className="space-y-2">
                <label className="text-xs font-mono uppercase text-muted-foreground">Slug ID or Full URL</label>
                <div className="flex gap-2">
                  <Input 
                    value={statsId}
                    onChange={(e) => setStatsId(e.target.value)}
                    placeholder="e.g. abc123xyz" 
                    className="font-mono bg-background/50"
                  />
                  <Button type="submit" disabled={loading}>
                    Check
                  </Button>
                </div>
              </div>
            </form>

            {stats !== null && (
              <div className="mt-6 flex flex-col items-center justify-center p-8 border border-dashed border-border rounded-md bg-secondary/10">
                <div className="text-5xl font-bold font-mono tracking-tighter text-foreground">{stats}</div>
                <div className="text-xs text-muted-foreground uppercase tracking-widest mt-2">Total Clicks</div>
              </div>
            )}
          </TabsContent>
        </Tabs>
      </TechnicalCard>
    </div>
  );
};

export default ShortenerInterface;