// Prism Dashboard Shell - Tailwind + React
    const PrismDashboard = () => {
      return (
        <div className="min-h-screen bg-[#050505] text-slate-200 font-mono selection:bg-violet-500/30">
          {/* Glassmorphic Sidebar */}
          <aside className="fixed left-0 top-0 h-full w-64 border-r border-white/5 bg-white/5 backdrop-blur-xl p-6 flex flex-col justify-between">
            <div>
              <div className="flex items-center gap-3 mb-10">
                <div className="w-8 h-8 bg-gradient-to-br from-violet-500 to-cyan-400 rounded-sm rotate-45 animate-pulse" />
                <h1 className="text-xl font-bold tracking-tighter text-white">PRISM</h1>
              </div>
              
              <nav className="space-y-4">
                <NavItem icon="💠" label="Identity" active />
                <NavItem icon="🛡️" label="Shielded State" />
                <NavItem icon="⚡" label="Ika Nodes" />
                <NavItem icon="🔗" label="Refraction" />
              </nav>
            </div>
    
            <div className="p-4 rounded-lg bg-indigo-500/10 border border-indigo-500/20">
              <p className="text-[10px] text-indigo-300 uppercase tracking-widest mb-1">Status</p>
              <div className="flex items-center gap-2">
                <span className="w-2 h-2 bg-green-500 rounded-full animate-ping" />
                <span className="text-xs">Sovereign Link Active</span>
              </div>
            </div>
          </aside>
    
          {/* Main Command Center */}
          <main className="ml-64 p-8">
            <header className="flex justify-between items-end mb-12">
              <div>
                <h2 className="text-4xl font-black text-transparent bg-clip-text bg-gradient-to-r from-white to-slate-500">
                  COMMAND_CENTER
                </h2>
                <p className="text-slate-500 mt-2">Managing private capital & cross-chain identity</p>
              </div>
              <div className="text-right">
                <p className="text-xs text-cyan-400 uppercase tracking-widest">Network: Sui Mainnet</p>
                <p className="text-xs text-slate-600">v0.1.0-alpha</p>
              </div>
            </header>
    
            {/* The "Heavy Metal" Grid */}
            <div className="grid grid-cols-12 gap-6">
              {/* Identity Prism Tile */}
              <div className="col-span-8 p-1 rounded-xl bg-gradient-to-br from-white/10 to-transparent">
                <div className="h-96 rounded-lg bg-[#1A1A1B] border border-white/5 p-8 relative overflow-hidden">
                   {/* This is where the 3D Three.js Prism will live */}
                   <div className="absolute inset-0 opacity-20 pointer-events-none bg-[url('https://www.transparenttextures.com/patterns/brushed-alum.png')]" />
                   <h3 className="text-lg font-bold text-violet-400 mb-6">Identity Refraction</h3>
                   <div className="space-y-4">
                     <IdentityRow network="BTC" addr="bc1q...z9p" status="Encrypted" />
                     <IdentityRow network="ETH" addr="0x71...4e2" status="Shielded" />
                     <IdentityRow network="SUI" addr="0x5a...c12" status="Active" />
                   </div>
                </div>
              </div>
    
              {/* Side Panels */}
              <div className="col-span-4 space-y-6">
                <MetricCard label="Private Assets" value="$1.24M" delta="+4.2%" />
                <MetricCard label="Active Traps" value="12" delta="Secure" />
              </div>
            </div>
          </main>
        </div>
      );
    };
