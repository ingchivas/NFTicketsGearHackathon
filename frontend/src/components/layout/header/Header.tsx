import { Account } from './account';
import { NavBar } from './navbar';

type Props = {
  isAccountVisible: boolean;
};

function Header({ isAccountVisible }: Props) {
  return (
    <header>
      <div className="px-8 py-4 flex justify-between items-center bg-slate-900">
        <h1 className="text-lg font-bold">NFTickets</h1>
        <div className="flex">
          <NavBar />
          {isAccountVisible && <Account />}
        </div>
      </div>
    </header>
  );
}

export { Header };
