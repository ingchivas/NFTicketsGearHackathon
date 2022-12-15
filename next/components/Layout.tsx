import dynamic from "next/dynamic";

import NavBar from "./NavBar";

interface LayoutProps {
  children: React.ReactNode
}

const Account = dynamic(() => import("./Account/Account"), {
  ssr: false,
});

const Layout = ({ children }: LayoutProps) => {
  return (
    <main className="min-h-screen w-full bg-slate-800 text-white">
      <div className="px-8 py-4 flex justify-between items-center bg-slate-900">
        <h1 className="text-lg font-bold">NFTickets</h1>
        <NavBar />
        <Account />
      </div>
      <div>
        {children}
      </div>
    </main>
  );
}

export default Layout;
