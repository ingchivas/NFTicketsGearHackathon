import { useAccount } from '@gear-js/react-hooks';

import happyMusic from "../../assets/images/happyMusic.svg";
import mello from "../../assets/images/mello.svg";

function Home() {
  return (
    <>
      <div className="h-screen flex">
        <div className="basis-1/2 flex justify-center items-center">
          <div className="w-2/3">
            <img
              src={happyMusic}
              alt=""
            />
          </div>
        </div>
        <div className="basis-1/2 flex justify-center items-center">
          <div className="flex flex-col w-2/3">
            <h1 className="text-white text-4xl font-bold">Secure concert tickets</h1>
            <p>Introducing, the best way of selling concert tickets. Based on the Gear blockchain and NFT technology. </p>

            <div className="py-4 flex justify-center items-center">
              <button
                type="button"
                className="flex-0 p-2 font-bold bg-indigo-600 border-2 border-indigo-600 rounded hover:bg-transparent transition-all"
              >
                Get started
              </button>
            </div>
          </div>
        </div>
      </div>
      <div className="relative h-screen bg-indigo-600">
        <div className="absolute top-0 left-0 w-full overflow-hidden">
          <svg className="relative block w-[calc(106%+1.3px)] h-[144px]" data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 120" preserveAspectRatio="none">
            <path d="M321.39,56.44c58-10.79,114.16-30.13,172-41.86,82.39-16.72,168.19-17.73,250.45-.39C823.78,31,906.67,72,985.66,92.83c70.05,18.48,146.53,26.09,214.34,3V0H0V27.35A600.21,600.21,0,0,0,321.39,56.44Z" className="fill-slate-800" />
          </svg>
        </div>
        <div className="absolute top-0 h-full left-0 w-full flex text-white">
          <div className="basis-1/2 flex justify-center items-center">
            <div className="flex flex-col w-2/3">
              <h1 className="text-white text-4xl font-bold">Super easy to use</h1>
              <ol className="py-2 list-decimal">
                <li>Install the <a href="https://polkadot.js.org/extension/">Polkabot extension</a>.</li>
                <li>Create or add a wallet</li>
                <li>Click the Sign In button at the top of this page</li>
                <li>Start purchasing tickets to your favorite concert.</li>
              </ol>
            </div>
          </div>
          <div className="basis-1/2 flex justify-center items-center">
            <div className="w-2/3">
              <img
                src={mello}
                alt=""
              />
            </div>
          </div>
        </div>
      </div>
    </>
  );
}

export { Home };
