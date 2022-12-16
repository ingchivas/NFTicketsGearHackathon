import { ChangeEvent, useState } from "react";
import { useAccount } from "@gear-js/react-hooks";

interface PopOutProps {
  id: number;
  name: string;
  imageUrl: string;
  handlePopOutClose: () => void;
}

function PopOut({ id, name, imageUrl, handlePopOutClose }: PopOutProps) {
  const { account } = useAccount();
  const [quantity, setQuantity] = useState(0);
  const [date, setDate] = useState("15/12/2022");

  const handleMinusQuantity = () => {
    setQuantity((prevState) => {
      if (prevState === 0) return prevState;
      return prevState - 1;
    });
  }

  const handlePlusQuantity = () => {
    setQuantity((prevState) => {
      if (prevState === 100) return prevState;
      return prevState + 1;
    });
  }

  const handleDateChange = (e: ChangeEvent<HTMLSelectElement>) => {
    const { value } = e.target;
    setDate(value);
  }

  const handleBuy = async () => {
    console.log("Account:", account);
    console.log("Quantity", quantity, "Date:", date);

    try {
      // Send request to backend
      const res = await fetch("http://localhost:3005/sendMessage/buyTickets", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          quantity,
          metadata: null
        })
      });

      const json = await res.json();

      console.log("Result from API", json);
      console.log(json);

      // Redirect to the sucessful page
      if (json.success) {
        console.log("Everything ok")
        window.location.replace(`/seeNft?wallet=${account?.address}`);
        return;
      };

      throw new Error(`Something went wrong: `, json);
    } catch (error) {
      console.error(error);
    }
  }

  return (
    <div className="absolute top-0 left-0 h-screen w-screen bg-indigo-400/25 flex justify-center items-center">
      <div className="p-4 bg-slate-900 rounded">
        <div className="flex justify-between items-center">
          <h2 className="font-bold text-indigo-400">You are buying tickets for</h2>
          <button
            type="button"
            className="flex-0 px-1 font-bold bg-red-600 border-2 border-red-600 rounded hover:bg-transparent transition-all"
            onClick={handlePopOutClose}
          >
            X
          </button>
        </div>
        <h3 className="pb-4 font-bold text-xl">{name}</h3>
        <img src={imageUrl} alt="" width={500} />
        <div className="py-4 flex">
          <div className="basis-1/2 px-2">
            <h4>How many tickets you want to buy?</h4>
            <div className="py-2 font-bold text-l">
              <button
                className="px-2 bg-indigo-600 rounded-tl rounded-bl"
                type="button"
                onClick={handleMinusQuantity}>-</button>
              <span className="px-2 bg-white text-black g">{quantity}</span>
              <button
                className="px-2 bg-indigo-600 rounded-tr rounded-br"
                type="button"
                onClick={handlePlusQuantity}>+</button>
            </div>
          </div>
          <div className="basis-1/2 px-2">
            <h4>Select the date of the concert</h4>
            <select className="px-2 bg-black" onChange={handleDateChange}>
              <option>15/12/2022</option>
              <option>16/12/2022</option>
              <option>17/12/2022</option>
              <option>18/12/2022</option>
            </select>
          </div>
        </div>

        <div className="flex justify-end items-center">
          <button
            type="button"
            className="flex-0 p-2 font-bold bg-indigo-600 border-2 border-indigo-600 rounded hover:bg-transparent transition-all"
            onClick={handleBuy}
          >
            Buy
          </button>
        </div>
      </div>
    </div>
  );
}

export { PopOut };
