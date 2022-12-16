import { GearKeyring } from "@gear-js/api";
import { useAccount } from "@gear-js/react-hooks";
import { PopOut } from "components/popout";
import { useState } from "react";
import taylor from "../../assets/images/taylor.webp";

function Buy() {
  const { account } = useAccount();
  const [popOut, setPopOutOpen] = useState(0);

  const handleClick = async function (concertID: number) {
    setPopOutOpen(concertID);

    const keyring = await GearKeyring.fromSuri('//Alice');
    const signature = keyring.sign("hola");
    console.log(account);
    console.log(keyring);
  }

  const handlePopOutClose = () => {
    setPopOutOpen(10);
  }

  return (
    <div className="p-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4">
      <div className="p-4 rounded bg-slate-900">
        <h2 className="pb-4 font-bold text-xl">Taylor Swift night (Taylor&#39;s birthday party)</h2>
        <img src={taylor} alt="" width={500} />
        <div className="pt-4 flex justify-center items-center">
          <button
            type="button"
            className="flex-0 p-2 font-bold bg-indigo-600 border-2 border-indigo-600 rounded hover:bg-transparent transition-all"
            onClick={() => handleClick(1)}
          >
            I want tickets
          </button>
        </div>
        {popOut === 1 ? <PopOut id={1} name="Taylor Swift night (Taylor&#39;s birthday party)" imageUrl={taylor} handlePopOutClose={handlePopOutClose} /> : ""}
      </div>
    </div>
  );
}

export { Buy };
