function NavBar() {
  return (
    <div className="px-4 flex items-center space-x-4">
      <div className="px-4 py-2 bg-slate-700 rounded cursor-pointer hover:bg-slate-600 transition-all">
        <a href="/buy">
          Buy tickets
        </a>
      </div>
    </div>
  );
}

export { NavBar };
