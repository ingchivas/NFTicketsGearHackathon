import { useAccount } from "@gear-js/react-hooks";
import { upload } from "api/functions";

function Buy() {
  const { account } = useAccount();

  const handleClick = function (concertID: number) {
    upload();
  }

  return (
    <div className="p-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4">
      <div className="p-4 rounded bg-slate-900">
        <h2 className="pb-4 font-bold text-xl">Nombre del concierto</h2>
        <img alt="" />
        <div className="pt-4 flex justify-center items-center">
          <button
            type="button"
            className="flex-0 p-2 font-bold bg-indigo-600 border-2 border-indigo-600 rounded hover:bg-transparent transition-all"
            onClick={() => handleClick(1)}
          >
            I want tickets
          </button>
        </div>
      </div>
    </div>
  );
}

export { Buy };
