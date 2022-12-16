import nft from "../../assets/images/nfts/QmQLpvBq63Jqd9cdmKNBgFzLmMBKiqeA9mWduk61zX3G9T.png";

function SeeNFT() {
  return (
    <div className="h-screen w-screen flex justify-center items-center">
      <div className="w-1/3 p-4 flex flex-col rounded shadow bg-slate-900">
        <h1 className="font-bold text-indigo-600 text-lg">Thanks for purchasing a ticket.</h1>
        <h2>Here is your newly generated NFT</h2>
        <img
          className="py-4"
          src={nft}
          alt=""
        />
        <div>
          Link:{" "}
          <a
            className="underline"
            width={500}
            href="https://gateway.pinata.cloud/ipfs/QmQLpvBq63Jqd9cdmKNBgFzLmMBKiqeA9mWduk61zX3G9T">https://gateway.pinata.cloud/ipfs/QmQLpvBq63Jqd9cdmKNBgFzLmMBKiqeA9mWduk61zX3G9T</a>
        </div>
      </div>
    </div>
  );
}

export { SeeNFT };
