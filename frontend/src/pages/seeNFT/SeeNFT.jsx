import nft from "../../assets/images/nfts/QmQLpvBq63Jqd9cdmKNBgFzLmMBKiqeA9mWduk61zX3G9T.png";

function SeeNFT() {
  return (
    <div className="h-screen w-screen flex justify-center items-center">
      <div className="flex rounded bg-slate-900">
        <div className="basis-1/2 text-white">
          <h1 className="font-bold">Thanks for purchasing a ticket.</h1>
          <h2>Here is your newly generated NFT</h2>
        </div>
        <div className="basis-1/2 flex justify-center items-center">
          <img src={nft} alt="" />
          <p>Link: <a href="https://gateway.pinata.cloud/ipfs/QmQLpvBq63Jqd9cdmKNBgFzLmMBKiqeA9mWduk61zX3G9T">https://gateway.pinata.cloud/ipfs/QmQLpvBq63Jqd9cdmKNBgFzLmMBKiqeA9mWduk61zX3G9T</a></p>
        </div>
      </div>
    </div>
  );
}

export { SeeNFT };
