import NavBar from "./NavBar";

interface LayoutProps {
  children: React.ReactNode
}

const Layout = ({ children }: LayoutProps) => {
  return (
    <main className="min-h-screen w-full bg-slate-800 text-white">
      <div className="px-8 py-4 flex justify-between items-center bg-slate-900">
        <h1 className="text-lg font-bold">NFTickets</h1>
        <NavBar />
      </div>
      <div>
        {children}
      </div>
    </main>
  );
}

export default Layout;
