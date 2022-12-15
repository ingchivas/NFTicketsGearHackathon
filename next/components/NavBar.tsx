import Link from "next/link";

const NavBar = () => {
  return (
    <div className="flex items-center space-x-4">
      <div className="px-4 py-2 bg-slate-700 rounded cursor-pointer hover:bg-slate-600 transition-all">
        Hola
      </div>
      <div className="px-4 py-2 bg-slate-700 rounded cursor-pointer hover:bg-slate-600 transition-all">
        Hola
      </div>
    </div>
  );
}

export default NavBar;
